pub mod registers;

use crate::cartridge::Mirroring;
use crate::ppu::registers::addr::AddrRegister;
use crate::ppu::registers::control::ControlRegister;
use crate::ppu::registers::mask::MaskRegister;
use crate::ppu::registers::oam::OAM;
use crate::ppu::registers::scroll::ScrollRegister;
use crate::ppu::registers::status::StatusRegister;
pub struct NesPPU {
    pub mirroring: Mirroring,
    
    pub chr_rom: Vec<u8>,
    pub palette_table: [u8; 32],
    pub vram: [u8; 2048],
    
    addr: AddrRegister,
    pub ctrl: ControlRegister,
    mask: MaskRegister,
    pub scroll: ScrollRegister,
    status: StatusRegister,
    pub oam: OAM,

    internal_data_buf: u8,

    scanline: u16,
    cycles: usize,
    pub nmi_interrupt: Option<u8>,
}

pub trait PPU {
    fn write_to_ppu_addr(&mut self, val: u8);
    fn write_to_ctrl(&mut self, val: u8);
    fn read_data(&mut self) -> u8;
    fn write_to_data(&mut self, data: u8);
    fn write_to_mask(&mut self, val: u8);
    fn read_status(&mut self) -> u8;
    fn write_to_scroll(&mut self, val: u8);
    fn write_to_oam_addr(&mut self, val: u8);
    fn write_to_oam_data(&mut self, data: u8);
    fn read_oam_data(&self) -> u8;
    fn write_to_oam_dma(&mut self, data: &[u8; 256]);
}

impl NesPPU {
    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self{
        NesPPU{
            chr_rom: chr_rom,
            mirroring: mirroring,

            vram: [0; 2048],
            palette_table: [0; 32],

            addr: AddrRegister::new(),
            ctrl: ControlRegister::new(),
            mask: MaskRegister::new(),
            scroll: ScrollRegister::new(),
            status: StatusRegister::new(),
            oam: OAM::new(),

            internal_data_buf: 0,

            scanline: 0,
            cycles: 0,
            nmi_interrupt: None,
        }
    }

    pub fn new_empty_rom() -> Self{
        NesPPU::new(vec![0; 2048], Mirroring::Horizontal)
    }

    fn increment_vram_addr(&mut self){
        self.addr.increment(self.ctrl.vram_addr_inc());
    }

    // Horizontal:
    //   [ A ] [ a ]
    //   [ B ] [ b ]

    // Vertical:
    //   [ A ] [ B ]
    //   [ a ] [ b ]
    fn mirror_vram_addr(&self, addr: u16) -> u16 {
        let mirrored_vram = addr & 0b10_1111_1111_1111; // 0x3000-0x3eff to 0x2000-0x2eff
        let vram_index = mirrored_vram - 0x2000; // vram vector
        let name_table = vram_index / 0x400;
        match (&self.mirroring, name_table){
            (Mirroring::Vertical, 2) | (Mirroring::Vertical, 3) => vram_index - 0x800,
            (Mirroring::Horizontal, 2) => vram_index - 0x400,
            (Mirroring::Horizontal, 1) => vram_index - 0x400,
            (Mirroring::Horizontal, 3) => vram_index - 0x800,
            _ => vram_index,
        }
    }

    pub fn tick(&mut self, cycles: u8) -> bool {
        self.cycles += cycles as usize;

        if self.cycles >= 341{ // each scanline lasts 341 ppu cycles
            if self.is_sprite_0_hit(self.cycles){
                self.status.set_sprite_zero_hit(true);
            }
            
            self.cycles -= 341;
            self.scanline += 1;

            if self.scanline == 241{
                self.status.set_vblank(true);
                self.status.set_sprite_zero_hit(false);

                if self.ctrl.generate_vblank_nmi(){
                    self.nmi_interrupt = Some(1);
                }
            }

            if self.scanline >= 262{
                self.scanline = 0;
                self.nmi_interrupt = None;

                self.status.set_sprite_zero_hit(false);
                self.status.set_vblank(false);
                return true;
            }
        }
        return false;
    }

    fn is_sprite_0_hit(&self, cycle: usize) -> bool{
        let y = self.oam.data[0] as usize;
        let x = self.oam.data[0] as usize;

        (y == self.scanline as usize) && x <= cycle && self.mask.show_sprites()
    }


    pub fn bkgr_ptrn_addr(&self) -> u16{
        self.ctrl.bkgr_ptrn_addr()
    }
    pub fn sprt_ptrn_addr(&self) -> u16{
        self.ctrl.sprt_ptrn_addr()
    }

}

