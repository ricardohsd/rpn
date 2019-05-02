extern crate rpn;

use std::io;
use rpn::Calculator;

fn main() {
    let mut expression = String::new();

    println!("Type a reversed polish notation:");
    io::stdin().read_line(&mut expression)
        .expect("Failed to read expression");

    let result = Calculator::run(&expression);

    match result {
        Ok(r) => println!("{}", r),
        Err(r) => println!("{}", r),
    }
}
