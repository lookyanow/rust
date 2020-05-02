mod arithmetic ;

use crate::arithmetic::*;

fn main() {
    println!("Hello, this is struct, impl and modules example");

    let s = Sum::new(2, 2);
    println!("This is with constructor method: {}", s.sum())
}
