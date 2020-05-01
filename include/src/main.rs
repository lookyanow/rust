mod arithmetic ;

use crate::arithmetic::*;

fn main() {
    println!("Hello, this is struct, impl and modules example");
    // let first = A { a: 30, };
    // println!("{}", first.a);

    // let s = Sum{ a: 2, b: 2};

    // println!("This is scruct sum: {}", s.sum());

    let s = Sum::new(2, 2);
    println!("This is with constructor method: {}", s.sum())
}
