// use bitflags::Flags;

bitflags! {

    // VSOx xxxx
    // |||| ||||
    // |||+-++++- (PPU open bus or 2C05 PPU identifier)
    // ||+------- Sprite overflow flag
    // |+-------- Sprite 0 hit flag
    // +--------- Vblank flag, cleared on read. Unreliable; see below.

    pub struct StatusRegister: u8{
        const SPRITE_OVERFLOW = 0b0010_0000;
        const SPRITE_0_HIT    = 0b0100_0000;
        const V_BLANK          = 0b1000_0000;
    }
}

impl StatusRegister {
    pub fn new() -> Self{
        StatusRegister::from_bits_truncate(0)
    }

    pub fn get_status(&mut self) -> u8{
        let res = self.bits();
        self.remove(StatusRegister::V_BLANK);

        return res;
    }

    pub fn set_vblank(&mut self, val: bool){
        self.set(StatusRegister::V_BLANK, val);
    }
}