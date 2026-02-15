use crate::cell::Cell;

#[derive(Clone, Copy, Debug)]
pub(crate) enum DataType {
    Number,
    Label,
}

#[derive(Clone)]
pub(crate) enum Data<T: Cell> {
    Number(T),
    Label(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Program<T: Cell> {
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
    JumpIfNotZero(String),
    Return,
    End,
    OutputChar,
    OutputInt,
    InputChar,
    InputInt,
}
