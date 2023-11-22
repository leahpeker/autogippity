mod my_funcs;
pub mod other_funcs;

use crate::my_funcs::add_five;
use crate::other_funcs::minus_funcs::sub_ten;

// Everything is defaulted to immutable
fn main() {
    let mut x: u32 = 50;
    println!("x is {}", x);

    let y: u32 = add_five(x);
    println!("y is {}", y);

    let z: u32 = sub_ten(y);
    println!("z is {}", z);

    x = 60;
    println!("x is {}", x);
}
