use rand::RngExt;
use sdl3::event::Event;
use sdl3::EventPump;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::pixels::PixelFormat;

use cpu::{CPU,Mem};

use crate::bus::Bus;
use crate::cartridge::Rom;
use crate::tile_renderer::show_tile;

pub mod cpu;
pub mod opcodes;
pub mod bus;
pub mod cartridge;
pub mod trace;
pub mod ppu;
pub mod render;
pub mod tile_renderer;


#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate bitflags;

fn handle_user_input(cpu: &mut CPU, event_pump: &mut EventPump){
    for event in event_pump.poll_iter(){
        match event{

            Event::Quit { .. } | Event::KeyDown {keycode: Some(Keycode::Escape), .. } => {
                std::process::exit(0)
            },
            Event::KeyDown {keycode: Some(Keycode::Up), .. } => {
                cpu.mem_write(0xff, 0x77)
            },
            Event::KeyDown {keycode: Some(Keycode::Left), .. } => {
                cpu.mem_write(0xff, 0x61)
            },
            Event::KeyDown {keycode: Some(Keycode::Down), .. } => {
                cpu.mem_write(0xff, 0x73)
            },
            Event::KeyDown {keycode: Some(Keycode::Right), .. } => {
                cpu.mem_write(0xff, 0x64)
            },
            _ => {}
        }
    }
}

// converts from 6502 colors to sdl
fn color(byte: u8) -> Color{
    match byte{
        0 => Color::BLACK,
        1 => Color::WHITE,
        2 | 9 => Color::GREY,
        3 | 10 => Color::RED,
        4 | 11 => Color::GREEN,
        5 | 12 => Color::BLUE,
        6 | 13 => Color::MAGENTA,
        7 | 14 => Color::YELLOW,
        _ => Color::CYAN,
    }
}

fn read_screen_state(cpu: &CPU, frame: &mut [u8; 32 * 32 * 3]) -> bool {
    let mut frame_idx = 0;
    let mut update = false;
    for i in 0x0200..0x600{
        let color_idx = cpu.mem_read(i as u16);
        let (b1, b2, b3) = color(color_idx).rgb();
        if frame[frame_idx] != b1 || frame[frame_idx + 1] != b2 || frame[frame_idx + 2] != b3{
            frame[frame_idx] = b1;
            frame[frame_idx + 1] = b2;
            frame[frame_idx + 2] = b3;
            update = true;
        } 
        frame_idx += 3;
    }
    return update
}




fn main() {
    // initialize sdl2
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Snake", (32.0 * 30.0) as u32, (32.0 * 30.0) as u32)
        .position_centered()
        .build().unwrap();

    let mut canvas = window.into_canvas();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(30.0, 30.0).unwrap();

    let creator = canvas.texture_creator();
    let mut texture = creator.create_texture_target(PixelFormat::RGB24, 32, 32).unwrap();
    texture.set_scale_mode(sdl3::render::ScaleMode::Nearest);
    

    // load program
    let bytes: Vec<u8> = std::fs::read("cartridge_roms/pacman.nes").unwrap();
    let rom = Rom::new(&bytes).unwrap();


    // let mut cpu = CPU::new(Bus::new(rom));
    // // cpu.load(game_code);
    // cpu.reset();
    // cpu.program_counter = 0xc000;

    // // handle screen
    // let mut screen_state = [0 as u8; 32 * 32 * 3];
    // let mut rng = rand::rng();

    // let frame_time = std::time::Duration::from_nanos(100_000);
    // let mut next_frame = std::time::Instant::now() + frame_time;

    // cpu.run_with_callback(move |cpu| {
    //     println!("{}",trace::trace(&cpu));

    //     handle_user_input(cpu, &mut event_pump);

    //     cpu.mem_write(0xfe, rng.random_range(1..16));

    //     if read_screen_state(cpu, &mut screen_state){
    //         texture.update(None, &screen_state, 32 * 3).unwrap();
    //         canvas.copy(&texture, None, None).unwrap();
    //         canvas.present();
    //     }

    //     if let Some(remaining) = next_frame.checked_duration_since(std::time::Instant::now()){
    //         if remaining > std::time::Duration::from_millis(1) {
    //             std::thread::sleep(remaining - std::time::Duration::from_millis(1));
    //         }
    //         while std::time::Instant::now() < next_frame{
    //             std::hint::spin_loop();
    //         }
    //     }
    //     next_frame += frame_time;
    // });

    // Tile Test
    let mut tile_n: usize = 0;
    let tile_frame = show_tile(&rom.chr_rom, 1, tile_n);

    texture.update(None, &tile_frame.data, 256*3).unwrap();
    canvas.copy(&texture, None, None).unwrap();
    canvas.present();

    loop{
        for event in event_pump.poll_iter(){
            match event{
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => std::process::exit(0),
                Event::KeyDown{ keycode: Some(Keycode::Space) ,..} => {
                    tile_n += 1;
                    let tile_frame = show_tile(&rom.chr_rom, 1, tile_n);

                    texture.update(None, &tile_frame.data, 256*3).unwrap();
                    canvas.copy(&texture, None, None).unwrap();
                    canvas.present();
                }
                _ => {}
            }
        }
    }

    print!("Exiting...");
}