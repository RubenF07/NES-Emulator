pub mod palette;
pub mod frame;

use crate::ppu::NesPPU;
use frame::Frame;

pub fn render(ppu: &NesPPU, frame: &mut Frame){
    let bank = ppu.bkgr_ptrn_addr();

    // Background
    for i in 0..0x03c0{ // just first name table for now
        let tile = ppu.vram[i] as u16;
        let tile_x = i % 32;
        let tile_y = i / 32;
        let tile = &ppu.chr_rom[(bank + tile * 16) as usize..=(bank + tile * 16 + 15) as usize];
        let palette = bg_palette(ppu, tile_x, tile_y);

        for y in 0..=7{
            let mut upper = tile[y];
            let mut lower = tile[y+8];

            for x in (0..=7).rev(){
                let val = (1 & lower) << 1 | (1 & upper);
                // upper >>= 1;
                // lower >>= 1;
                upper = upper >> 1;
                lower = lower >> 1;
                
                let rgb = match val{
                    0 => palette::SYSTEM_PALETTE[ppu.palette_table[0] as usize],
                    1 => palette::SYSTEM_PALETTE[palette[1] as usize],
                    2 => palette::SYSTEM_PALETTE[palette[2] as usize],
                    3 => palette::SYSTEM_PALETTE[palette[3] as usize],
                    _ => panic!("Impossible rgb"),
                };
                
                frame.set_pixel(tile_x*8 + x, tile_y*8 + y, rgb);
            }
        }
    }
    
    // Sprites
    for i in (0..ppu.oam.data.len()).step_by(4).rev(){
        let tile_idx = ppu.oam.data[i + 1] as u16;
        let tile_x = ppu.oam.data[i + 3] as usize;
        let tile_y = ppu.oam.data[i] as usize;
        
        let flip_vert = if ppu.oam.data[i + 2] >> 7 & 1 == 1{
            true
        } else{
            false
        };
        let flip_hor = if ppu.oam.data[i + 2] >> 6 & 1 == 1{
            true
        } else{
            false
        };

        let palette_idx = ppu.oam.data[i + 2] & 0b11;
        let palette = sprite_palette(ppu, palette_idx);

        let bank: u16 = ppu.sprt_ptrn_addr();
        
        let tile = &ppu.chr_rom[(bank + tile_idx * 16) as usize..=(bank + tile_idx * 16 + 15) as usize];
        
        for y in 0..=7{
            let mut upper = tile[y];
            let mut lower = tile[y + 8];
            'ololo: for x in (0..=7).rev(){
                let val = (1 & lower) << 1 | (1 & upper);
                upper = upper >> 1;
                lower = lower >> 1;
                
                let rgb = match val{
                    0 => continue 'ololo, // transparent
                    // 1 => palette::SYSTEM_PALETTE[0x23],
                    // 2 => palette::SYSTEM_PALETTE[0x27],
                    // 3 => palette::SYSTEM_PALETTE[0x30],
                    1 => palette::SYSTEM_PALETTE[palette[1] as usize],
                    2 => palette::SYSTEM_PALETTE[palette[2] as usize],
                    3 => palette::SYSTEM_PALETTE[palette[3] as usize],
                    _ => panic!("Impossible rgb"),
                };
                match (flip_hor, flip_vert){
                    (false, false) => frame.set_pixel(tile_x + x, tile_y + y, rgb),
                    (true, false) => frame.set_pixel(tile_x + 7 -x, tile_y + y, rgb),
                    (false, true) => frame.set_pixel(tile_x + x, tile_y + 7 - y, rgb),
                    (true, true) => frame.set_pixel(tile_x + 7 -x, tile_y + 7 - y, rgb),
                }
            }
            
            
        }

    }
}

fn bg_palette(ppu: &NesPPU, tile_column: usize, tile_row: usize) -> [u8;4]{
    let attr_table_idx = tile_row / 4 * 8 + tile_column / 4;
    let attr_byte = ppu.vram[0x3c0 + attr_table_idx]; // hardcoded first nametable

    let palette_idx = match(tile_column % 4 / 2, tile_row % 4 / 2){
        (0,0) => attr_byte & 0b11,
        (1,0) => (attr_byte >> 2) & 0b11,
        (0,1) => (attr_byte >> 4) & 0b11,
        (1,1) => (attr_byte >> 6) & 0b11,
        (_,_) => unreachable!("impossible"),
    };

    let palette_start: usize = 1 + (palette_idx as usize) * 4;
    return [ppu.palette_table[0], ppu.palette_table[palette_start], ppu.palette_table[palette_start+1], ppu.palette_table[palette_start+2]]
}

fn sprite_palette(ppu: &NesPPU, palette_idx: u8) -> [u8; 4]{
    let start = 0x11 + (palette_idx * 4) as usize;
    return [0, ppu.palette_table[start], ppu.palette_table[start + 1], ppu.palette_table[start + 2]]
}