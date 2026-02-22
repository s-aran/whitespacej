mod cell;
mod imp;
mod parser;
mod vm;

use crate::parser::Parser;
use crate::vm::Machine;

#[derive(clap::Parser)]
struct Args {
    file: String,
}

fn main() {
    let args = <Args as clap::Parser>::parse();
    let source = std::fs::read_to_string(args.file).unwrap();

    let program = Parser::parse::<i64>(source).unwrap();

    let mut vm = Machine::new(program);
    vm.execute();
}
