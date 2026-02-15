use std::collections::HashMap;

use crate::{
    cell::Cell,
    imp::{DataType, Program},
};

#[derive(Clone, PartialEq, Eq)]
enum Whitespace {
    Space,
    Tab,
    LineFeed,
    NoWhitespace,
}

impl From<char> for Whitespace {
    fn from(value: char) -> Self {
        match value {
            ' ' => Whitespace::Space,
            '\t' => Whitespace::Tab,
            '\n' => Whitespace::LineFeed,
            _ => Whitespace::NoWhitespace,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    StackManipulation,
    Arithmetic,
    HeapAccess,
    FlowControl,
    Io,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StackManipulationImp {
    Push,
    Duplicate,
    Copy,
    Swap,
    Discard,
    Slide,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArithmeticImp {
    Addition,
    Subtraction,
    Multiplication,
    IntegerDivision,
    Modulo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HeapAccessImp {
    Store,
    Retrieve,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FlowControlImp {
    Label,
    Call,
    Jump,
    JumpIfZero,
    JumpIfNotZero,
    Return,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum IoImp {
    OutputChar,
    OutputInt,
    InputChar,
    InputInt,
}

#[derive(Clone, Copy, Debug)]
enum ParseState {
    Program,
    Param(DataType),
}

#[derive(Clone)]
struct ParserWorking {
    i: usize,
    c: char,

    state: ParseState,
    token: String,
    space: Whitespace,
    instruction: Option<Instruction>,
}

impl Default for ParserWorking {
    fn default() -> Self {
        Self {
            i: 0,
            c: '\0',
            space: Whitespace::NoWhitespace,
            state: ParseState::Program,
            token: String::new(),
            instruction: None,
        }
    }
}

impl ParserWorking {
    pub fn soft_reset(&mut self) {
        self.state = ParseState::Program;
        self.token.clear();
        self.instruction = None;
    }
}

fn dump(s: &String) -> String {
    let char_map: HashMap<char, char> = HashMap::from([(' ', 'S'), ('\t', 'T'), ('\n', 'L')]);

    let mut result = String::new();
    for c in s.chars() {
        let rc = char_map.get(&c).unwrap_or(&c);
        result.push(*rc);
    }

    result
}

pub(crate) struct Parser {
    //
}

impl Parser {
    pub fn parse<T: Cell>(source: impl Into<String>) -> Vec<Program<T>> {
        let instruction_map: HashMap<&'static str, Instruction> = HashMap::from([
            (" ", Instruction::StackManipulation),
            ("\t ", Instruction::Arithmetic),
            ("\t\t", Instruction::HeapAccess),
            ("\n", Instruction::FlowControl),
            ("\t\n", Instruction::Io),
        ]);

        let stack_manipulation_map: HashMap<&'static str, StackManipulationImp> = HashMap::from([
            (" ", StackManipulationImp::Push),
            ("\n ", StackManipulationImp::Duplicate),
            ("\t ", StackManipulationImp::Copy),
            ("\n\t", StackManipulationImp::Swap),
            ("\n\n", StackManipulationImp::Discard),
            ("\t\n", StackManipulationImp::Slide),
        ]);

        let arithmetic_map: HashMap<&'static str, ArithmeticImp> = HashMap::from([
            ("  ", ArithmeticImp::Addition),
            (" \t", ArithmeticImp::Subtraction),
            (" \n", ArithmeticImp::Multiplication),
            ("\t ", ArithmeticImp::IntegerDivision),
            ("\t\t", ArithmeticImp::Modulo),
        ]);

        let heap_access_map: HashMap<&'static str, HeapAccessImp> =
            HashMap::from([(" ", HeapAccessImp::Store), ("\t", HeapAccessImp::Retrieve)]);

        let flow_control_map: HashMap<&'static str, FlowControlImp> = HashMap::from([
            ("  ", FlowControlImp::Label),
            (" \t", FlowControlImp::Call),
            (" \n", FlowControlImp::Jump),
            ("\t ", FlowControlImp::JumpIfZero),
            ("\t\t", FlowControlImp::JumpIfNotZero),
            ("\t\n", FlowControlImp::Return),
            ("\n\n", FlowControlImp::End),
        ]);

