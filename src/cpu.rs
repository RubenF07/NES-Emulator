use crate::opcodes::OPS_CODE_MAP;

bitflags! {
    /// # Status Register (P) https://www.nesdev.org/wiki/Status_flags
    ///
    ///  7 6 5 4 3 2 1 0
    ///  N V _ B D I Z C
    ///  | |   | | | | +--- Carry Flag
    ///  | |   | | | +----- Zero Flag
    ///  | |   | | +------- Interrupt Disable
    ///  | |   | +--------- Decimal Mode (not used on NES)
    ///  | |   +----------- Break Command
    ///  | +--------------- Overflow Flag
    ///  +----------------- Negative Flag
    
    #[derive(Clone)]
    pub struct CpuFlags: u8 {
        const CARRY             = 0b0000_0001;
        const ZERO              = 0b0000_0010;
        const INTERRUPT_DISABLE = 0b0000_0100;
        const DECIMAL_MODE      = 0b0000_1000;
        const BREAK             = 0b0001_0000;
        const BREAK2            = 0b0010_0000;
        const OVERFLOW          = 0b0100_0000;
        const NEGATIVE          = 0b1000_0000;
    }
}

const STACK: u16 = 0x0100;
const STACK_RESET: u8 = 0xfd;

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: CpuFlags,
    pub program_counter: u16,
    pub stack_pointer: u8,
    memory: [u8; 0xFFFF]
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    NoneAddresssing,
}



impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: CpuFlags::from_bits_truncate(0b100100),
            program_counter: 0,
            stack_pointer: STACK_RESET,
            memory: [0; 0xFFFF]
        }
    }

    // ------------------------------------
    // Operations - https://www.nesdev.org/obelisk-6502-guide/reference.html#PHP
    // ------------------------------------

    

    // Add with carry
    fn adc(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let mem_val = self.mem_read(addr);
        
        self.add_to_register_a(mem_val);
    }

    // Logical AND
    fn and(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let val = self.mem_read(addr);

        self.set_register_a(self.register_a & val);
    }

    // Arethmatic shift left
    fn asl_accumulator(&mut self){
        self.status.set(CpuFlags::CARRY, self.register_a >> 7 == 1);

        self.set_register_a(self.register_a << 1);
    }
    fn asl(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let val = self.mem_read(addr);

        self.status.set(CpuFlags::CARRY, val >> 7 == 1);

        let res = val << 1;
        self.mem_write(addr, res);
        
        self.status.set(CpuFlags::NEGATIVE, res & 0b1000_0000 != 0);
    }
    
    // Bit test
    fn bit(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let val = self.mem_read(addr);
        
        let res = self.register_a & val;
        
        self.status.set(CpuFlags::ZERO, res == 0);
        
        self.status.set(CpuFlags::NEGATIVE, val & 0b1000_0000 > 0);
        self.status.set(CpuFlags::OVERFLOW, val & 0b0100_0000 > 0);
    }
    
    // Decrement memory
    fn dec(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let val = self.mem_read(addr);
        
        let res = val.wrapping_sub(1);
        self.mem_write(addr, res);
        self.update_zero_and_negative_flags(res);
    }
    
    // Decrement X register
    fn dex(&mut self){
        self.register_x= self.register_x.wrapping_sub(1);
        
        self.update_zero_and_negative_flags(self.register_x);
    }
    
    // Decrement Y register
    fn dey(&mut self){
        self.register_y= self.register_y.wrapping_sub(1);

        self.update_zero_and_negative_flags(self.register_y);
    }
    
    // Exclusive OR
    fn eor(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let val = self.mem_read(addr);
        
        self.set_register_a(self.register_a ^ val);
    }
    
    // Increment memort
    fn inc(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let val = self.mem_read(addr);
        
        let res = val.wrapping_add(1);
        self.mem_write(addr, res);
        
        self.update_zero_and_negative_flags(res);  
    }
    
    // Increment register X
    fn inx(&mut self){
        if self.register_x == u8::MAX{
            self.register_x = 0
        }
        else{
            self.register_x += 1;
        }
        self.update_zero_and_negative_flags(self.register_x)
    }
    
    // Increment register Y
    fn iny(&mut self){
        if self.register_y == u8::MAX{
            self.register_y = 0
        }
        else{
            self.register_y += 1;
        }
        self.update_zero_and_negative_flags(self.register_y)
    }
    
    // Jump
    fn jmp_absolute(&mut self){
        let mem_addr = self.mem_read_u16(self.program_counter);
        
        self.program_counter = mem_addr;
    }
    fn jmp_indirect(&mut self){
        let mem_addr = self.mem_read_u16(self.program_counter);
        
        // let indirect_ref = self.mem_read_u16(mem_addr);
        //6502 bug mode with with page boundary:
        //  if address $3000 contains $40, $30FF contains $80, and $3100 contains $50,
        // the result of JMP ($30FF) will be a transfer of control to $4080 rather than $5080 as you intended
        // i.e. the 6502 took the low byte of the address from $30FF and the high byte from $3000
        
        let indirect_ref = if mem_addr & 0x00FF == 0x00FF {
            let lo = self.mem_read(mem_addr);
            let hi = self.mem_read(mem_addr & 0xFF00);
            (hi as u16) << 8 | (lo as u16)
        } else {
            self.mem_read_u16(mem_addr)
        };
        
        self.program_counter = indirect_ref;
    }
    
    // Jump to subroutine
    fn jsr(&mut self){
        self.stack_push_u16(self.program_counter + 2 - 1);
        let target_addr = self.mem_read_u16(self.program_counter);
        
        self.program_counter = target_addr
    }
    
    // Load val into register A
    fn lda(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let val = self.mem_read(addr);
        
        self.set_register_a(val);
    }
    
    // Load val into register X
    fn ldx(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let val = self.mem_read(addr);
        
        self.set_register_x(val);
    }
    
    // Load val into register Y
    fn ldy(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let val = self.mem_read(addr);
        
        self.set_register_y(val);
    }
    
    // Logical shift right
    fn lsr_accumulator(&mut self){
        self.status.set(CpuFlags::CARRY, self.register_a & 1 == 1);
        
        self.set_register_a(self.register_a >> 1);
    }
    fn lsr(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let val = self.mem_read(addr);
        
        self.status.set(CpuFlags::CARRY, val & 1 == 1);
        
        let res = val >> 1;
        self.mem_write(addr, res);
        
        self.update_zero_and_negative_flags(res);
    }
    
    // Logical inclusive or
    fn ora(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let val = self.mem_read(addr);
        
        self.set_register_a(self.register_a | val);
    }

    // Push processor status
    fn php(&mut self){
        let mut flags = self.status.clone();
        
        flags.insert(CpuFlags::BREAK);
        flags.insert(CpuFlags::BREAK2);
        self.stack_push(flags.bits());
    }
    
    // Pull accumulator
    fn pla(&mut self){
        let val = self.stack_pop();
        
        self.set_register_a(val);
    }
    
    // Pull processor status
    fn plp(&mut self){
        self.status = CpuFlags::from_bits_retain(self.stack_pop());
        
        self.status.remove(CpuFlags::BREAK);
        self.status.remove(CpuFlags::BREAK2);
    }
    
    // Rotate left
    fn rol_accumulator(&mut self){
        self.status.set(CpuFlags::CARRY, self.register_a >> 7 == 1);
        
        self.set_register_a((self.register_a << 1) | (self.status.contains(CpuFlags::CARRY) as u8));
    }
    fn rol(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let val = self.mem_read(addr);
        let old_carry = self.status.contains(CpuFlags::CARRY) as u8;
        
        self.status.set(CpuFlags::CARRY, val >> 7 == 1);

        let res = (val << 1) | old_carry;
        self.mem_write(addr, res);
        
        self.status.set(CpuFlags::NEGATIVE, res & 0b1000_0000 != 0);
    }
    
    // Rotate right
    fn ror_accumulator(&mut self){
        self.status.set(CpuFlags::CARRY, self.register_a & 1 == 1);
        
        let carry = (self.status.contains(CpuFlags::CARRY) as u8) << 7;
        
        self.set_register_a((self.register_a >> 1) + carry);
    }
    fn ror(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let val = self.mem_read(addr);
        
        self.status.set(CpuFlags::CARRY, val & 1 == 1);
        
        let carry = (self.status.contains(CpuFlags::CARRY) as u8) << 7;
        let res = (val >> 1) + carry;
        self.mem_write(addr, res);
        
        self.status.set(CpuFlags::NEGATIVE, res & 0b1000_0000 != 0);
    }
    
    // Return from subroutine
    fn rts(&mut self){
        let new_counter = self.stack_pop_u16() + 1;
        
        self.program_counter = new_counter;
    }

    // Subtract with carry
    // A - B = A + (-B) | -B = !B + 1
    fn sbc(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        let val = self.mem_read(addr);

        let b_neg = (val as i8).wrapping_neg().wrapping_sub(1);

        self.add_to_register_a(b_neg as u8);
    }
    
    // Store register A value at mem address
    fn sta(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_a);
    }    
    
    // Store register X value at mem address
    fn stx(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_x);
    }    
    
    // Store register Y value at mem address
    fn sty(&mut self, mode: &AddressingMode){
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_y);
    }    
    
    // Transfer(copy) register A value into register X
    fn tax(&mut self){
        self.set_register_x(self.register_a);
    }
    
    // Transfer(copy) register A value into register Y
    fn tay(&mut self){
        self.set_register_y(self.register_a);
    }

    // Transfer stack pointer to X
    fn tsx(&mut self){
        self.set_register_x(self.stack_pointer);
    }

    // ------------------------------------
    // Helpers
    // ------------------------------------
    
    fn update_zero_and_negative_flags(&mut self, result: u8){
        if result == 0 {
            self.status.insert(CpuFlags::ZERO);
        } else {
            self.status.remove(CpuFlags::ZERO);
        }

        if result & 0b1000_0000 != 0 {
            self.status.insert(CpuFlags::NEGATIVE);
        } else {
            self.status.remove(CpuFlags::NEGATIVE);
        }
    }

    fn set_register_a(&mut self, val: u8){
        self.register_a = val;
        self.update_zero_and_negative_flags(self.register_a)
    }
    
    fn set_register_x(&mut self, val: u8){
        self.register_x = val;
        self.update_zero_and_negative_flags(self.register_x)
    }
    
    fn set_register_y(&mut self, val: u8){
        self.register_y = val;
        self.update_zero_and_negative_flags(self.register_y)
    }

    /// http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
    fn add_to_register_a(&mut self, data: u8) {
        let sum = self.register_a as u16
            + data as u16
            + (if self.status.contains(CpuFlags::CARRY) {
                1
            } else {
                0
            }) as u16;

        self.status.set(CpuFlags::CARRY, sum > 0xff);

        let result = sum as u8;

        if (data ^ result) & (result ^ self.register_a) & 0x80 != 0 {
            self.status.insert(CpuFlags::OVERFLOW);
        } else {
            self.status.remove(CpuFlags::OVERFLOW)
        }

        self.set_register_a(result);
    }

    // Branch if carry clear
    fn branch(&mut self, condition:bool){
        if condition{
            let jump: i8 = self.mem_read(self.program_counter) as i8;
            let jump_addr = self.program_counter.wrapping_add(1).wrapping_add(jump as u16);

            self.program_counter = jump_addr;
        }
    }

    // Compare values
    fn compare(&mut self, mode: &AddressingMode, compare_with: u8){
        let addr = self.get_operand_address(mode);
        let val = self.mem_read(addr);

        self.status.set(CpuFlags::CARRY, compare_with >= val);

        self.update_zero_and_negative_flags(compare_with.wrapping_sub(val));
    }

    fn stack_pop(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        return self.mem_read(STACK + (self.stack_pointer as u16))
    }
    
    fn stack_push(&mut self, val: u8){
        self.mem_write(STACK + (self.stack_pointer as u16), val);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
    }

    fn stack_push_u16(&mut self, val: u16){
        let hi = (val >> 8) as u8;
        let lo = (val & 0x00ff) as u8;

        self.stack_push(hi);
        self.stack_push(lo);
    }

    fn stack_pop_u16(&mut self) -> u16{
        let hi = self.stack_pop() as u16;
        let lo = self.stack_pop() as u16;

        return (hi << 8) | lo;
    }



    // ------------------------------------
    // Memory
    // ------------------------------------
    
    fn mem_read(&mut self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }
    
    // read Little-Endian 16bit vals
    fn mem_read_u16(&mut self, pos: u16) -> u16 {
        let lo = self.mem_read(pos) as u16;
        let hi = self.mem_read(pos + 1) as u16;
        return (hi << 8) | (lo as u16)
    }
    
    // read Little-Endian 16bit vals
    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        
        self.mem_write(pos, lo);
        self.mem_write(pos + 1, hi);
    }
    
    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }
    
    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.status = CpuFlags::from_bits_truncate(0b100100);
        
        self.program_counter = self.mem_read_u16(0xFFFC);
    }
    
    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }
    
    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }
    
    pub fn run(&mut self) {
        loop {
            let opscode = self.mem_read(self.program_counter);
            self.program_counter += 1;

            let ops_spec = OPS_CODE_MAP.get(&opscode).expect(("Invalid OPS Code: ".to_string()+&opscode.to_string()).as_str());
            let mode = &ops_spec.mode;
            let count_inc = ops_spec.bytes as u16;

            // println!("code: {:?}, mode: {:?}, bytes: {:?}");
            
            match opscode{
                // ADC
                0x69 | 0x65 | 0x75 | 0x6d | 0x7d | 0x79 | 0x61 | 0x71 => self.adc(mode),

                // AND
                0x29 | 0x25 | 0x35 | 0x2d | 0x3d | 0x39 | 0x21 | 0x31 => self.and(mode),

                // ASL
                0x0a => self.asl_accumulator(),
                0x06 | 0x16 | 0x0e | 0x1e => self.asl(mode),

                // BCC
                0x90 => self.branch(!self.status.contains(CpuFlags::CARRY)),
                
                // BCS
                0xb0 => self.branch(self.status.contains(CpuFlags::CARRY)),
                
                // BEQ
                0xf0 => self.branch(self.status.contains(CpuFlags::ZERO)),

                // BIT
                0x24 | 0x2c => self.bit(mode),

                // BMI
                0x30 => self.branch(self.status.contains(CpuFlags::NEGATIVE)),

                // BNE
                0xd0 => self.branch(!self.status.contains(CpuFlags::ZERO)),

                // BPL
                0x10 => self.branch(!self.status.contains(CpuFlags::NEGATIVE)),
                
                // BRK
                0x00 => return,
                
                // BVC
                0x50 => self.branch(!self.status.contains(CpuFlags::OVERFLOW)),
                
                // BVS
                0x70 => self.branch(self.status.contains(CpuFlags::OVERFLOW)),

                // CLC
                0x18 => self.status.remove(CpuFlags::CARRY),
                
                // CLD - skipped
                
                // CLI
                0x58 => self.status.remove(CpuFlags::INTERRUPT_DISABLE),
                
                // CLV
                0xb8 => self.status.remove(CpuFlags::OVERFLOW),

                // CMP
                0xc9 | 0xc5 | 0xd5 | 0xcd | 0xdd | 0xd9 | 0xc1 | 0xd1 => self.compare(mode, self.register_a),

                // CPX
                0xe0 | 0xe4 | 0xec => self.compare(mode, self.register_x),
                
                // CPY
                0xc0 | 0xc4 | 0xcc => self.compare(mode, self.register_y),

                // DEC
                0xc6 | 0xd6 | 0xce | 0xde => self.dec(mode),

                // DEX
                0xca => self.dex(),
                
                // DEY
                0x88 => self.dey(),

                // EOR
                0x49 | 0x45 | 0x55 | 0x4d | 0x5d | 0x59 | 0x41 | 0x51 => self.eor(mode),

                // INC
                0xe6 | 0xf6 | 0xee | 0xfe => self.inc(mode),

                // INX
                0xe8 => self.inx(),
                
                // INY
                0xc8 => self.iny(),

                // JMP
                0x4c => self.jmp_absolute(),
                0x6c => self.jmp_indirect(),

                // JSR
                0x20 => self.jsr(),
                
                // LDA
                0xa9 | 0xa5 | 0xb5 | 0xad | 0xbd | 0xb9 | 0xa1 | 0xb1  => self.lda(mode),

                // LDX
                0xa2 | 0xa6 | 0xb6 | 0xae | 0xbe => self.ldx(mode),
                
                // LDY
                0xa0 | 0xa4 | 0xb4 | 0xac | 0xbc => self.ldy(mode),

                // LSR
                0x4a => self.lsr_accumulator(),
                0x46 | 0x56 | 0x4e | 0x5e => self.lsr(mode),

                // NOP
                0xea => {}

                // ORA
                0x09 | 0x05 | 0x15 | 0x0d | 0x1d | 0x19 | 0x01 | 0x11 => self.ora(mode),

                // PHA
                0x48 => self.stack_push(self.register_a),

                // PHP
                0x08 => self.php(),

                // PLA
                0x68 => self.pla(),

                // PLP
                0x28 => self.plp(),

                // ROL
                0x2a => self.rol_accumulator(),
                0x26 | 0x36 | 0x2e | 0x3e => self.rol(mode),
                
                // ROR
                0x6a => self.ror_accumulator(),
                0x66 | 0x76 | 0x6e | 0x7e => self.ror(mode),

                // RTS
                0x60 => self.rts(),

                // SBC
                0xe9 | 0xe5 | 0xf5 | 0xed | 0xfd | 0xf9 | 0xe1 | 0xf1 => self.sbc(mode),

                // SEC
                0x38 => self.status.insert(CpuFlags::CARRY),

                // SED - skipped

                // SEI
                0x78 => self.status.insert(CpuFlags::INTERRUPT_DISABLE),

                // STA
                0x85 | 0x95 | 0x8d | 0x9d | 0x99 | 0x81 | 0x91 => self.sta(mode),

                // STX
                0x86 | 0x96 | 0x8e => self.stx(mode),

                // STY
                0x84 | 0x94 | 0x8c => self.sty(mode),

                // TAX
                0xaa => self.tax(),
                
                // TAY
                0xa8 => self.tay(),

                // TSX
                0xba => self.tsx(),

                // TXA
                0x8a => self.set_register_a(self.register_x),

                // TXS
                0x9a => self.stack_pointer = self.register_x,

                // TYA
                0x98 => self.set_register_a(self.register_y), 
                
                _ => todo!()
            }
            self.program_counter += count_inc - 1;
        }
    }
    
    // ------------------------------------
    // Adressing
    // ------------------------------------

    fn get_operand_address(&mut self, mode: &AddressingMode) -> u16 {
        match mode {
            // [OPS] #$01
            AddressingMode::Immediate => self.program_counter,

            // [OPS] $01
            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,

            // [OPS] $0100
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
    
            // [OPS] $01, X
            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_x) as u16;
                return addr
            },
            
            // [OPS] $01, Y
            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_y) as u16;
                return addr
            },
            
            // [OPS] $0100, X
            // [OPS] $[0100 + X_value]
            AddressingMode::Absolute_X => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_x as u16);
                return addr
            },
            
            // [OPS] $0100, Y
            // [OPS] $[0100 + Y_value]
            AddressingMode::Absolute_Y => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_y as u16);
                return addr
            },

            // [OPS] ($40, X)
            // [OPS] $[$[40 + X_value]: u16]
            AddressingMode::Indirect_X => {
                let base = self.mem_read(self.program_counter);

                let ptr: u8 = (base as u8).wrapping_add(self.register_x);
                let lo = self.mem_read(ptr as u16);
                let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                return (hi as u16) << 8 | (lo as u16)
            },

            // [OPS] ($40), Y
            // [OPS] $[$40:u16 + Y_value]
            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.program_counter);

                let lo = self.mem_read(base as u16);
                let hi = self.mem_read(base.wrapping_add(1) as u16);
                let deref_base = (hi as u16) << 8 | (lo as u16);
                let deref = deref_base.wrapping_add(self.register_y as u16);
                return deref
            },

            AddressingMode::NoneAddresssing => {
                panic!("mode {:?} is not supported", mode);
            }
            
        }
    }
    
}




#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(cpu.status.bits() & 0b0000_0010 == 0b00);
        assert!(cpu.status.bits() & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag(){
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9,0x00,0x00]);
        assert!(cpu.status.bits() & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_zero_a_to_x(){
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 10, 0xaa, 0x00]);
        
        assert_eq!(cpu.register_x, 10)
    }
    
    #[test]
    fn test_inx(){
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 2)        
    }
    
    #[test]
    fn test_inx_overflow(){
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)        
    }
    
    #[test]
   fn test_5_ops_working_together() {
       let mut cpu = CPU::new();
       cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
 
       assert_eq!(cpu.register_x, 0xc1)
   }

   #[test]
   fn test_lda_from_memory() {
       let mut cpu = CPU::new();
       cpu.mem_write(0x10, 0x55);

       cpu.load_and_run(vec![0xa5, 0x10, 0x00]);

       assert_eq!(cpu.register_a, 0x55);
   }

}