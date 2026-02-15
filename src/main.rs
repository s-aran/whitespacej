mod cell;
mod imp;
mod parser;

use std::{collections::HashMap, hash::Hash};

use crate::parser::Parser;
use crate::{cell::Cell, imp::Data};

trait CharType<C: Sized> {
    fn input_char() -> C;
    fn output_char(char: C);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Char {
    value: char,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Byte<T: Cell> {
    value: T,
}

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

    pub fn swap(&mut self) {
        if self.stack.is_empty() {
            return;
        }

        if self.stack.len() == 1 {
            self.stack.insert(0, T::zero());
        }

        let a = self.stack.len() - 1;
        let b = a - 1;
        self.stack.swap(a, b);
    }
}

struct Heap<K: Cell + Hash, V: Cell> {
    map: HashMap<K, Data<V>>,
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
    pub fn get(&self, key: &K) -> &Data<V> {
        self.map.get(key).unwrap()
    }

    #[inline]
    pub fn get_mut(&mut self, key: &K) -> &mut Data<V> {
        self.map.get_mut(key).unwrap()
    }

    #[inline]
    pub fn set(&mut self, key: K, value: Data<V>) {
        self.map.insert(key, value);
    }
}

struct Machine<T: Cell> {
    stack: Stack<T>,
    heap: Heap<T, T>,
}

#[derive(clap::Parser)]
struct Args {
    file: String,
}

fn main() {
    let args = <Args as clap::Parser>::parse();
    let source = std::fs::read_to_string(args.file).unwrap();

    let program = Parser::parse::<i64>(source);

    println!("{:#?}", program);
}
