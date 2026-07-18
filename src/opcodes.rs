use crate::cpu::AddressingMode::{self, NoneAddresssing};
use std::collections::HashMap;

pub struct OpCode {
    pub hex: u8,
    pub mnemonic: &'static str,
    pub bytes: u8,
    pub mode: AddressingMode,
}
impl OpCode{
    pub fn new(hex: u8, mnemonic: &'static str, bytes: u8, mode: AddressingMode) -> Self {
        OpCode{
            hex: hex,
            mnemonic: mnemonic,
            bytes: bytes,
            mode: mode,
        }
    }
}
lazy_static!{
pub static ref OPS_CODE_MAP: HashMap<u8, OpCode> = {
    let mut m = HashMap::new();
    m.insert(0x00, OpCode::new(0x00, "BRK", 1, AddressingMode::NoneAddresssing));
    m.insert(0xaa, OpCode::new(0xaa, "TAX", 1, AddressingMode::NoneAddresssing));    
    m.insert(0xe8, OpCode::new(0xe8, "INX", 1, AddressingMode::NoneAddresssing));

    m.insert(0xa9, OpCode::new(0xa9, "LDA", 2, AddressingMode::Immediate));
    m.insert(0xa5, OpCode::new(0xa5, "LDA", 2, AddressingMode::ZeroPage));
    m.insert(0xb5, OpCode::new(0xb5, "LDA", 2, AddressingMode::ZeroPage_X));
    m.insert(0xad, OpCode::new(0xad, "LDA", 3, AddressingMode::Absolute));
    m.insert(0xbd, OpCode::new(0xbd, "LDA", 3, AddressingMode::Absolute_X));
    m.insert(0xb9, OpCode::new(0xb9, "LDA", 3, AddressingMode::Absolute_Y));
    m.insert(0xa1, OpCode::new(0xa1, "LDA", 2, AddressingMode::Indirect_X));
    m.insert(0xb1, OpCode::new(0xb1, "LDA", 2, AddressingMode::Indirect_Y));
    
    m.insert(0x85, OpCode::new(0x85, "STA", 2, AddressingMode::ZeroPage));
    m.insert(0x95, OpCode::new(0x95, "STA", 2, AddressingMode::ZeroPage_X));
    m.insert(0x8d, OpCode::new(0x8d, "STA", 3, AddressingMode::Absolute));
    m.insert(0x9d, OpCode::new(0x9d, "STA", 3, AddressingMode::Absolute_X));
    m.insert(0x99, OpCode::new(0x99, "STA", 3, AddressingMode::Absolute_Y));
    m.insert(0x81, OpCode::new(0x81, "STA", 2, AddressingMode::Indirect_X));
    m.insert(0x91, OpCode::new(0x91, "STA", 2, AddressingMode::Indirect_Y));


    return m
};
}