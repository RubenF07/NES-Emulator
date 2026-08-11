use crate::cpu::{AddressingMode, CPU, Mem};
use crate::opcodes::OPS_CODE_MAP;


pub fn trace(cpu: &CPU) -> String{
    let opcode = OPS_CODE_MAP.get(&cpu.mem_read(cpu.program_counter)).expect(&format!("Failed to read opcode: {:02x}",cpu.mem_read(cpu.program_counter)));
    let bytes = opcode.bytes;

    // println!("{:?}",opcode.mnemonic);

    let (byte0, byte1, byte2) = (cpu.mem_read(cpu.program_counter),cpu.mem_read(cpu.program_counter.wrapping_add(1)),cpu.mem_read(cpu.program_counter.wrapping_add(2)));

    let opc_str = match bytes{
        1 => format!("{:02x}", byte0),
        2 => format!("{:02x} {:02x}", byte0, byte1),
        3 => format!("{:02x} {:02x} {:02x}", byte0, byte1, byte2),
        _ => panic!()
    };

    let readable_parsed = match opcode.bytes {
        1 => {
            match opcode.mode{
                AddressingMode::NoneAddressing => "",
                AddressingMode::Accumulator => "A",
                _ => panic!()
            }
        }
        2 => {
            let base_addr = byte1;
            let mem_addr = if opcode.mode != AddressingMode::NoneAddressing{
                cpu.get_op_addr_offset(&opcode.mode, 1).0
            }
            else{
                (cpu.program_counter + 2).wrapping_add((base_addr as i8) as u16)
            };
            let val = cpu.mem_read(mem_addr);
            
            match opcode.mode{
                AddressingMode::Immediate => &format!("#${:02x}", base_addr),
                AddressingMode::ZeroPage => &format!("${:02x} = {:02x}", mem_addr, val),
                AddressingMode::ZeroPage_X =>  &format!("${:02x},X @ {:02x} = {:02x}", base_addr, mem_addr, val),
                AddressingMode::ZeroPage_Y =>  &format!("${:02x},Y @ {:02x} = {:02x}", base_addr, mem_addr, val),
                AddressingMode::Indirect_X =>  &format!("(${:02x},X) @ {:02x} = {:04x} = {:02x}", base_addr, base_addr.wrapping_add(cpu.register_x), mem_addr, val),
                AddressingMode::Indirect_Y =>  {
                    let lo = cpu.mem_read(base_addr as u16);
                    let hi = cpu.mem_read(base_addr.wrapping_add(1) as u16);
                    let deref_base = (hi as u16) << 8 | (lo as u16);
                    &format!("(${:02x}),Y = {:04x} @ {:04x} = {:02x}", base_addr, deref_base, mem_addr, val)
                },
                AddressingMode::NoneAddressing => &format!("${:04x}", mem_addr), // Branch op codes
                _ => panic!()
            }
        }
        3 => {
            let base_addr = ((byte2 as u16) << 8) | byte1 as u16;
            let mem_addr = if opcode.mode != AddressingMode::NoneAddressing{
                cpu.get_op_addr_offset(&opcode.mode, 1).0
            }
            else{
                if base_addr & 0x00ff == 0x00ff{
                    let lo = cpu.mem_read(base_addr);
                    let hi = cpu.mem_read(base_addr & 0xff00);
                    (hi as u16) << 8 | (lo as u16)
                }
                else{
                    cpu.mem_read_u16(base_addr)
                }
            };
            let val = cpu.mem_read(mem_addr);

            if opcode.hex == 0x4c || opcode.hex == 0x20{
                &format!("${:04x}",base_addr)
            }
            else{
                match opcode.mode{
                    AddressingMode::Absolute => &format!("${:04x} = {:02x}", base_addr, val),
                    AddressingMode::Absolute_X => &format!("${:04x},X @ {:04x} = {:02x}", base_addr, mem_addr, val),
                    AddressingMode::Absolute_Y => &format!("${:04x},Y @ {:04x} = {:02x}", base_addr, mem_addr, val),
                    AddressingMode::NoneAddressing => {
                        if opcode.hex == 0x6c{ // JMP indirect
                            &format!("(${:04x}) = {:04x}", base_addr, mem_addr)
                        }
                        else{
                            // TODO
                            panic!("IS THIS NECESSARY?")
                        }
                        
                        
                    },
                    _ => panic!()
                }
            }
        }
        _ => ""
    };

    let trace_mnemonic = if opcode.mnemonic == "IGN" || opcode.mnemonic == "SKB"{
        "NOP"
    }
    else if opcode.mnemonic == "ISC"{
        "ISB"
    }
    else{
        opcode.mnemonic
    };

    let asm_str = format!("{:04x} {:8}  {} {}", cpu.program_counter, opc_str, trace_mnemonic, readable_parsed);

    let register_str = format!("A:{:02x} X:{:02x} Y:{:02x} P:{:02x}", 
        cpu.register_a, cpu.register_x, cpu.register_y, cpu.status);
    
    let trace_str = format!("{:47} {}", asm_str, register_str).to_ascii_uppercase();

    return trace_str
}