impl PPU for NesPPU { 
    fn read_data(&mut self) -> u8 {
        let addr = self.addr.get();
        self.increment_vram_addr();

        match addr{
            0..=0x1fff => {
                let res = self.internal_data_buf;
                self.internal_data_buf = self.chr_rom[addr as usize];
                res
            }
            0x2000..=0x2fff => {
                let res = self.internal_data_buf;
                self.internal_data_buf = self.vram[self.mirror_vram_addr(addr) as usize];
                res
            }

            0x3000..=0x3eff => unimplemented!("addresses 0x3000..0x3eff is not usable, {:04x}",addr),
            
            //Addresses $3F10/$3F14/$3F18/$3F1C are mirrors of $3F00/$3F04/$3F08/$3F0C
            0x3f10| 0x3f14 | 0x3f18 | 0x3f1c => {
                let add_mirror = addr - 0x10;
                self.palette_table[(add_mirror - 0x3f00) as usize]
            }
            
            0x3f00..=0x3fff => { // Palette + mirror addresses
                self.palette_table[(addr % 0x20) as usize]
            }
            _ => panic!("unexpected mirror access to {:04x}",addr)
        }
    }

    fn write_to_data(&mut self, data: u8) {
        let addr = self.addr.get();
        match addr{
            0..=0x1fff => println!("Attempted to write to chr rom: {:04x}", addr),
            0x2000..=0x2fff => {
                self.vram[self.mirror_vram_addr(addr) as usize] = data;
            }
            // 0x3000..=0x3eff => unimplemented!("shouldn't use addr: {:04x}",addr),
            0x3000..=0x3eff => {
                self.vram[self.mirror_vram_addr(addr) as usize] = data;
            }

            //Addresses $3F10/$3F14/$3F18/$3F1C are mirrors of $3F00/$3F04/$3F08/$3F0C
            0x3f10| 0x3f14 | 0x3f18 | 0x3f1c => {
                let add_mirror = addr - 0x10;
                self.palette_table[(add_mirror - 0x3f00) as usize] = data;
            }
            0x3f00..=0x3fff => { // Palette + mirror addresses
                self.palette_table[(addr % 0x20) as usize] = data;
            }
            _ => panic!("Unexpected address write: {:04x}",addr),
        }
        self.increment_vram_addr();
    }

    fn write_to_ppu_addr(&mut self, val: u8){
        self.addr.update(val);
    }
    
    fn write_to_ctrl(&mut self, val: u8){
        let pre_nmi_status = self.ctrl.generate_vblank_nmi();
        self.ctrl.update(val);

        if !pre_nmi_status && self.ctrl.generate_vblank_nmi() && self.status.in_vblank(){
            self.nmi_interrupt = Some(1);
        }
    }

    fn write_to_mask(&mut self, val: u8) {
        self.mask.update(val);
    }

    fn read_status(&mut self) -> u8 {
        // self.addr.reset_latch();
        // self.status.get_status()
        let data = self.status.get_status();
        self.status.set_vblank(false);
        self.addr.reset_latch();
        self.scroll.reset_latch();
        data
    }

    fn write_to_scroll(&mut self, val: u8) {
        self.scroll.write(val);
    }

    fn write_to_oam_addr(&mut self, val: u8) {
        self.oam.write_addr(val);
    }

    fn write_to_oam_data(&mut self, data: u8) {
        self.oam.write_to_data(data);
    }

    fn read_oam_data(&self) -> u8 {
        self.oam.read_data()
    }

