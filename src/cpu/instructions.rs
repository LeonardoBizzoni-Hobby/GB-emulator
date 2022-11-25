pub enum Instruction {
    ADD(ArithmeticTarget),
    ADDHL(GroupedArithmeticTarget),
    ADC(ArithmeticTarget),
    SUB(ArithmeticTarget),
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
