mod command;
mod instruction;
mod interpreter;

use command::*;
use instruction::*;
use interpreter::*;

fn main() {
    let mut state = State::new(1 << 16);
    let is = parse("[[+++]]").unwrap();
    //state.execute(is);
    println!("{:?}", is);
    let cs = compile(is);
    println!("{:?}", cs);
}
