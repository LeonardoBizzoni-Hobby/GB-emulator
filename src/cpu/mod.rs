use self::{
    instructions::{
        ArithmeticTarget, BitPosition, GroupedArithmeticTarget, IncDecTarget, Instruction, JumpCondition, LoadType, LoadByteSrc, LoadByteTarget, LoadWordTarget, IndirectSrc,
    },
    memory::MemoryBus,
    registers::Registers,
};

pub mod emulator;
mod flags;
mod instructions;
mod memory;
mod registers;

#[allow(dead_code)]
struct CPU {
    registers: Registers,
    pc: u16,
    bus: MemoryBus,
}

#[allow(dead_code)]
impl CPU {
    fn step(&mut self) {
        let mut instruction_address = self.bus.read_byte(self.pc);
	let is_prefix = instruction_address == 0xCB;

	if is_prefix {
	    instruction_address = self.bus.read_byte(self.pc+1);
	}

        self.pc = if let Some(instruction) = Instruction::from_byte(instruction_address, is_prefix) {
            self.execute(instruction)
        } else {
            panic!("Invalid instruction found at: 0x{:x}", instruction_address);
        };
    }

    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::ADD(register) => {
                let value = match register {
                    ArithmeticTarget::A => self.registers.a,
                    ArithmeticTarget::B => self.registers.b,
                    ArithmeticTarget::C => self.registers.c,
                    ArithmeticTarget::D => self.registers.d,
                    ArithmeticTarget::E => self.registers.e,
                    ArithmeticTarget::H => self.registers.h,
                    ArithmeticTarget::L => self.registers.l,
                };

                self.registers.a = self.add(value);
		self.pc.wrapping_add(1)
            }
            Instruction::ADDHL(grouped_register) => {
                let value = match grouped_register {
                    GroupedArithmeticTarget::BC => self.registers.get_bc(),
                    GroupedArithmeticTarget::DE => self.registers.get_de(),
                    GroupedArithmeticTarget::HL => self.registers.get_hl(),
                };

                let res = self.addhl(value);
                self.registers.set_hl(res);
		self.pc.wrapping_add(1)
            }
            Instruction::ADC(register) => {
                let value = match register {
                    ArithmeticTarget::A => self.registers.a,
                    ArithmeticTarget::B => self.registers.b,
                    ArithmeticTarget::C => self.registers.c,
                    ArithmeticTarget::D => self.registers.d,
                    ArithmeticTarget::E => self.registers.e,
                    ArithmeticTarget::H => self.registers.h,
                    ArithmeticTarget::L => self.registers.l,
                };

                self.registers.a = self.add_with_carry(value);
		self.pc.wrapping_add(1)
            }
            Instruction::SUB(register) => {
                let value = match register {
                    ArithmeticTarget::A => self.registers.a,
                    ArithmeticTarget::B => self.registers.b,
                    ArithmeticTarget::C => self.registers.c,
                    ArithmeticTarget::D => self.registers.d,
                    ArithmeticTarget::E => self.registers.e,
                    ArithmeticTarget::H => self.registers.h,
                    ArithmeticTarget::L => self.registers.l,
                };

                self.registers.a = self.sub(value);
		self.pc.wrapping_add(1)
            }
            Instruction::SBC(register) => {
                let value = match register {
                    ArithmeticTarget::A => self.registers.a,
                    ArithmeticTarget::B => self.registers.b,
                    ArithmeticTarget::C => self.registers.c,
                    ArithmeticTarget::D => self.registers.d,
                    ArithmeticTarget::E => self.registers.e,
                    ArithmeticTarget::H => self.registers.h,
                    ArithmeticTarget::L => self.registers.l,
                };

                self.registers.a = self.sub_with_carry(value);
		self.pc.wrapping_add(1)
            }
            Instruction::AND(register) => {
                let value = match register {
                    ArithmeticTarget::A => self.registers.a,
                    ArithmeticTarget::B => self.registers.b,
                    ArithmeticTarget::C => self.registers.c,
                    ArithmeticTarget::D => self.registers.d,
                    ArithmeticTarget::E => self.registers.e,
                    ArithmeticTarget::H => self.registers.h,
                    ArithmeticTarget::L => self.registers.l,
                };

                self.registers.a = self.and(value);
		self.pc.wrapping_add(1)
            }
            Instruction::OR(register) => {
                let value = match register {
                    ArithmeticTarget::A => self.registers.a,
                    ArithmeticTarget::B => self.registers.b,
                    ArithmeticTarget::C => self.registers.c,
                    ArithmeticTarget::D => self.registers.d,
                    ArithmeticTarget::E => self.registers.e,
                    ArithmeticTarget::H => self.registers.h,
                    ArithmeticTarget::L => self.registers.l,
                };

                self.registers.a = self.or(value);
		self.pc.wrapping_add(1)
            }
            Instruction::XOR(register) => {
                let value = match register {
                    ArithmeticTarget::A => self.registers.a,
                    ArithmeticTarget::B => self.registers.b,
                    ArithmeticTarget::C => self.registers.c,
                    ArithmeticTarget::D => self.registers.d,
                    ArithmeticTarget::E => self.registers.e,
                    ArithmeticTarget::H => self.registers.h,
                    ArithmeticTarget::L => self.registers.l,
                };

                self.registers.a = self.xor(value);
		self.pc.wrapping_add(1)
            }
            Instruction::CP(register) => {
                let value = match register {
                    ArithmeticTarget::A => self.registers.a,
                    ArithmeticTarget::B => self.registers.b,
                    ArithmeticTarget::C => self.registers.c,
                    ArithmeticTarget::D => self.registers.d,
                    ArithmeticTarget::E => self.registers.e,
                    ArithmeticTarget::H => self.registers.h,
                    ArithmeticTarget::L => self.registers.l,
                };

                self.compare(value);
		self.pc.wrapping_add(1)
            }

