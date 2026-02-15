use std::collections::HashMap;
use std::hash::Hash;

use crate::{cell::Cell, imp::Program};

struct Stack<T: Cell> {
    stack: Vec<T>,
}

impl<T: Cell> Stack<T> {
    pub fn new() -> Self {
        Self { stack: vec![] }
    }

    #[inline]
    pub fn stack(&self) -> &Vec<T> {
        &self.stack
    }

    #[inline]
    pub fn push(&mut self, value: T) {
        self.stack.push(value);
    }

    #[inline]
    pub fn pop(&mut self) -> T {
        self.stack.pop().unwrap_or(T::zero())
    }

    #[inline]
    pub fn last(&self) -> &T {
        self.stack.last().unwrap()
    }

    pub fn swap(&mut self) {
        let a = self.stack.len() - 1;
        let b = a - 1;
        self.stack.swap(a, b);
    }
}

struct Heap<K: Cell + Hash, V: Cell> {
    map: HashMap<K, V>,
}

impl<K: Cell, V: Cell> Heap<K, V> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    #[inline]
    pub fn contains(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    #[inline]
    pub fn get(&self, key: &K) -> &V {
        self.map.get(key).unwrap()
    }

    #[inline]
    pub fn set(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }
}

#[derive(Debug, strum::Display)]
enum RuntimeError<T: Cell> {
    ConvertFailed(String),
    SmallerStack(usize, usize),
    LabelDoesNotFound(String),
    KeyDoesNotFound(T),
    Unknown,
}

pub(crate) struct Machine<T: Cell> {
    stack: Stack<T>,
    heap: Heap<T, T>,
    call_stack: Vec<usize>,
    jump_table: HashMap<String, usize>,

    pc: usize,
    program_list: Vec<Program<T>>,
}

impl<T: Cell> Machine<T> {
    pub fn new(program: Vec<Program<T>>) -> Self {
        Self {
            stack: Stack::new(),
            heap: Heap::new(),
            call_stack: vec![],
            jump_table: HashMap::new(),
            pc: 0,
            program_list: program,
        }
    }

    fn to_viewable(whitespace: &String) -> String {
        whitespace
            .chars()
            .map(|c| match c {
                ' ' => 'S',
                '\t' => 'T',
                '\n' => 'L',
                _ => c,
            })
            .collect()
    }

    fn dump(&self) {
        println!("program:");
        const COUNT: usize = 5;
        for i in (self.pc.saturating_sub(COUNT))..=self.pc {
            if i == self.pc {
                print!("-> ");
            } else {
                print!("   ");
            }

            if let Some(e) = self.program_list.get(self.pc - i) {
                println!("{}: {}", i, e);
            }
        }

        println!("jump table:");
        for (k, v) in self.jump_table.iter() {
            println!("  {} => {}", Self::to_viewable(k), v);
        }

        println!("stack:");
        for (i, e) in self.stack.stack().iter().enumerate() {
            println!("  {}: {}", i, e.to_i64().unwrap());
        }

        println!("heap:");
        for (k, v) in self.heap.map.iter() {
            println!("  {}: {}", k.to_i64().unwrap(), v.to_i64().unwrap());
        }
    }

    fn push(&mut self, value: T) -> Result<(), RuntimeError<T>> {
        self.stack.push(value);
        Ok(())
    }

    fn duplicate(&mut self) -> Result<(), RuntimeError<T>> {
        if self.stack.stack().is_empty() {
            return Err(RuntimeError::SmallerStack(self.stack.stack().len(), 1));
        }

        self.stack.push(*self.stack.last());
        Ok(())
    }

    fn copy(&mut self, nth: T) -> Result<(), RuntimeError<T>> {
        if let Some(v) = self.stack.stack().get(nth.to_usize().unwrap()) {
            self.stack.push(*v);
            Ok(())
        } else {
            Err(RuntimeError::SmallerStack(
                self.stack.stack().len(),
                nth.to_usize().unwrap(),
            ))
        }
    }

    fn swap(&mut self) -> Result<(), RuntimeError<T>> {
        if self.stack.stack().len() < 2 {
            return Err(RuntimeError::SmallerStack(self.stack.stack().len(), 2));
        }

        self.stack.swap();
        Ok(())
    }

    fn discard(&mut self) -> Result<(), RuntimeError<T>> {
        if self.stack.stack().is_empty() {
            return Err(RuntimeError::SmallerStack(self.stack.stack().len(), 1));
        }

        self.stack.pop();
        Ok(())
    }

    fn slide(&mut self, n: T) -> Result<(), RuntimeError<T>> {
        if self.stack.stack().len() <= n.to_usize().unwrap() {
            return Err(RuntimeError::SmallerStack(
                self.stack.stack().len(),
                n.to_usize().unwrap(),
            ));
        }

        let keep = self.stack.pop();
        for _ in 0..n.to_usize().unwrap() {
            self.stack.pop();
        }
        self.stack.push(keep);

        Ok(())
    }

    fn add(&mut self) -> Result<(), RuntimeError<T>> {
        let rhs = self.stack.pop();
        let lhs = self.stack.pop();

        self.stack.push(lhs + rhs);
        Ok(())
    }

    fn sub(&mut self) -> Result<(), RuntimeError<T>> {
        let rhs = self.stack.pop();
        let lhs = self.stack.pop();

        self.stack.push(lhs - rhs);
        Ok(())
    }

    fn mul(&mut self) -> Result<(), RuntimeError<T>> {
        let rhs = self.stack.pop();
        let lhs = self.stack.pop();

        self.stack.push(lhs * rhs);
        Ok(())
    }

    fn div(&mut self) -> Result<(), RuntimeError<T>> {
        let rhs = self.stack.pop();
        let lhs = self.stack.pop();

        self.stack.push(lhs / rhs);
        Ok(())
    }

