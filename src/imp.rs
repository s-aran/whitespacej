use crate::cell::Cell;

#[derive(Clone, Copy, Debug)]
pub(crate) enum DataType {
    Number,
    Label,
}

#[derive(Debug, Clone, PartialEq, Eq, strum::Display)]
pub enum Program<T: Cell> {
    Push(T),
    Duplicate,
    Copy(T),
    Swap,
    Discard,
    Slide(T),
    Addition,
    Subtraction,
    Multiplication,
    IntegerDivision,
    Modulo,
    Store,
    Retrieve,
    Label(String),
    Call(String),
    Jump(String),
    JumpIfZero(String),
    JumpIfNegative(String),
    Return,
    End,
    OutputChar,
    OutputInt,
    InputChar,
    InputInt,
}
