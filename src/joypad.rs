use bitflags::Flags;

bitflags! {
    #[derive(Clone, Copy)]
    pub struct JoypadButtons: u8{
        const BUTTON_A  = 0b0000_0001;
        const BUTTON_B  = 0b0000_0010;
        const SELECT    = 0b0000_0100;
        const START     = 0b0000_1000;
        const UP        = 0b0001_0000;
        const DOWN      = 0b0010_0000;
        const LEFT      = 0b0100_0000;
        const RIGHT     = 0b1000_0000;
    }
}

pub struct Joypad{
    strobe_mode: bool,
    button_idx: u8,
    button_status: JoypadButtons,
}

impl Joypad{
    pub fn new() -> Self{
        Joypad{
            strobe_mode: false,
            button_idx: 0,
            button_status: JoypadButtons::from_bits_truncate(0),
        }
    }

    pub fn set_strobe(&mut self, is_strobe: bool){
        self.strobe_mode = is_strobe;
        if is_strobe{
            self.button_idx = 0;
        }
    }

    pub fn get_button(&mut self) -> u8{
        if self.button_idx > 7{
            return 1;
        }
        let res = (self.button_status.bits() >> self.button_idx) & 1;

        if !self.strobe_mode{
            self.button_idx += 1
        }

        res
    }

    pub fn set_button_pressed(&mut self, button:JoypadButtons, pressed:bool){
        self.button_status.set(button, pressed);
    }
}