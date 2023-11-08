mod lexicalanalyzer;
mod token;
mod parser;
mod source;

use std::io;
use crate::{source::Source, parser::parse_expression};

fn main() {
    let mut s = String::new();

    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line");
    let mut source = Source{next_pos: 0, raw: s};
    let value = parse_expression(&mut source);
    println!("value:{}",value);
}