    fn modulo(&mut self) -> Result<(), RuntimeError<T>> {
        let rhs = self.stack.pop();
        let lhs = self.stack.pop();

        self.stack.push(lhs % rhs);
        Ok(())
    }

    fn store(&mut self) -> Result<(), RuntimeError<T>> {
        let value = self.stack.pop();
        let key = self.stack.pop();

        self.heap.set(key, value);
        Ok(())
    }

    fn retrieve(&mut self) -> Result<(), RuntimeError<T>> {
        let key = self.stack.pop();
        if !self.heap.contains(&key) {
            return Err(RuntimeError::KeyDoesNotFound(key));
        }
        let value = self.heap.get(&key).clone();

        self.stack.push(value);
        Ok(())
    }

    fn call_subroutine(&mut self, label: String) -> Result<(), RuntimeError<T>> {
        if !self.jump_table.contains_key(&label) {
            return Err(RuntimeError::LabelDoesNotFound(label));
        }

        self.call_stack.push(self.pc);
        self.pc = *self.jump_table.get(&label).unwrap();
        Ok(())
    }

    fn jump(&mut self, label: String) -> Result<(), RuntimeError<T>> {
        if !self.jump_table.contains_key(&label) {
            return Err(RuntimeError::LabelDoesNotFound(label));
        }

        self.pc = *self.jump_table.get(&label).unwrap();
        Ok(())
    }

    fn jump_if_zero(&mut self, label: String) -> Result<(), RuntimeError<T>> {
        if !self.jump_table.contains_key(&label) {
            return Err(RuntimeError::LabelDoesNotFound(label));
        }

        let condition = self.stack.pop();
        if condition == T::zero() {
            self.pc = *self.jump_table.get(&label).unwrap();
        }

        Ok(())
    }

    fn jump_if_not_zero(&mut self, label: String) -> Result<(), RuntimeError<T>> {
        if !self.jump_table.contains_key(&label) {
            return Err(RuntimeError::LabelDoesNotFound(label));
        }

        let condition = self.stack.pop();
        if condition != T::zero() {
            self.pc = *self.jump_table.get(&label).unwrap();
        }

        Ok(())
    }

    fn end_subroutine(&mut self) -> Result<(), RuntimeError<T>> {
        if self.call_stack.is_empty() {
            return Err(RuntimeError::Unknown);
        }

        self.pc = self.call_stack.pop().unwrap();
        Ok(())
    }

    fn halt(&self) -> ! {
        std::process::exit(0);
    }

    fn output_char(&mut self) -> Result<(), RuntimeError<T>> {
        if self.stack.stack().is_empty() {
            return Err(RuntimeError::SmallerStack(self.stack.stack().len(), 1));
        }

        let c = char::from_u32(self.stack.pop().to_u32().unwrap()).unwrap();
        print!("{}", c);

        Ok(())
    }

    fn output_number(&mut self) -> Result<(), RuntimeError<T>> {
        let n = self.stack.pop().to_i64().unwrap();
        print!("{}", n);

        Ok(())
    }

    fn input_char(&mut self) -> Result<(), RuntimeError<T>> {
        if self.stack.stack().is_empty() {
            return Err(RuntimeError::SmallerStack(self.stack.stack().len(), 1));
        }

        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();

        if buffer.len() <= 0 {
            return Ok(());
        }

        let c = buffer.chars().next().unwrap() as u32;
        let key = self.stack.pop();
        self.heap.set(key, c.into());

        Ok(())
    }

    fn input_number(&mut self) -> Result<(), RuntimeError<T>> {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();

        match T::from_str_radix(buffer.as_str(), 10) {
            Ok(v) => self.stack.push(v),
            Err(_) => {
                return Err(RuntimeError::ConvertFailed(buffer));
            }
        }

        Ok(())
    }

    fn fetch(&self) -> &Program<T> {
        if let Some(p) = self.program_list.get(self.pc) {
            p
        } else {
            self.halt()
        }
    }

    fn register_label(&mut self) {
        for (idx, prog) in self.program_list.iter().enumerate() {
            if let Program::Label(label) = prog {
                self.jump_table.insert(label.clone(), idx);
            }
        }
    }

    pub fn execute(&mut self) -> ! {
        self.register_label();

        loop {
            let p = self.fetch();
            let result = match p {
                Program::Push(value) => self.push(*value),
                Program::Duplicate => self.duplicate(),
                Program::Copy(nth) => self.copy(*nth),
                Program::Swap => self.swap(),
                Program::Discard => self.discard(),
                Program::Slide(n) => self.slide(*n),
                Program::Addition => self.add(),
                Program::Subtraction => self.sub(),
                Program::Multiplication => self.mul(),
                Program::IntegerDivision => self.div(),
                Program::Modulo => self.modulo(),
                Program::Store => self.store(),
                Program::Retrieve => self.retrieve(),
                Program::Label(_) => Ok(()),
                Program::Call(label) => self.call_subroutine(label.clone()),
                Program::Jump(label) => self.jump(label.clone()),
                Program::JumpIfZero(label) => self.jump_if_zero(label.clone()),
                Program::JumpIfNotZero(label) => self.jump_if_not_zero(label.clone()),
                Program::Return => self.end_subroutine(),
                Program::End => self.halt(),
                Program::OutputChar => self.output_char(),
                Program::OutputInt => self.output_number(),
                Program::InputChar => self.input_char(),
                Program::InputInt => self.input_number(),
            };

            if let Err(e) = result {
                eprintln!("Runtime Error occurred: {}", e);
                self.dump();
                self.halt();
            }

            self.pc += 1;
            if self.pc >= self.program_list.len() {
                println!("");
                self.halt();
            }
        }
    }
}
