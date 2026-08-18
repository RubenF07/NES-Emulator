use std::collections::HashMap;

use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::PixelFormat;

use cpu::CPU;

use crate::bus::Bus;
use crate::joypad::Joypad;
use crate::joypad::JoypadButtons;
use crate::ppu::NesPPU;
use crate::cartridge::Rom;
use crate::render::frame::Frame;
use crate::render::render;
use crate::tile_renderer::show_tile;

pub mod cpu;
pub mod opcodes;
pub mod bus;
pub mod cartridge;
pub mod trace;
pub mod ppu;
pub mod render;
pub mod tile_renderer;
pub mod joypad;


#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate bitflags;

fn get_key_map() -> HashMap<Keycode, JoypadButtons>{
    let mut map = HashMap::new();
    map.insert(Keycode::Down, JoypadButtons::DOWN);
    map.insert(Keycode::Up, JoypadButtons::UP);
    map.insert(Keycode::Right, JoypadButtons::RIGHT);
    map.insert(Keycode::Left, JoypadButtons::LEFT);
    map.insert(Keycode::Space, JoypadButtons::SELECT);
    map.insert(Keycode::Return, JoypadButtons::START);
    map.insert(Keycode::A, JoypadButtons::BUTTON_A);
    map.insert(Keycode::S, JoypadButtons::BUTTON_B);

    map
}


fn main() {
    // initialize sdl2
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("NES Emulator", (256.0 * 3.0) as u32, (240.0 * 3.0) as u32)
        .position_centered()
        .build().unwrap();

    let mut canvas = window.into_canvas();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(3.0, 3.0).unwrap();

    let creator = canvas.texture_creator();
    let mut texture = creator.create_texture_target(PixelFormat::RGB24, 256, 240).unwrap();
    texture.set_scale_mode(sdl3::render::ScaleMode::Nearest);
    

    // load program
    let bytes: Vec<u8> = std::fs::read("cartridge_roms/mario.nes").unwrap();
    let rom = Rom::new(&bytes).unwrap();

    let mut frame = Frame::new();

    let key_map = get_key_map();

    // game cycle
    let bus = Bus::new(rom, move |ppu: &NesPPU, joypad: &mut Joypad| {
        render(ppu, &mut frame);
        texture.update(None, &frame.data, 256 * 3).unwrap();

        canvas.copy(&texture, None, None).unwrap();

        canvas.present();
        for event in event_pump.poll_iter(){
            match event{
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => std::process::exit(0),
                
                Event::KeyDown { keycode, .. } => {
                    if let Some(key) = key_map.get(&keycode.unwrap_or(Keycode::Ampersand)){
                        joypad.set_button_pressed(*key, true)
                    }
                },
                Event::KeyUp { keycode, .. } => {
                    if let Some(key) = key_map.get(&keycode.unwrap_or(Keycode::Ampersand)){
                        joypad.set_button_pressed(*key, false)
                    }
                },

                _ => {}
            }
        }
    });

    let mut cpu = CPU::new(bus);

    cpu.reset();
    cpu.run();


    // // Tile Test
    // let mut tile_n: usize = 0;
    // let tile_frame = show_tile(&rom.chr_rom, 1, tile_n);

    // texture.update(None, &tile_frame.data, 256*3).unwrap();
    // canvas.copy(&texture, None, None).unwrap();
    // canvas.present();

    // loop{
    //     for event in event_pump.poll_iter(){
    //         match event{
    //             Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), ..} => std::process::exit(0),
    //             Event::KeyDown{ keycode: Some(Keycode::Space) ,..} => {
    //                 tile_n += 1;
    //                 let tile_frame = show_tile(&rom.chr_rom, 1, tile_n);

    //                 texture.update(None, &tile_frame.data, 256*3).unwrap();
    //                 canvas.copy(&texture, None, None).unwrap();
    //                 canvas.present();
    //             }
    //             _ => {}
    //         }
    //     }
    // }

    print!("Exiting...");
}