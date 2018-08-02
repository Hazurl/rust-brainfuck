#![feature(range_contains)]
#![feature(nll)]

mod command;
mod instruction;
mod interpreter;
mod optimizer;

use command::*;
use interpreter::*;
use optimizer::*;

fn main() {
    let mut state = State::new(1 << 16);
    let is = parse("+- > +++++++++ [- >> +++++++++ <<] >>.").unwrap();
    let better_is = remove_noop(compact(is));
    let cs = compile(better_is);
    println!("{:?}", cs);
    state.execute(cs);
}
