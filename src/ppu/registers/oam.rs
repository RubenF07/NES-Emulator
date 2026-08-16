pub struct OAM{
    addr: u8,
    pub data: [u8; 256],
}

impl OAM{
    pub fn new() -> Self{
        OAM{
            addr: 0,
            data: [0;256],
            // dma: u8 // high byte of source addr - not used
        }
    }

    pub fn write_addr(&mut self, val: u8){
        self.addr = val;
    }

    pub fn read_data(&self) -> u8{
        return self.data[self.addr as usize];
    }

    pub fn write_to_data(&mut self, data: u8){
        self.data[self.addr as usize] = data;
        self.addr = self.addr.wrapping_add(1);
    }

    pub fn write_dma(&mut self, data: &[u8; 256]){
        for byte in data {
            self.write_to_data(*byte);
        }
    }
}