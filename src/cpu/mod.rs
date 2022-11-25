use self::{
    instructions::{ArithmeticTarget, GroupedArithmeticTarget, Instruction},
    registers::Registers,
};

pub mod emulator;
mod flags;
mod instructions;
mod registers;

struct CPU {
    registers: Registers,
}

impl CPU {
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(register) => {
                let value = match register {
                    ArithmeticTarget::A => self.registers.a,
                    ArithmeticTarget::B => self.registers.b,
                    ArithmeticTarget::C => self.registers.c,
                    ArithmeticTarget::D => self.registers.d,
                    ArithmeticTarget::E => self.registers.e,
                    ArithmeticTarget::H => self.registers.l,
                    ArithmeticTarget::L => self.registers.h,
                };

                let res = self.add(value);
                self.registers.a = res;
            }
            Instruction::ADDHL(grouped_register) => {
                let value = match grouped_register {
                    GroupedArithmeticTarget::BC => self.registers.get_bc(),
                    GroupedArithmeticTarget::DE => self.registers.get_de(),
                    GroupedArithmeticTarget::HL => self.registers.get_hl(),
                };

                let res = self.addhl(value);
                self.registers.set_hl(res);
            }
	    Instruction::ADC(register) => {
                let value = match register {
                    ArithmeticTarget::A => self.registers.a,
                    ArithmeticTarget::B => self.registers.b,
                    ArithmeticTarget::C => self.registers.c,
                    ArithmeticTarget::D => self.registers.d,
                    ArithmeticTarget::E => self.registers.e,
                    ArithmeticTarget::H => self.registers.l,
                    ArithmeticTarget::L => self.registers.h,
                };

		let res = self.add_with_carry(value);
		self.registers.a = res;
	    }
	    Instruction::SUB(register) => {
                let value = match register {
                    ArithmeticTarget::A => self.registers.a,
                    ArithmeticTarget::B => self.registers.b,
                    ArithmeticTarget::C => self.registers.c,
                    ArithmeticTarget::D => self.registers.d,
                    ArithmeticTarget::E => self.registers.e,
                    ArithmeticTarget::H => self.registers.l,
                    ArithmeticTarget::L => self.registers.h,
                };

		let res = self.sub(value);
		self.registers.a = res;
	    }
        }
    }
}

impl CPU {
    fn add(&mut self, value: u8) -> u8 {
        let (res, did_overflow) = self.registers.a.overflowing_add(value);

        // Flag handling
        self.registers.f.zero = res == 0;
        self.registers.f.subtraction = false;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        self.registers.f.carry = did_overflow;

        res
    }

    // Half-carry occurs from bit 11 to bit 12
    fn addhl(&mut self, value: u16) -> u16 {
        let mask = 0b111_1111_1111;
        let hl = self.registers.get_hl();
        let (res, did_overflow) = hl.overflowing_add(value);

        // Flag handling
        self.registers.f.subtraction = false;
        self.registers.f.half_carry = (hl & mask) + (value & mask) > mask;
        self.registers.f.carry = did_overflow;

        res
    }

    fn add_with_carry(&mut self, value: u8) -> u8 {
	let (carry_res, did_carry_overflow) = self.registers.a.overflowing_add(self.registers.f.carry as u8);
	let (res, did_overflow) = carry_res.overflowing_add(value);

	self.registers.f.zero = res == 0;
	self.registers.f.subtraction = false;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) + (self.registers.f.carry as u8) > 0xF;
	self.registers.f.carry = did_carry_overflow || did_overflow;

	res
    }

    fn sub(&mut self, value: u8) -> u8 {
        let (res, did_overflow) = self.registers.a.overflowing_sub(value);

        // Flag handling
        self.registers.f.zero = res == 0;
        self.registers.f.subtraction = true;
        self.registers.f.half_carry = (((self.registers.a & 0xF) as i8 - (value & 0xF) as i8) & 0x10) != 0;
        self.registers.f.carry = did_overflow;

        res
    }

    fn sub_with_carry(&mut self, value: u8) -> u8 {
	let (carry_res, did_carry_overflow) = self.registers.a.overflowing_sub(self.registers.f.carry as u8);
	let (res, did_overflow) = carry_res.overflowing_sub(value);

	self.registers.f.zero = res == 0;
	self.registers.f.subtraction = true;
        self.registers.f.half_carry = (((self.registers.a & 0xF) as i8 - (value & 0xF) as i8 - self.registers.f.carry as i8) & 0x10) != 0;
	self.registers.f.carry = did_carry_overflow || did_overflow;

	res
    }
}
