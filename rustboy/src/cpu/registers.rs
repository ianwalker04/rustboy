const ZERO_FLAG_BYTE_POS: u8 = 7;
const SUBTRACT_FLAG_BYTE_POS: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POS: u8 = 5;
const CARRY_FLAG_BYTE_POS: u8 = 4;

pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagsRegister,
    h: u8,
    l: u8
}

pub struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool
}

enum Instruction {
    ADD(ArithmeticTarget)
}
  
enum ArithmeticTarget {
    A, B, C, D, E, H, L
}
  

impl Registers {

    fn get_af(&self) -> u16 {
        (self.a as u16) << 8
        | self.f as u16
    }

    fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = (value & 0xFF) as u8;
    }

    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8
        | self.c as u16
    }

    fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    fn get_de(&self) -> u16 {
        (self.d as u16) << 8
        | self.e as u16
    }

    fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    fn get_hl(&self) -> u16 {
        (self.h as u16) << 8
        | self.l as u16
    }

    fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }

    impl std::convert::From<FlagsRegister> for u8  {
        fn from(flag: FlagsRegister) -> u8 {
            (if flag.zero       { 1 } else { 0 }) << ZERO_FLAG_BYTE_POS |
            (if flag.subtract   { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POS |
            (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POS |
            (if flag.carry      { 1 } else { 0 }) << CARRY_FLAG_BYTE_POS
        }
    }
    
    impl std::convert::From<u8> for FlagsRegister {
        fn from(byte: u8) -> Self {
            let zero = ((byte >> ZERO_FLAG_BYTE_POS) & 0b1) != 0;
            let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POS) & 0b1) != 0;
            let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POS) & 0b1) != 0;
            let carry = ((byte >> CARRY_FLAG_BYTE_POS) & 0b1) != 0;
    
            FlagsRegister {
                zero,
                subtract,
                half_carry,
                carry
            }
        }
    }

}
