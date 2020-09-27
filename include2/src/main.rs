mod arithmetic ;

use crate::arithmetic::*;

fn main() {
    println!("Hello, this is struct, impl and modules example");

    let s = Sum::new(2, 2);
    println!("This is with sum method result from arithmetic module: {}", s.sum())
}
