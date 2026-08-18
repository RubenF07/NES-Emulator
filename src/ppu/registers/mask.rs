use bitflags::Flags;

bitflags! {

   // BGRs bMmG
   // |||| ||||
   // |||| |||+- Greyscale (0: normal color, 1: greyscale)
   // |||| ||+-- 1: Show background in leftmost 8 pixels of screen, 0: Hide
   // |||| |+--- 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
   // |||| +---- 1: Enable background rendering
   // |||+------ 1: Enable sprite rendering
   // ||+------- Emphasize red (green on PAL/Dendy)
   // |+-------- Emphasize green (red on PAL/Dendy)
   // +--------- Emphasize blue

    pub struct MaskRegister: u8{
        const GRAYSCALE            = 0b0000_0001;
        const LEFT_8PX_BGR         = 0b0000_0010;
        const LEFT_8PX_SPR         = 0b0000_0100;
        const BACKGROUND_RENDERING = 0b0000_1000;
        const SPRITE_RENDERING     = 0b0001_0000;
        const EMPH_RED             = 0b0010_0000;
        const EMPH_GREEN           = 0b0100_0000;
        const EMPH_BLUE            = 0b1000_0000;
    }
}

impl MaskRegister {
    pub fn new() -> Self{
        MaskRegister::from_bits_truncate(0)
    }

    pub fn update(&mut self, data: u8){
        self.clear();
        self.insert(MaskRegister::from_bits_truncate(data));
    }

    pub fn show_sprites(&self) -> bool{
        self.contains(MaskRegister::SPRITE_RENDERING)
    }
}