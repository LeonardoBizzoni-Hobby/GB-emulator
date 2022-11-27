pub enum Instruction {
    ADDHL(GroupedArithmeticTarget),

    ADD(ArithmeticTarget),
    ADC(ArithmeticTarget),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    AND(ArithmeticTarget),
    OR(ArithmeticTarget),
    XOR(ArithmeticTarget),
    CP(ArithmeticTarget),
    SRL(ArithmeticTarget),
    RR(ArithmeticTarget),
    RL(ArithmeticTarget),
    RRC(ArithmeticTarget),
    RLC(ArithmeticTarget),
    SRA(ArithmeticTarget),
    SLA(ArithmeticTarget),
    SWAP(ArithmeticTarget),

    INC(IncDecTarget),
    DEC(IncDecTarget),

    CCF,
    SCF,
    RRA,
    RLA,
    RRCA,
    RLCA,
    CPL,

    BIT(ArithmeticTarget, BitPosition),
    RESET(ArithmeticTarget, BitPosition),
    SET(ArithmeticTarget, BitPosition),
}

pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub enum GroupedArithmeticTarget {
    BC,
    DE,
    HL,
}

pub enum IncDecTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    BC,
    DE,
    HL,
}

pub enum BitPosition {
    B0,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
}

impl std::convert::From<BitPosition> for u8 {
    fn from(bit: BitPosition) -> u8 {
	match bit {
	    BitPosition::B0 => 0,
	    BitPosition::B1 => 1,
	    BitPosition::B2 => 2,
	    BitPosition::B3 => 3,
	    BitPosition::B4 => 4,
	    BitPosition::B5 => 5,
	    BitPosition::B6 => 6,
	    BitPosition::B7 => 7,
	}
    }
}

impl Instruction {
    pub fn from_byte(instruction_address: u8) -> Option<Instruction> {
	todo!()
    }
}