            Instruction::INC(register) => {
                match register {
                    IncDecTarget::A => self.registers.a = self.inc_8b(self.registers.a),
                    IncDecTarget::B => self.registers.b = self.inc_8b(self.registers.b),
                    IncDecTarget::C => self.registers.c = self.inc_8b(self.registers.c),
                    IncDecTarget::D => self.registers.d = self.inc_8b(self.registers.d),
                    IncDecTarget::E => self.registers.e = self.inc_8b(self.registers.e),
                    IncDecTarget::H => self.registers.h = self.inc_8b(self.registers.h),
                    IncDecTarget::L => self.registers.l = self.inc_8b(self.registers.l),
                    IncDecTarget::BC => {
                        let res = self.inc_16b(self.registers.get_bc());
                        self.registers.set_bc(res);
                    }
                    IncDecTarget::DE => {
                        let res = self.inc_16b(self.registers.get_de());
                        self.registers.set_de(res);
                    }
                    IncDecTarget::HL => {
                        let res = self.inc_16b(self.registers.get_hl());
                        self.registers.set_hl(res);
                    }
                };

		self.pc.wrapping_add(1)
            }
            Instruction::DEC(register) => {
                match register {
                    IncDecTarget::A => self.registers.a = self.dec_8b(self.registers.a),
                    IncDecTarget::B => self.registers.b = self.dec_8b(self.registers.b),
                    IncDecTarget::C => self.registers.c = self.dec_8b(self.registers.c),
                    IncDecTarget::D => self.registers.d = self.dec_8b(self.registers.d),
                    IncDecTarget::E => self.registers.e = self.dec_8b(self.registers.e),
                    IncDecTarget::H => self.registers.h = self.dec_8b(self.registers.h),
                    IncDecTarget::L => self.registers.l = self.dec_8b(self.registers.l),
                    IncDecTarget::BC => {
                        let res = self.dec_16b(self.registers.get_bc());
                        self.registers.set_bc(res);
                    }
                    IncDecTarget::DE => {
                        let res = self.dec_16b(self.registers.get_de());
                        self.registers.set_de(res);
                    }
                    IncDecTarget::HL => {
                        let res = self.dec_16b(self.registers.get_hl());
                        self.registers.set_hl(res);
                    }
                };

		self.pc.wrapping_add(1)
            }
            Instruction::CCF => {
                self.complement_carry();
		self.pc.wrapping_add(1)
            }
            Instruction::SCF => {
                self.set_carry();
		self.pc.wrapping_add(1)
            }
            Instruction::RRA => {
                self.rotate_right_a_with_carry();
		self.pc.wrapping_add(1)
            }
            Instruction::RLA => {
                self.rotate_left_a_with_carry();
		self.pc.wrapping_add(1)
            }
            Instruction::RRCA => {
                self.rotate_right_a();
		self.pc.wrapping_add(1)
            }
            Instruction::RLCA => {
                self.rotate_left_a();
		self.pc.wrapping_add(1)
            }
            Instruction::CPL => {
                self.complement();
		self.pc.wrapping_add(1)
            }
            Instruction::BIT(register, bit) => {
                let value = match register {
                    ArithmeticTarget::A => self.registers.a,
                    ArithmeticTarget::B => self.registers.b,
                    ArithmeticTarget::C => self.registers.c,
                    ArithmeticTarget::D => self.registers.d,
                    ArithmeticTarget::E => self.registers.e,
                    ArithmeticTarget::H => self.registers.h,
                    ArithmeticTarget::L => self.registers.l,
                };

                self.test_bit(value, bit);
		self.pc.wrapping_add(1)
            }
            Instruction::RESET(register, bit) => {
                match register {
                    ArithmeticTarget::A => self.registers.a = self.reset_bit(self.registers.a, bit),
                    ArithmeticTarget::B => self.registers.b = self.reset_bit(self.registers.b, bit),
                    ArithmeticTarget::C => self.registers.c = self.reset_bit(self.registers.c, bit),
                    ArithmeticTarget::D => self.registers.d = self.reset_bit(self.registers.d, bit),
                    ArithmeticTarget::E => self.registers.e = self.reset_bit(self.registers.e, bit),
                    ArithmeticTarget::H => self.registers.h = self.reset_bit(self.registers.h, bit),
                    ArithmeticTarget::L => self.registers.l = self.reset_bit(self.registers.l, bit),
                };

		self.pc.wrapping_add(1)
            }
            Instruction::SET(register, bit) => {
                match register {
                    ArithmeticTarget::A => self.registers.a = self.set_bit(self.registers.a, bit),
                    ArithmeticTarget::B => self.registers.b = self.set_bit(self.registers.b, bit),
                    ArithmeticTarget::C => self.registers.c = self.set_bit(self.registers.c, bit),
                    ArithmeticTarget::D => self.registers.d = self.set_bit(self.registers.d, bit),
                    ArithmeticTarget::E => self.registers.e = self.set_bit(self.registers.e, bit),
                    ArithmeticTarget::H => self.registers.h = self.set_bit(self.registers.h, bit),
                    ArithmeticTarget::L => self.registers.l = self.set_bit(self.registers.l, bit),
                };

		self.pc.wrapping_add(1)
            }
            Instruction::SRL(register) => {
                match register {
                    ArithmeticTarget::A => {
                        self.registers.a = self.shift_right_logical(self.registers.a)
                    }
                    ArithmeticTarget::B => {
                        self.registers.b = self.shift_right_logical(self.registers.b)
                    }
                    ArithmeticTarget::C => {
                        self.registers.c = self.shift_right_logical(self.registers.c)
                    }
                    ArithmeticTarget::D => {
                        self.registers.d = self.shift_right_logical(self.registers.d)
                    }
                    ArithmeticTarget::E => {
                        self.registers.e = self.shift_right_logical(self.registers.e)
                    }
                    ArithmeticTarget::H => {
                        self.registers.h = self.shift_right_logical(self.registers.h)
                    }
                    ArithmeticTarget::L => {
                        self.registers.l = self.shift_right_logical(self.registers.l)
                    }
                };

		self.pc.wrapping_add(1)
            }
            Instruction::RR(register) => {
                match register {
                    ArithmeticTarget::A => {
                        self.registers.a = self.rotate_right_with_carry(self.registers.a)
                    }
                    ArithmeticTarget::B => {
                        self.registers.b = self.rotate_right_with_carry(self.registers.b)
                    }
                    ArithmeticTarget::C => {
                        self.registers.c = self.rotate_right_with_carry(self.registers.c)
                    }
                    ArithmeticTarget::D => {
                        self.registers.d = self.rotate_right_with_carry(self.registers.d)
                    }
                    ArithmeticTarget::E => {
                        self.registers.e = self.rotate_right_with_carry(self.registers.e)
                    }
                    ArithmeticTarget::H => {
                        self.registers.h = self.rotate_right_with_carry(self.registers.h)
                    }
                    ArithmeticTarget::L => {
                        self.registers.l = self.rotate_right_with_carry(self.registers.l)
                    }
                };

		self.pc.wrapping_add(1)
            }
            Instruction::RL(register) => {
                match register {
                    ArithmeticTarget::A => {
                        self.registers.a = self.rotate_left_with_carry(self.registers.a)
                    }
                    ArithmeticTarget::B => {
                        self.registers.b = self.rotate_left_with_carry(self.registers.b)
                    }
                    ArithmeticTarget::C => {
                        self.registers.c = self.rotate_left_with_carry(self.registers.c)
                    }
                    ArithmeticTarget::D => {
                        self.registers.d = self.rotate_left_with_carry(self.registers.d)
                    }
                    ArithmeticTarget::E => {
                        self.registers.e = self.rotate_left_with_carry(self.registers.e)
                    }
                    ArithmeticTarget::H => {
                        self.registers.h = self.rotate_left_with_carry(self.registers.h)
                    }
                    ArithmeticTarget::L => {
                        self.registers.l = self.rotate_left_with_carry(self.registers.l)
                    }
                };

		self.pc.wrapping_add(1)
            }
            Instruction::RRC(register) => {
                match register {
                    ArithmeticTarget::A => self.registers.a = self.rotate_right(self.registers.a),
                    ArithmeticTarget::B => self.registers.b = self.rotate_right(self.registers.b),
                    ArithmeticTarget::C => self.registers.c = self.rotate_right(self.registers.c),
                    ArithmeticTarget::D => self.registers.d = self.rotate_right(self.registers.d),
                    ArithmeticTarget::E => self.registers.e = self.rotate_right(self.registers.e),
                    ArithmeticTarget::H => self.registers.h = self.rotate_right(self.registers.h),
                    ArithmeticTarget::L => self.registers.l = self.rotate_right(self.registers.l),
                };

		self.pc.wrapping_add(1)
            }
            Instruction::RLC(register) => {
                match register {
                    ArithmeticTarget::A => self.registers.a = self.rotate_left(self.registers.a),
                    ArithmeticTarget::B => self.registers.b = self.rotate_left(self.registers.b),
                    ArithmeticTarget::C => self.registers.c = self.rotate_left(self.registers.c),
                    ArithmeticTarget::D => self.registers.d = self.rotate_left(self.registers.d),
                    ArithmeticTarget::E => self.registers.e = self.rotate_left(self.registers.e),
                    ArithmeticTarget::H => self.registers.h = self.rotate_left(self.registers.h),
                    ArithmeticTarget::L => self.registers.l = self.rotate_left(self.registers.l),
                };

		self.pc.wrapping_add(1)
            }
            Instruction::SRA(register) => {
                match register {
                    ArithmeticTarget::A => {
                        self.registers.a = self.rotate_right_arithmetic(self.registers.a)
                    }
                    ArithmeticTarget::B => {
                        self.registers.b = self.rotate_right_arithmetic(self.registers.b)
                    }
                    ArithmeticTarget::C => {
                        self.registers.c = self.rotate_right_arithmetic(self.registers.c)
                    }
                    ArithmeticTarget::D => {
                        self.registers.d = self.rotate_right_arithmetic(self.registers.d)
                    }
                    ArithmeticTarget::E => {
                        self.registers.e = self.rotate_right_arithmetic(self.registers.e)
                    }
                    ArithmeticTarget::H => {
                        self.registers.h = self.rotate_right_arithmetic(self.registers.h)
                    }
                    ArithmeticTarget::L => {
                        self.registers.l = self.rotate_right_arithmetic(self.registers.l)
                    }
                };

		self.pc.wrapping_add(1)
            }
            Instruction::SLA(register) => {
                match register {
                    ArithmeticTarget::A => {
                        self.registers.a = self.rotate_left_arithmetic(self.registers.a)
                    }
                    ArithmeticTarget::B => {
                        self.registers.b = self.rotate_left_arithmetic(self.registers.b)
                    }
                    ArithmeticTarget::C => {
                        self.registers.c = self.rotate_left_arithmetic(self.registers.c)
                    }
                    ArithmeticTarget::D => {
                        self.registers.d = self.rotate_left_arithmetic(self.registers.d)
                    }
                    ArithmeticTarget::E => {
                        self.registers.e = self.rotate_left_arithmetic(self.registers.e)
                    }
                    ArithmeticTarget::H => {
                        self.registers.h = self.rotate_left_arithmetic(self.registers.h)
                    }
                    ArithmeticTarget::L => {
                        self.registers.l = self.rotate_left_arithmetic(self.registers.l)
                    }
                };

		self.pc.wrapping_add(1)
            }
            Instruction::SWAP(register) => {
                match register {
                    ArithmeticTarget::A => self.registers.a = self.swap(self.registers.a),
                    ArithmeticTarget::B => self.registers.b = self.swap(self.registers.b),
                    ArithmeticTarget::C => self.registers.c = self.swap(self.registers.c),
                    ArithmeticTarget::D => self.registers.d = self.swap(self.registers.d),
                    ArithmeticTarget::E => self.registers.e = self.swap(self.registers.e),
                    ArithmeticTarget::H => self.registers.h = self.swap(self.registers.h),
                    ArithmeticTarget::L => self.registers.l = self.swap(self.registers.l),
                };

		self.pc.wrapping_add(1)
            }
	    Instruction::JP(condition) => {
		let condition = match condition {
		    JumpCondition::NotZero => !self.registers.f.zero,
		    JumpCondition::Zero => self.registers.f.zero,
		    JumpCondition::NotCarry => !self.registers.f.carry,
		    JumpCondition::Carry => self.registers.f.carry,
		    JumpCondition::Always => true,
		};

		self.jump(condition)
	    }
	    Instruction::JR(condition) => {
		let condition = match condition {
		    JumpCondition::NotZero => !self.registers.f.zero,
		    JumpCondition::Zero => self.registers.f.zero,
		    JumpCondition::NotCarry => !self.registers.f.carry,
		    JumpCondition::Carry => self.registers.f.carry,
		    JumpCondition::Always => true,
		};

		self.jump_relative(condition)
	    }
	    Instruction::JPI => {
		self.registers.get_hl()
	    }
	    Instruction::LD(load_type) => {
		match load_type {
		    LoadType::Byte(target, src) => {
			let value = match src {
			    LoadByteSrc::A => self.registers.a,
			    LoadByteSrc::B => self.registers.b,
			    LoadByteSrc::C => self.registers.c,
			    LoadByteSrc::D => self.registers.d,
			    LoadByteSrc::E => self.registers.e,
			    LoadByteSrc::H => self.registers.h,
			    LoadByteSrc::L => self.registers.l,
			    LoadByteSrc::D8 => self.bus.read_byte(self.pc + 1), // direct 8bit value
			    LoadByteSrc::HLI => self.bus.read_byte(self.registers.get_hl()),
			};

			match target {
			    LoadByteTarget::A => self.registers.a = value,
			    LoadByteTarget::B => self.registers.b = value,
			    LoadByteTarget::C => self.registers.c = value,
			    LoadByteTarget::D => self.registers.d = value,
			    LoadByteTarget::E => self.registers.e = value,
			    LoadByteTarget::H => self.registers.h = value,
			    LoadByteTarget::L => self.registers.l = value,
			    LoadByteTarget::HLI => self.bus.write_byte(self.registers.get_hl(), value),
			};

			match src {
			    LoadByteSrc::D8 => self.pc.wrapping_add(3),
			    _ => self.pc.wrapping_add(1),
			}
		    }
		    LoadType::Word(target) => {
			let value = self.next_word();
			match target {
			    LoadWordTarget::BC => self.registers.set_bc(value),
			    LoadWordTarget::DE => self.registers.set_de(value),
			    LoadWordTarget::HL => self.registers.set_hl(value),
			}

			self.pc.wrapping_add(3)
		    }
		    LoadType::AFromIndirect(target) => {
			self.registers.a = match target {
			    IndirectSrc::BC => self.bus.read_byte(self.registers.get_bc()),
			    IndirectSrc::DE => self.bus.read_byte(self.registers.get_de()),
			    IndirectSrc::HLMinus => {
				let hl = self.registers.get_hl();
				self.registers.set_hl(hl.wrapping_sub(1));
				self.bus.read_byte(hl)
			    },
			    IndirectSrc::HLPlus => {
				let hl = self.registers.get_hl();
				self.registers.set_hl(hl.wrapping_add(1));
				self.bus.read_byte(hl)
			    },
			    IndirectSrc::D8 => self.bus.read_byte(self.next_word()),
			    IndirectSrc::IOPortC => self.bus.read_byte(0xFF00 + self.registers.c as u16),
			};

			match target {
			    IndirectSrc::D8 => self.pc.wrapping_add(3),
			    _ => self.pc.wrapping_add(1),
			}
		    }
		    LoadType::IndirectFromA(src) => {
			let value = self.registers.a;
			match src {
			    IndirectSrc::BC => self.bus.write_byte(self.registers.get_bc(), value),
			    IndirectSrc::DE => self.bus.write_byte(self.registers.get_de(), value),
			    IndirectSrc::HLMinus => {
				let hl = self.registers.get_hl();
				self.registers.set_hl(hl.wrapping_sub(1));
				self.bus.write_byte(hl, value)
			    },
			    IndirectSrc::HLPlus => {
				let hl = self.registers.get_hl();
				self.registers.set_hl(hl.wrapping_add(1));
				self.bus.write_byte(hl, value)
			    },
			    IndirectSrc::D8 => self.bus.write_byte(self.next_word(), value),
			    IndirectSrc::IOPortC => self.bus.write_byte(0xFF00 + self.registers.c as u16, value),
			}

			match src {
			    IndirectSrc::D8 => self.pc.wrapping_add(3),
			    _ => self.pc.wrapping_add(1),
			}
		    }
		    LoadType::AFromByteAddress => {
			let offset = self.bus.read_byte(self.pc+1) as u16;
			self.bus.write_byte(0xFF00+offset, self.registers.a);
			self.pc.wrapping_add(2)
		    },
		    LoadType::ByteAddressFromA => {
			let offset = self.bus.read_byte(self.pc+1) as u16;
			self.registers.a = self.bus.read_byte(0xFF00+offset);
			self.pc.wrapping_add(2)
		    },
		}
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
        let (carry_res, did_carry_overflow) = self
            .registers
            .a
            .overflowing_add(self.registers.f.carry as u8);
        let (res, did_overflow) = carry_res.overflowing_add(value);

        self.registers.f.zero = res == 0;
        self.registers.f.subtraction = false;
        self.registers.f.half_carry =
            (self.registers.a & 0xF) + (value & 0xF) + (self.registers.f.carry as u8) > 0xF;
        self.registers.f.carry = did_carry_overflow || did_overflow;

        res
    }