        let io_map: HashMap<&'static str, IoImp> = HashMap::from([
            ("  ", IoImp::OutputChar),
            (" \t", IoImp::OutputInt),
            ("\t ", IoImp::InputChar),
            ("\t\t", IoImp::InputInt),
        ]);

        let char_map: HashMap<char, &'static str> =
            HashMap::from([(' ', "S"), ('\t', "T"), ('\n', "L")]);
        let allow_chars = char_map.keys().collect::<String>();

        let mut working = ParserWorking::default();
        let mut working_stack: Vec<ParserWorking> = vec![];

        let mut result: Vec<Program<T>> = vec![];

        for (i, c) in source.into().chars().enumerate() {
            working.i = i;
            working.c = c;
            working.space = c.into();

            if let Some(s) = char_map.get(&c) {
                println!("c={}, t={}, {:?}", s, dump(&working.token), working.state);
            } else {
                println!("unexpected char: {} (0x{:08X})", c, c as u32);
            }

            if !allow_chars.contains(c) {
                continue;
            }

            working.token.push(c);

            match working.state {
                ParseState::Program => {
                    if working.instruction.is_none() {
                        if let Some(imp) = instruction_map.get(working.token.as_str()) {
                            working.instruction = Some(*imp);
                            working.token.clear();
                        } else {
                            println!("!!");
                        }
                        continue;
                    }

                    println!("imp = {:?}", working.instruction);

                    let token = working.token.as_str();
                    match working.instruction.unwrap() {
                        Instruction::StackManipulation => {
                            if let Some(imp) = stack_manipulation_map.get(token) {
                                match imp {
                                    StackManipulationImp::Push
                                    | StackManipulationImp::Copy
                                    | StackManipulationImp::Slide => {
                                        working_stack.push(working.clone());
                                        working = ParserWorking::default();
                                        working.instruction = Some(Instruction::StackManipulation);
                                        working.state = ParseState::Param(DataType::Number);
                                        continue;
                                    }
                                    StackManipulationImp::Duplicate => {
                                        result.push(Program::Duplicate)
                                    }
                                    StackManipulationImp::Swap => result.push(Program::Swap),
                                    StackManipulationImp::Discard => result.push(Program::Discard),
                                }

                                working.soft_reset();
                            }
                        }
                        Instruction::Arithmetic => {
                            if let Some(_) = arithmetic_map.get(token) {
                                if let Some(imp) = arithmetic_map.get(token) {
                                    match imp {
                                        ArithmeticImp::Addition => result.push(Program::Addition),
                                        ArithmeticImp::Subtraction => {
                                            result.push(Program::Subtraction)
                                        }
                                        ArithmeticImp::Multiplication => {
                                            result.push(Program::Multiplication)
                                        }
                                        ArithmeticImp::IntegerDivision => {
                                            result.push(Program::IntegerDivision)
                                        }
                                        ArithmeticImp::Modulo => result.push(Program::Modulo),
                                    }
                                }

                                working.soft_reset();
                            }
                        }
                        Instruction::HeapAccess => {
                            if let Some(imp) = heap_access_map.get(token) {
                                match imp {
                                    HeapAccessImp::Store => result.push(Program::Store),
                                    HeapAccessImp::Retrieve => result.push(Program::Retrieve),
                                }

                                working.soft_reset();
                            }
                        }
                        Instruction::FlowControl => {
                            if let Some(imp) = flow_control_map.get(token) {
                                match imp {
                                    FlowControlImp::Label
                                    | FlowControlImp::Call
                                    | FlowControlImp::Jump
                                    | FlowControlImp::JumpIfZero
                                    | FlowControlImp::JumpIfNotZero => {
                                        working_stack.push(working.clone());
                                        working = ParserWorking::default();
                                        working.instruction = Some(Instruction::FlowControl);
                                        working.state = ParseState::Param(DataType::Label);
                                        continue;
                                    }
                                    FlowControlImp::Return => result.push(Program::Return),
                                    FlowControlImp::End => result.push(Program::End),
                                }

                                working.soft_reset();
                            }
                        }
                        Instruction::Io => {
                            if let Some(imp) = io_map.get(token) {
                                match imp {
                                    IoImp::OutputChar => result.push(Program::OutputChar),
                                    IoImp::OutputInt => result.push(Program::OutputInt),
                                    IoImp::InputChar => result.push(Program::InputChar),
                                    IoImp::InputInt => result.push(Program::InputInt),
                                }

                                working.soft_reset();
                            }
                        }
                    }
                }
                ParseState::Param(t) => {
                    if working.space != Whitespace::LineFeed {
                        continue;
                    }

                    // pop delimiter of LF
                    working.token.pop();

                    match t {
                        DataType::Number => {
                            let mut chars = working.token.chars();
                            let positive = match chars.next().unwrap_or_default().into() {
                                Whitespace::Space => true,
                                Whitespace::Tab => false,
                                _ => panic!(),
                            };
                            let mut number: T = T::zero();
                            for c in chars {
                                let n = match c.into() {
                                    Whitespace::Space => T::zero(),
                                    Whitespace::Tab => T::one(),
                                    _ => panic!(),
                                };
                                number <<= T::one();
                                number |= n;
                            }

                            if !positive {
                                number = -number;
                            }

                            working = working_stack.pop().unwrap();

                            match working.instruction.unwrap() {
                                Instruction::StackManipulation => {
                                    match stack_manipulation_map
                                        .get(&working.token.as_str())
                                        .unwrap()
                                    {
                                        StackManipulationImp::Push => {
                                            result.push(Program::Push(number))
                                        }
                                        StackManipulationImp::Copy => {
                                            result.push(Program::Copy(number))
                                        }
                                        StackManipulationImp::Slide => {
                                            result.push(Program::Slide(number))
                                        }
                                        _ => panic!(),
                                    }

                                    working.soft_reset();
                                }
                                _ => {
                                    panic!();
                                }
                            }

                            continue;
                        }
                        DataType::Label => match working.instruction.unwrap() {
                            Instruction::FlowControl => {
                                let cloned_token = working.token.clone();
                                working = working_stack.pop().unwrap();

                                match flow_control_map.get(&working.token.as_str()).unwrap() {
                                    FlowControlImp::Label => {
                                        result.push(Program::Label(cloned_token));
                                    }
                                    FlowControlImp::Call => {
                                        result.push(Program::Call(cloned_token));
                                    }
                                    FlowControlImp::Jump => {
                                        result.push(Program::Jump(cloned_token));
                                    }
                                    FlowControlImp::JumpIfZero => {
                                        result.push(Program::JumpIfZero(cloned_token));
                                    }
                                    FlowControlImp::JumpIfNotZero => {
                                        result.push(Program::JumpIfNotZero(cloned_token));
                                    }
                                    _ => {
                                        panic!();
                                    }
                                }

                                working.soft_reset();
                            }
                            _ => {
                                panic!();
                            }
                        },
                    }
                }
                _ => {
                    panic!("STOP");
                }
            }
        }

        result
    }
}
