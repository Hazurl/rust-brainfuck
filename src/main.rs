mod command;
mod instruction;
mod interpreter;

use command::*;
use interpreter::*;

fn main() {
    let mut state = State::new(1 << 16);
    let is = parse("++++++++[->+<]>.").unwrap();
    let cs = compile(is);
    println!("{:?}", cs);
    state.execute(cs);
}