    fn sub(&mut self, value: u8) -> u8 {
        let (res, did_overflow) = self.registers.a.overflowing_sub(value);

        // Flag handling
        self.registers.f.zero = res == 0;
        self.registers.f.subtraction = true;
        self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF);
        self.registers.f.carry = did_overflow;

        res
    }

    fn sub_with_carry(&mut self, value: u8) -> u8 {
        let tmp_carry = self.registers.f.carry as u8;
        let (carry_res, did_carry_overflow) = self.registers.a.overflowing_sub(tmp_carry);
        let (res, did_overflow) = carry_res.overflowing_sub(value);

        self.registers.f.zero = res == 0;
        self.registers.f.subtraction = true;
        self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF) + tmp_carry;
        self.registers.f.carry = did_carry_overflow || did_overflow;

        res
    }

    fn and(&mut self, value: u8) -> u8 {
        let res = self.registers.a & value;

        self.registers.f.zero = res == 0;
        self.registers.f.subtraction = false;
        self.registers.f.half_carry = true;
        self.registers.f.carry = false;

        res
    }

    fn or(&mut self, value: u8) -> u8 {
        let res = self.registers.a | value;

        self.registers.f.zero = res == 0;
        self.registers.f.subtraction = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;

        res
    }

    fn xor(&mut self, value: u8) -> u8 {
        let res = self.registers.a ^ value;

        self.registers.f.zero = res == 0;
        self.registers.f.subtraction = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;

        res
    }

    // A - value
    fn compare(&mut self, value: u8) {
        self.registers.f.zero = self.registers.a == value;
        self.registers.f.subtraction = true;
        self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF);
        self.registers.f.carry = self.registers.a < value;
    }

    fn inc_8b(&mut self, value: u8) -> u8 {
        let res = value.wrapping_add(value);

        self.registers.f.zero = res == 0;
        self.registers.f.subtraction = false;
        self.registers.f.half_carry = (value & 0xF) == 0xF;

        res
    }

    fn inc_16b(&mut self, value: u16) -> u16 {
        value.wrapping_add(1)
    }

    fn dec_8b(&mut self, value: u8) -> u8 {
        let res = value.wrapping_sub(value);

        self.registers.f.zero = res == 0;
        self.registers.f.subtraction = true;
        self.registers.f.half_carry = (value & 0xF) == 0x0;

        res
    }

    fn dec_16b(&mut self, value: u16) -> u16 {
        value.wrapping_sub(1)
    }

    fn complement_carry(&mut self) {
        self.registers.f.subtraction = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = self.registers.f.carry ^ true;
    }

    fn set_carry(&mut self) {
        self.registers.f.subtraction = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = true;
    }

    fn rotate_right_a_with_carry(&mut self) {
        let carry: u8 = if self.registers.f.carry { 1 } else { 0 } << 7;

        self.registers.f.zero = false;
        self.registers.f.subtraction = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = (self.registers.a & 0b1) == 0b1;

        self.registers.a = (self.registers.a >> 1) | carry;
    }

    fn rotate_left_a_with_carry(&mut self) {
        let carry: u8 = if self.registers.f.carry { 1 } else { 0 };

        self.registers.f.zero = false;
        self.registers.f.subtraction = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = (self.registers.a & 0x80) == 0x80;

        self.registers.a = (self.registers.a << 1) | carry;
    }

    fn rotate_right_a(&mut self) {
        self.registers.f.zero = false;
        self.registers.f.subtraction = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = (self.registers.a & 0b1) == 0b1;

        self.registers.a = self.registers.a >> 1;
    }

    fn rotate_left_a(&mut self) {
        self.registers.f.zero = false;
        self.registers.f.subtraction = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = (self.registers.a & 0x80) == 0x80;

        self.registers.a = self.registers.a << 1;
    }

    fn complement(&mut self) {
        self.registers.f.subtraction = true;
        self.registers.f.half_carry = true;
        self.registers.a = self.registers.a ^ 0xFF;
    }

    fn test_bit(&mut self, value: u8, bit: BitPosition) {
        let bit: u8 = bit.into();
        let res = (value >> bit) & 0b1;

        self.registers.f.zero = res == 0;
        self.registers.f.subtraction = false;
        self.registers.f.half_carry = true;
    }

    fn reset_bit(&self, value: u8, bit: BitPosition) -> u8 {
        let bit: u8 = bit.into();
        value & !(1 << bit)
    }

    fn set_bit(&self, value: u8, bit: BitPosition) -> u8 {
        let bit: u8 = bit.into();
        value | (1 << bit)
    }

    fn shift_right_logical(&mut self, value: u8) -> u8 {
        let res = value >> 1;

        self.registers.f.zero = res == 0;
        self.registers.f.subtraction = false;
        self.registers.f.half_carry = true;
        self.registers.f.carry = (self.registers.a & 0b1) == 0b1;

        res
    }

    fn rotate_right_with_carry(&mut self, value: u8) -> u8 {
        let carry: u8 = if self.registers.f.carry { 1 } else { 0 } << 7;
        let res = (value >> 1) | carry;

        self.registers.f.zero = res == 0;
        self.registers.f.subtraction = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = (value & 0b1) == 0b1;

        res
    }

    fn rotate_left_with_carry(&mut self, value: u8) -> u8 {
        let carry: u8 = if self.registers.f.carry { 1 } else { 0 } << 7;
        let res = (value << 1) | carry;

        self.registers.f.zero = false;
        self.registers.f.subtraction = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = (value & 0x80) == 0x80;

        res
    }

    fn rotate_right(&mut self, value: u8) -> u8 {
        let res = value >> 1;

        self.registers.f.zero = res == 0;
        self.registers.f.subtraction = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = (value & 0b1) == 0b1;

        res
    }

    fn rotate_left(&mut self, value: u8) -> u8 {
        let res = value << 1;

        self.registers.f.zero = false;
        self.registers.f.subtraction = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = (value & 0x80) == 0x80;

        res
    }

    fn rotate_right_arithmetic(&mut self, value: u8) -> u8 {
        let res = value & 0x80 | (value >> 1);

        self.registers.f.zero = false;
        self.registers.f.subtraction = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = (value & 0b1) == 0b1;

        res
    }

    fn rotate_left_arithmetic(&mut self, value: u8) -> u8 {
        let res = value & 0x80 | (value << 1);

        self.registers.f.zero = false;
        self.registers.f.subtraction = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = (value & 0x80) == 0x80;

        res
    }

    fn swap(&mut self, value: u8) -> u8 {
        let res = (value & 0xF0 >> 4) | (value & 0xF << 4);

        self.registers.f.zero = res == 0;
        self.registers.f.subtraction = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;

        res
    }

    fn next_word(&self) -> u16 {
	let lower_nibble = self.bus.read_byte(self.pc+1) as u16;
	let higher_nibble = self.bus.read_byte(self.pc+2) as u16;
	
	(higher_nibble << 8) | lower_nibble
    }

    fn jump(&self, condition: bool) -> u16 {
	if condition {
	    self.next_word()
	} else {
	    self.pc.wrapping_add(3)
	}
    }

    fn jump_relative(&self, condition: bool) -> u16 {
	let next = self.pc.wrapping_add(2);

	if condition {
	    let offset = self.bus.read_byte(self.pc+1) as i8;

	    let pc = if offset >= 0 {
		next.wrapping_add(offset as u16)
	    } else {
		next.wrapping_sub(offset.abs() as u16)
	    };

	    pc
	} else {
	    next
	}
    }
}