    fn write_to_oam_dma(&mut self, data: &[u8; 256]) {
        self.oam.write_dma(data);
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn test_ppu_vram_writes() {
        let mut ppu = NesPPU::new_empty_rom();
        ppu.write_to_ppu_addr(0x23);
        ppu.write_to_ppu_addr(0x05);
        ppu.write_to_data(0x66);

        assert_eq!(ppu.vram[0x0305], 0x66);
    }

    #[test]
    fn test_ppu_vram_reads() {
        let mut ppu = NesPPU::new_empty_rom();
        ppu.write_to_ctrl(0);
        ppu.vram[0x0305] = 0x66;

        ppu.write_to_ppu_addr(0x23);
        ppu.write_to_ppu_addr(0x05);

        ppu.read_data(); //load_into_buffer
        assert_eq!(ppu.addr.get(), 0x2306);
        assert_eq!(ppu.read_data(), 0x66);
    }

    #[test]
    fn test_ppu_vram_reads_cross_page() {
        let mut ppu = NesPPU::new_empty_rom();
        ppu.write_to_ctrl(0);
        ppu.vram[0x01ff] = 0x66;
        ppu.vram[0x0200] = 0x77;

        ppu.write_to_ppu_addr(0x21);
        ppu.write_to_ppu_addr(0xff);

        ppu.read_data(); //load_into_buffer
        assert_eq!(ppu.read_data(), 0x66);
        assert_eq!(ppu.read_data(), 0x77);
    }

    #[test]
    fn test_ppu_vram_reads_step_32() {
        let mut ppu = NesPPU::new_empty_rom();
        ppu.write_to_ctrl(0b100);
        ppu.vram[0x01ff] = 0x66;
        ppu.vram[0x01ff + 32] = 0x77;
        ppu.vram[0x01ff + 64] = 0x88;

        ppu.write_to_ppu_addr(0x21);
        ppu.write_to_ppu_addr(0xff);

        ppu.read_data(); //load_into_buffer
        assert_eq!(ppu.read_data(), 0x66);
        assert_eq!(ppu.read_data(), 0x77);
        assert_eq!(ppu.read_data(), 0x88);
    }

    // Horizontal: https://www.nesdev.org/wiki/Mirroring
    //   [0x2000 A ] [0x2400 a ]
    //   [0x2800 B ] [0x2C00 b ]
    #[test]
    fn test_vram_horizontal_mirror() {
        let mut ppu = NesPPU::new_empty_rom();
        ppu.write_to_ppu_addr(0x24);
        ppu.write_to_ppu_addr(0x05);

        ppu.write_to_data(0x66); //write to a

        ppu.write_to_ppu_addr(0x28);
        ppu.write_to_ppu_addr(0x05);

        ppu.write_to_data(0x77); //write to B

        ppu.write_to_ppu_addr(0x20);
        ppu.write_to_ppu_addr(0x05);

        ppu.read_data(); //load into buffer
        assert_eq!(ppu.read_data(), 0x66); //read from A

        ppu.write_to_ppu_addr(0x2C);
        ppu.write_to_ppu_addr(0x05);

        ppu.read_data(); //load into buffer
        assert_eq!(ppu.read_data(), 0x77); //read from b
    }

    // Vertical: https://www.nesdev.org/wiki/Mirroring
    //   [0x2000 A ] [0x2400 B ]
    //   [0x2800 a ] [0x2C00 b ]
    #[test]
    fn test_vram_vertical_mirror() {
        let mut ppu = NesPPU::new(vec![0; 2048], Mirroring::Vertical);

        ppu.write_to_ppu_addr(0x20);
        ppu.write_to_ppu_addr(0x05);

        ppu.write_to_data(0x66); //write to A

        ppu.write_to_ppu_addr(0x2C);
        ppu.write_to_ppu_addr(0x05);

        ppu.write_to_data(0x77); //write to b

        ppu.write_to_ppu_addr(0x28);
        ppu.write_to_ppu_addr(0x05);

        ppu.read_data(); //load into buffer
        assert_eq!(ppu.read_data(), 0x66); //read from a

        ppu.write_to_ppu_addr(0x24);
        ppu.write_to_ppu_addr(0x05);

        ppu.read_data(); //load into buffer
        assert_eq!(ppu.read_data(), 0x77); //read from B
    }

    #[test]
    fn test_read_status_resets_latch() {
        let mut ppu = NesPPU::new_empty_rom();
        ppu.vram[0x0305] = 0x66;

        ppu.write_to_ppu_addr(0x21);
        ppu.write_to_ppu_addr(0x23);
        ppu.write_to_ppu_addr(0x05);

        ppu.read_data(); //load_into_buffer
        assert_ne!(ppu.read_data(), 0x66);

        ppu.read_status();

        ppu.write_to_ppu_addr(0x23);
        ppu.write_to_ppu_addr(0x05);

        ppu.read_data(); //load_into_buffer
        assert_eq!(ppu.read_data(), 0x66);
    }

    #[test]
    fn test_ppu_vram_mirroring() {
        let mut ppu = NesPPU::new_empty_rom();
        ppu.write_to_ctrl(0);
        ppu.vram[0x0305] = 0x66;

        ppu.write_to_ppu_addr(0x63); //0x6305 -> 0x2305
        ppu.write_to_ppu_addr(0x05);

        ppu.read_data(); //load into_buffer
        assert_eq!(ppu.read_data(), 0x66);
        // assert_eq!(ppu.addr.read(), 0x0306)
    }

    #[test]
    fn test_read_status_resets_vblank() {
        let mut ppu = NesPPU::new_empty_rom();
        ppu.status.set_vblank(true);

        let status = ppu.read_status();

        assert_eq!(status >> 7, 1);
        assert_eq!(ppu.status.get_status() >> 7, 0);
    }

    #[test]
    fn test_oam_read_write() {
        let mut ppu = NesPPU::new_empty_rom();
        ppu.write_to_oam_addr(0x10);
        ppu.write_to_oam_data(0x66);
        ppu.write_to_oam_data(0x77);

        ppu.write_to_oam_addr(0x10);
        assert_eq!(ppu.read_oam_data(), 0x66);

        ppu.write_to_oam_addr(0x11);
        assert_eq!(ppu.read_oam_data(), 0x77);
    }

    #[test]
    fn test_oam_dma() {
        let mut ppu = NesPPU::new_empty_rom();

        let mut data = [0x66; 256];
        data[0] = 0x77;
        data[255] = 0x88;

        ppu.write_to_oam_addr(0x10);
        ppu.write_to_oam_dma(&data);

        ppu.write_to_oam_addr(0xf); //wrap around
        assert_eq!(ppu.read_oam_data(), 0x88);

        ppu.write_to_oam_addr(0x10);
        assert_eq!(ppu.read_oam_data(), 0x77);
  
        ppu.write_to_oam_addr(0x11);
        assert_eq!(ppu.read_oam_data(), 0x66);
    }
}