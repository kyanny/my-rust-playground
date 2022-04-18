use std::io;
use std::io::BufRead;
// prelude is also OK
// use std::io::prelude::*;

fn main() {
    // https://doc.rust-lang.org/std/io/trait.BufRead.html#examples
    // https://doc.rust-lang.org/std/primitive.str.html#method.to_uppercase
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap().to_uppercase());
    }
}
