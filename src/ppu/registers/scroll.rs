// 1st write
// 7  bit  0
// ---- ----
// XXXX XXXX
// |||| ||||
// ++++-++++- X scroll bits 7-0 (bit 8 in PPUCTRL bit 0)

// 2nd write
// 7  bit  0
// ---- ----
// YYYY YYYY
// |||| ||||
// ++++-++++- Y scroll bits 7-0 (bit 8 in PPUCTRL bit 1)

pub struct ScrollRegister{
    pub x_scroll: u8,
    pub y_scroll: u8,
    first_write: bool,
}

impl ScrollRegister{
    pub fn new() -> Self{
        ScrollRegister{
            x_scroll: 0,
            y_scroll: 0,
            first_write: true,
        }
    }

    pub fn write(&mut self, data: u8){
        if self.first_write{
            self.x_scroll = data;
        }
        else{
            self.y_scroll = data;
        }
        self.first_write = !self.first_write; 
    }

    pub fn reset_latch(&mut self){
        self.first_write = true;
    }
}