use crate::cpu::AddressingMode;
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

    m.insert(0x69, OpCode::new(0x69, "ADC", 2, AddressingMode::Immediate));
    m.insert(0x65, OpCode::new(0x65, "ADC", 2, AddressingMode::ZeroPage));
    m.insert(0x75, OpCode::new(0x75, "ADC", 2, AddressingMode::ZeroPage_X));
    m.insert(0x6d, OpCode::new(0x6d, "ADC", 3, AddressingMode::Absolute));
    m.insert(0x7d, OpCode::new(0x7d, "ADC", 3, AddressingMode::Absolute_X));
    m.insert(0x79, OpCode::new(0x79, "ADC", 3, AddressingMode::Absolute_Y));
    m.insert(0x61, OpCode::new(0x61, "ADC", 2, AddressingMode::Indirect_X));
    m.insert(0x71, OpCode::new(0x71, "ADC", 2, AddressingMode::Indirect_Y));
    
    m.insert(0x29, OpCode::new(0x29, "AND", 2, AddressingMode::Immediate));
    m.insert(0x25, OpCode::new(0x25, "AND", 2, AddressingMode::ZeroPage));
    m.insert(0x35, OpCode::new(0x35, "AND", 2, AddressingMode::ZeroPage_X));
    m.insert(0x2d, OpCode::new(0x2d, "AND", 3, AddressingMode::Absolute));
    m.insert(0x3d, OpCode::new(0x3d, "AND", 3, AddressingMode::Absolute_X));
    m.insert(0x39, OpCode::new(0x39, "AND", 3, AddressingMode::Absolute_Y));
    m.insert(0x21, OpCode::new(0x21, "AND", 2, AddressingMode::Indirect_X));
    m.insert(0x31, OpCode::new(0x31, "AND", 2, AddressingMode::Indirect_Y));
    
    m.insert(0x0a, OpCode::new(0x0a, "ASL", 1, AddressingMode::Accumulator));
    m.insert(0x06, OpCode::new(0x06, "ASL", 2, AddressingMode::ZeroPage));
    m.insert(0x16, OpCode::new(0x16, "ASL", 2, AddressingMode::ZeroPage_X));
    m.insert(0x0e, OpCode::new(0x0e, "ASL", 3, AddressingMode::Absolute));
    m.insert(0x1e, OpCode::new(0x1e, "ASL", 3, AddressingMode::Absolute_X));
    
    m.insert(0x90, OpCode::new(0x90, "BCC", 2, AddressingMode::NoneAddressing));
    
    m.insert(0xb0, OpCode::new(0xb0, "BCS", 2, AddressingMode::NoneAddressing));
    
    m.insert(0xf0, OpCode::new(0xf0, "BEQ", 2, AddressingMode::NoneAddressing));
    
    m.insert(0x24, OpCode::new(0x24, "BIT", 2, AddressingMode::ZeroPage));
    m.insert(0x2c, OpCode::new(0x2c, "BIT", 3, AddressingMode::Absolute));
    
    m.insert(0x30, OpCode::new(0x30, "BMI", 2, AddressingMode::NoneAddressing));
    
    m.insert(0xd0, OpCode::new(0xd0, "BNE", 2, AddressingMode::NoneAddressing));
    
    m.insert(0x10, OpCode::new(0x10, "BPL", 2, AddressingMode::NoneAddressing));
    
    m.insert(0x00, OpCode::new(0x00, "BRK", 1, AddressingMode::NoneAddressing));
    
    m.insert(0x50, OpCode::new(0x50, "BVC", 2, AddressingMode::NoneAddressing));
    
    m.insert(0x70, OpCode::new(0x70, "BVS", 2, AddressingMode::NoneAddressing));
    
    m.insert(0x18, OpCode::new(0x18, "CLC", 1, AddressingMode::NoneAddressing));
    
    m.insert(0xd8, OpCode::new(0xd8, "CLD", 1, AddressingMode::NoneAddressing));
    
    m.insert(0x58, OpCode::new(0x58, "CLI", 1, AddressingMode::NoneAddressing));
    
    m.insert(0xb8, OpCode::new(0xb8, "CLV", 1, AddressingMode::NoneAddressing));
    
    m.insert(0xc9, OpCode::new(0xc9, "CMP", 2, AddressingMode::Immediate));
    m.insert(0xc5, OpCode::new(0xc5, "CMP", 2, AddressingMode::ZeroPage));
    m.insert(0xd5, OpCode::new(0xd5, "CMP", 2, AddressingMode::ZeroPage_X));
    m.insert(0xcd, OpCode::new(0xcd, "CMP", 3, AddressingMode::Absolute));
    m.insert(0xdd, OpCode::new(0xdd, "CMP", 3, AddressingMode::Absolute_X));
    m.insert(0xd9, OpCode::new(0xd9, "CMP", 3, AddressingMode::Absolute_Y));
    m.insert(0xc1, OpCode::new(0xc1, "CMP", 2, AddressingMode::Indirect_X));
    m.insert(0xd1, OpCode::new(0xd1, "CMP", 2, AddressingMode::Indirect_Y));
    
    m.insert(0xe0, OpCode::new(0xe0, "CPX", 2, AddressingMode::Immediate));
    m.insert(0xe4, OpCode::new(0xe4, "CPX", 2, AddressingMode::ZeroPage));
    m.insert(0xec, OpCode::new(0xec, "CPX", 3, AddressingMode::Absolute));
    
    m.insert(0xc0, OpCode::new(0xc0, "CPY", 2, AddressingMode::Immediate));
    m.insert(0xc4, OpCode::new(0xc4, "CPY", 2, AddressingMode::ZeroPage));
    m.insert(0xcc, OpCode::new(0xcc, "CPY", 3, AddressingMode::Absolute));
    
    m.insert(0xc6, OpCode::new(0xc6, "DEC", 2, AddressingMode::ZeroPage));
    m.insert(0xd6, OpCode::new(0xd6, "DEC", 2, AddressingMode::ZeroPage_X));
    m.insert(0xce, OpCode::new(0xce, "DEC", 3, AddressingMode::Absolute));
    m.insert(0xde, OpCode::new(0xde, "DEC", 3, AddressingMode::Absolute_X));
    
    m.insert(0xca, OpCode::new(0xca, "DEX", 1, AddressingMode::NoneAddressing));
    
    m.insert(0x88, OpCode::new(0x88, "DEY", 1, AddressingMode::NoneAddressing));
    
    m.insert(0x49, OpCode::new(0x49, "EOR", 2, AddressingMode::Immediate));
    m.insert(0x45, OpCode::new(0x45, "EOR", 2, AddressingMode::ZeroPage));
    m.insert(0x55, OpCode::new(0x55, "EOR", 2, AddressingMode::ZeroPage_X));
    m.insert(0x4d, OpCode::new(0x4d, "EOR", 3, AddressingMode::Absolute));
    m.insert(0x5d, OpCode::new(0x5d, "EOR", 3, AddressingMode::Absolute_X));
    m.insert(0x59, OpCode::new(0x59, "EOR", 3, AddressingMode::Absolute_Y));
    m.insert(0x41, OpCode::new(0x41, "EOR", 2, AddressingMode::Indirect_X));
    m.insert(0x51, OpCode::new(0x51, "EOR", 2, AddressingMode::Indirect_Y));
    
    m.insert(0xe6, OpCode::new(0xe6, "INC", 2, AddressingMode::ZeroPage));
    m.insert(0xf6, OpCode::new(0xf6, "INC", 2, AddressingMode::ZeroPage_X));
    m.insert(0xee, OpCode::new(0xee, "INC", 3, AddressingMode::Absolute));
    m.insert(0xfe, OpCode::new(0xfe, "INC", 3, AddressingMode::Absolute_X));
    
    m.insert(0xe8, OpCode::new(0xe8, "INX", 1, AddressingMode::NoneAddressing));
    
    m.insert(0xc8, OpCode::new(0xc8, "INY", 1, AddressingMode::NoneAddressing));
    
    m.insert(0x4c, OpCode::new(0x4c, "JMP", 3, AddressingMode::Absolute));
    m.insert(0x6c, OpCode::new(0x6c, "JMP", 3, AddressingMode::NoneAddressing)); //Indirect
    
    m.insert(0x20, OpCode::new(0x20, "JSR", 3, AddressingMode::Absolute));
    
    m.insert(0xa9, OpCode::new(0xa9, "LDA", 2, AddressingMode::Immediate));
    m.insert(0xa5, OpCode::new(0xa5, "LDA", 2, AddressingMode::ZeroPage));
    m.insert(0xb5, OpCode::new(0xb5, "LDA", 2, AddressingMode::ZeroPage_X));
    m.insert(0xad, OpCode::new(0xad, "LDA", 3, AddressingMode::Absolute));
    m.insert(0xbd, OpCode::new(0xbd, "LDA", 3, AddressingMode::Absolute_X));
    m.insert(0xb9, OpCode::new(0xb9, "LDA", 3, AddressingMode::Absolute_Y));
    m.insert(0xa1, OpCode::new(0xa1, "LDA", 2, AddressingMode::Indirect_X));
    m.insert(0xb1, OpCode::new(0xb1, "LDA", 2, AddressingMode::Indirect_Y));
    
    m.insert(0xa2, OpCode::new(0xa2, "LDX", 2, AddressingMode::Immediate));
    m.insert(0xa6, OpCode::new(0xa6, "LDX", 2, AddressingMode::ZeroPage));
    m.insert(0xb6, OpCode::new(0xb6, "LDX", 2, AddressingMode::ZeroPage_Y));
    m.insert(0xae, OpCode::new(0xae, "LDX", 3, AddressingMode::Absolute));
    m.insert(0xbe, OpCode::new(0xbe, "LDX", 3, AddressingMode::Absolute_Y));
    
    m.insert(0xa0, OpCode::new(0xa0, "LDY", 2, AddressingMode::Immediate));
    m.insert(0xa4, OpCode::new(0xa4, "LDY", 2, AddressingMode::ZeroPage));
    m.insert(0xb4, OpCode::new(0xb4, "LDY", 2, AddressingMode::ZeroPage_X));
    m.insert(0xac, OpCode::new(0xac, "LDY", 3, AddressingMode::Absolute));
    m.insert(0xbc, OpCode::new(0xbc, "LDY", 3, AddressingMode::Absolute_X));
    
    m.insert(0x4a, OpCode::new(0x4a, "LSR", 1, AddressingMode::Accumulator));
    m.insert(0x46, OpCode::new(0x46, "LSR", 2, AddressingMode::ZeroPage));
    m.insert(0x56, OpCode::new(0x56, "LSR", 2, AddressingMode::ZeroPage_X));
    m.insert(0x4e, OpCode::new(0x4e, "LSR", 3, AddressingMode::Absolute));
    m.insert(0x5e, OpCode::new(0x5e, "LSR", 3, AddressingMode::Absolute_X));
    
    m.insert(0xea, OpCode::new(0xea, "NOP", 1, AddressingMode::NoneAddressing));
    
    m.insert(0x09, OpCode::new(0x09, "ORA", 2, AddressingMode::Immediate));
    m.insert(0x05, OpCode::new(0x05, "ORA", 2, AddressingMode::ZeroPage));
    m.insert(0x15, OpCode::new(0x15, "ORA", 2, AddressingMode::ZeroPage_X));
    m.insert(0x0d, OpCode::new(0x0d, "ORA", 3, AddressingMode::Absolute));
    m.insert(0x1d, OpCode::new(0x1d, "ORA", 3, AddressingMode::Absolute_X));
    m.insert(0x19, OpCode::new(0x19, "ORA", 3, AddressingMode::Absolute_Y));
    m.insert(0x01, OpCode::new(0x01, "ORA", 2, AddressingMode::Indirect_X));
    m.insert(0x11, OpCode::new(0x11, "ORA", 2, AddressingMode::Indirect_Y));
    
    m.insert(0x48, OpCode::new(0x48, "PHA", 1, AddressingMode::NoneAddressing));
    
    m.insert(0x08, OpCode::new(0x08, "PHP", 1, AddressingMode::NoneAddressing));
    
    m.insert(0x68, OpCode::new(0x68, "PLA", 1, AddressingMode::NoneAddressing));
    
    m.insert(0x28, OpCode::new(0x28, "PLP", 1, AddressingMode::NoneAddressing));
    
    m.insert(0x2a, OpCode::new(0x2a, "ROL", 1, AddressingMode::Accumulator));
    m.insert(0x26, OpCode::new(0x26, "ROL", 2, AddressingMode::ZeroPage));
    m.insert(0x36, OpCode::new(0x36, "ROL", 2, AddressingMode::ZeroPage_X));
    m.insert(0x2e, OpCode::new(0x2e, "ROL", 3, AddressingMode::Absolute));
    m.insert(0x3e, OpCode::new(0x3e, "ROL", 3, AddressingMode::Absolute_X));
    
    m.insert(0x6a, OpCode::new(0x6a, "ROR", 1, AddressingMode::Accumulator));
    m.insert(0x66, OpCode::new(0x66, "ROR", 2, AddressingMode::ZeroPage));
    m.insert(0x76, OpCode::new(0x76, "ROR", 2, AddressingMode::ZeroPage_X));
    m.insert(0x6e, OpCode::new(0x6e, "ROR", 3, AddressingMode::Absolute));
    m.insert(0x7e, OpCode::new(0x7e, "ROR", 3, AddressingMode::Absolute_X));
    
    m.insert(0x60, OpCode::new(0x60, "RTS", 1, AddressingMode::NoneAddressing));
    
    m.insert(0x40, OpCode::new(0x40, "RTI", 1, AddressingMode::NoneAddressing));
    
    m.insert(0xe9, OpCode::new(0xe9, "SBC", 2, AddressingMode::Immediate));
    m.insert(0xe5, OpCode::new(0xe5, "SBC", 2, AddressingMode::ZeroPage));
    m.insert(0xf5, OpCode::new(0xf5, "SBC", 2, AddressingMode::ZeroPage_X));
    m.insert(0xed, OpCode::new(0xed, "SBC", 3, AddressingMode::Absolute));
    m.insert(0xfd, OpCode::new(0xfd, "SBC", 3, AddressingMode::Absolute_X));
    m.insert(0xf9, OpCode::new(0xf9, "SBC", 3, AddressingMode::Absolute_Y));
    m.insert(0xe1, OpCode::new(0xe1, "SBC", 2, AddressingMode::Indirect_X));
    m.insert(0xf1, OpCode::new(0xf1, "SBC", 2, AddressingMode::Indirect_Y));
    
    m.insert(0x38, OpCode::new(0x38, "SEC", 1, AddressingMode::NoneAddressing));
    
    m.insert(0xf8, OpCode::new(0xf8, "SED", 1, AddressingMode::NoneAddressing));
    
    m.insert(0x78, OpCode::new(0x78, "SEI", 1, AddressingMode::NoneAddressing));
    
    m.insert(0x85, OpCode::new(0x85, "STA", 2, AddressingMode::ZeroPage));
    m.insert(0x95, OpCode::new(0x95, "STA", 2, AddressingMode::ZeroPage_X));
    m.insert(0x8d, OpCode::new(0x8d, "STA", 3, AddressingMode::Absolute));
    m.insert(0x9d, OpCode::new(0x9d, "STA", 3, AddressingMode::Absolute_X));
    m.insert(0x99, OpCode::new(0x99, "STA", 3, AddressingMode::Absolute_Y));
    m.insert(0x81, OpCode::new(0x81, "STA", 2, AddressingMode::Indirect_X));
    m.insert(0x91, OpCode::new(0x91, "STA", 2, AddressingMode::Indirect_Y));
    
    m.insert(0x86, OpCode::new(0x86, "STX", 2, AddressingMode::ZeroPage));
    m.insert(0x96, OpCode::new(0x96, "STX", 2, AddressingMode::ZeroPage_Y));
    m.insert(0x8e, OpCode::new(0x8e, "STX", 3, AddressingMode::Absolute));
    
    m.insert(0x84, OpCode::new(0x84, "STY", 2, AddressingMode::ZeroPage));
    m.insert(0x94, OpCode::new(0x94, "STY", 2, AddressingMode::ZeroPage_X));
    m.insert(0x8c, OpCode::new(0x8c, "STY", 3, AddressingMode::Absolute));
    
    m.insert(0xaa, OpCode::new(0xaa, "TAX", 1, AddressingMode::NoneAddressing));    
    
    m.insert(0xa8, OpCode::new(0xa8, "TAY", 1, AddressingMode::NoneAddressing));    
    
    m.insert(0xba, OpCode::new(0xba, "TSX", 1, AddressingMode::NoneAddressing));
    
    m.insert(0x8a, OpCode::new(0x8a, "TXA", 1, AddressingMode::NoneAddressing));
    
    m.insert(0x9a, OpCode::new(0x9a, "TXS", 1, AddressingMode::NoneAddressing));
    
    m.insert(0x98, OpCode::new(0x98, "TYA", 1, AddressingMode::NoneAddressing));
    
    // Unofficial Codes
    
    m.insert(0x4b, OpCode::new(0x4b, "ALR", 2, AddressingMode::Immediate));
    
    m.insert(0x0b, OpCode::new(0x0b, "ANC", 2, AddressingMode::Immediate));
    m.insert(0x2b, OpCode::new(0x2b, "ANC", 2, AddressingMode::Immediate));
    
    m.insert(0x6b, OpCode::new(0x6b, "ARR", 2, AddressingMode::Immediate));
    
    m.insert(0xcb, OpCode::new(0xcb, "AXS", 2, AddressingMode::Immediate));
    
    m.insert(0xa7, OpCode::new(0xa7, "LAX", 2, AddressingMode::ZeroPage));
    m.insert(0xb7, OpCode::new(0xb7, "LAX", 2, AddressingMode::ZeroPage_Y));
    m.insert(0xaf, OpCode::new(0xaf, "LAX", 3, AddressingMode::Absolute));
    m.insert(0xbf, OpCode::new(0xbf, "LAX", 3, AddressingMode::Absolute_Y));
    m.insert(0xa3, OpCode::new(0xa3, "LAX", 2, AddressingMode::Indirect_X));
    m.insert(0xb3, OpCode::new(0xb3, "LAX", 2, AddressingMode::Indirect_Y));
    
    m.insert(0x87, OpCode::new(0x87, "SAX", 2, AddressingMode::ZeroPage));
    m.insert(0x97, OpCode::new(0x97, "SAX", 2, AddressingMode::ZeroPage_Y));
    m.insert(0x83, OpCode::new(0x83, "SAX", 2, AddressingMode::Indirect_X));
    m.insert(0x8f, OpCode::new(0x8f, "SAX", 3, AddressingMode::Absolute));
    
    m.insert(0xc7, OpCode::new(0xc7, "DCP", 2, AddressingMode::ZeroPage));
    m.insert(0xd7, OpCode::new(0xd7, "DCP", 2, AddressingMode::ZeroPage_X));
    m.insert(0xcf, OpCode::new(0xcf, "DCP", 3, AddressingMode::Absolute));
    m.insert(0xdf, OpCode::new(0xdf, "DCP", 3, AddressingMode::Absolute_X));
    m.insert(0xdb, OpCode::new(0xdb, "DCP", 3, AddressingMode::Absolute_Y));
    m.insert(0xc3, OpCode::new(0xc3, "DCP", 2, AddressingMode::Indirect_X));
    m.insert(0xd3, OpCode::new(0xd3, "DCP", 2, AddressingMode::Indirect_Y));
    
    m.insert(0xe7, OpCode::new(0xe7, "ISC", 2, AddressingMode::ZeroPage));
    m.insert(0xf7, OpCode::new(0xf7, "ISC", 2, AddressingMode::ZeroPage_X));
    m.insert(0xef, OpCode::new(0xef, "ISC", 3, AddressingMode::Absolute));
    m.insert(0xff, OpCode::new(0xff, "ISC", 3, AddressingMode::Absolute_X));
    m.insert(0xfb, OpCode::new(0xfb, "ISC", 3, AddressingMode::Absolute_Y));
    m.insert(0xe3, OpCode::new(0xe3, "ISC", 2, AddressingMode::Indirect_X));
    m.insert(0xf3, OpCode::new(0xf3, "ISC", 2, AddressingMode::Indirect_Y));
    
    m.insert(0x27, OpCode::new(0x27, "RLA", 2, AddressingMode::ZeroPage));
    m.insert(0x37, OpCode::new(0x37, "RLA", 2, AddressingMode::ZeroPage_X));
    m.insert(0x2f, OpCode::new(0x2f, "RLA", 3, AddressingMode::Absolute));
    m.insert(0x3f, OpCode::new(0x3f, "RLA", 3, AddressingMode::Absolute_X));
    m.insert(0x3b, OpCode::new(0x3b, "RLA", 3, AddressingMode::Absolute_Y));
    m.insert(0x23, OpCode::new(0x23, "RLA", 2, AddressingMode::Indirect_X));
    m.insert(0x33, OpCode::new(0x33, "RLA", 2, AddressingMode::Indirect_Y));
    
    m.insert(0x67, OpCode::new(0x67, "RRA", 2, AddressingMode::ZeroPage));
    m.insert(0x77, OpCode::new(0x77, "RRA", 2, AddressingMode::ZeroPage_X));
    m.insert(0x6f, OpCode::new(0x6f, "RRA", 3, AddressingMode::Absolute));
    m.insert(0x7f, OpCode::new(0x7f, "RRA", 3, AddressingMode::Absolute_X));
    m.insert(0x7b, OpCode::new(0x7b, "RRA", 3, AddressingMode::Absolute_Y));
    m.insert(0x63, OpCode::new(0x63, "RRA", 2, AddressingMode::Indirect_X));
    m.insert(0x73, OpCode::new(0x73, "RRA", 2, AddressingMode::Indirect_Y));
    
    m.insert(0x07, OpCode::new(0x07, "SLO", 2, AddressingMode::ZeroPage));
    m.insert(0x17, OpCode::new(0x17, "SLO", 2, AddressingMode::ZeroPage_X));
    m.insert(0x0f, OpCode::new(0x0f, "SLO", 3, AddressingMode::Absolute));
    m.insert(0x1f, OpCode::new(0x1f, "SLO", 3, AddressingMode::Absolute_X));
    m.insert(0x1b, OpCode::new(0x1b, "SLO", 3, AddressingMode::Absolute_Y));
    m.insert(0x03, OpCode::new(0x03, "SLO", 2, AddressingMode::Indirect_X));
    m.insert(0x13, OpCode::new(0x13, "SLO", 2, AddressingMode::Indirect_Y));
    
    m.insert(0x47, OpCode::new(0x47, "SRE", 2, AddressingMode::ZeroPage));
    m.insert(0x57, OpCode::new(0x57, "SRE", 2, AddressingMode::ZeroPage_X));
    m.insert(0x4f, OpCode::new(0x4f, "SRE", 3, AddressingMode::Absolute));
    m.insert(0x5f, OpCode::new(0x5f, "SRE", 3, AddressingMode::Absolute_X));
    m.insert(0x5b, OpCode::new(0x5b, "SRE", 3, AddressingMode::Absolute_Y));
    m.insert(0x43, OpCode::new(0x43, "SRE", 2, AddressingMode::Indirect_X));
    m.insert(0x53, OpCode::new(0x53, "SRE", 2, AddressingMode::Indirect_Y));
    
    // Duplicate
    m.insert(0xeb, OpCode::new(0xeb, "SBC", 2, AddressingMode::Immediate));
    
    // Unimplemented
    m.insert(0x9e, OpCode::new(0x9e, "SHX", 3, AddressingMode::Absolute_Y));
    
    m.insert(0x9c, OpCode::new(0x9c, "SHY", 3, AddressingMode::Absolute_Y));
    
    // NOPs
    m.insert(0x1a, OpCode::new(0x1a, "NOP", 1, AddressingMode::NoneAddressing));
    m.insert(0x3a, OpCode::new(0x3a, "NOP", 1, AddressingMode::NoneAddressing));
    m.insert(0x5a, OpCode::new(0x5a, "NOP", 1, AddressingMode::NoneAddressing));
    m.insert(0x7a, OpCode::new(0x7a, "NOP", 1, AddressingMode::NoneAddressing));
    m.insert(0xda, OpCode::new(0xda, "NOP", 1, AddressingMode::NoneAddressing));
    m.insert(0xfa, OpCode::new(0xfa, "NOP", 1, AddressingMode::NoneAddressing));
    
    m.insert(0x80, OpCode::new(0x80, "SKB", 2, AddressingMode::Immediate));
    m.insert(0x82, OpCode::new(0x82, "SKB", 2, AddressingMode::Immediate));
    m.insert(0x89, OpCode::new(0x89, "SKB", 2, AddressingMode::Immediate));
    m.insert(0xc2, OpCode::new(0xc2, "SKB", 2, AddressingMode::Immediate));
    m.insert(0xe2, OpCode::new(0xe2, "SKB", 2, AddressingMode::Immediate));

    m.insert(0x0c, OpCode::new(0x0c, "IGN", 3, AddressingMode::Absolute));
    m.insert(0x1c, OpCode::new(0x1c, "IGN", 3, AddressingMode::Absolute_X));
    m.insert(0x3c, OpCode::new(0x3c, "IGN", 3, AddressingMode::Absolute_X));
    m.insert(0x5c, OpCode::new(0x5c, "IGN", 3, AddressingMode::Absolute_X));
    m.insert(0x7c, OpCode::new(0x7c, "IGN", 3, AddressingMode::Absolute_X));
    m.insert(0xdc, OpCode::new(0xdc, "IGN", 3, AddressingMode::Absolute_X));
    m.insert(0xfc, OpCode::new(0xfc, "IGN", 3, AddressingMode::Absolute_X));
    m.insert(0x04, OpCode::new(0x04, "IGN", 2, AddressingMode::ZeroPage));
    m.insert(0x44, OpCode::new(0x44, "IGN", 2, AddressingMode::ZeroPage));
    m.insert(0x64, OpCode::new(0x64, "IGN", 2, AddressingMode::ZeroPage));
    m.insert(0x14, OpCode::new(0x14, "IGN", 2, AddressingMode::ZeroPage_X));
    m.insert(0x34, OpCode::new(0x34, "IGN", 2, AddressingMode::ZeroPage_X));
    m.insert(0x54, OpCode::new(0x54, "IGN", 2, AddressingMode::ZeroPage_X));
    m.insert(0x74, OpCode::new(0x74, "IGN", 2, AddressingMode::ZeroPage_X));
    m.insert(0xd4, OpCode::new(0xd4, "IGN", 2, AddressingMode::ZeroPage_X));
    m.insert(0xf4, OpCode::new(0xf4, "IGN", 2, AddressingMode::ZeroPage_X));



    // m.insert(0x, OpCode::new(0x, "", , AddressingMode::));

    return m
};
}