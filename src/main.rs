
#[macro_use]
extern crate nom;

mod parse;
mod eval;

use parse::*;
use eval::*;

fn main() {
	println!("{:?}", prog_parse(b" as principle  admin password toor do \nexit\n"));
}