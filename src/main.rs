extern crate glmath;
use glmath::glmath::*;

fn test(a: &Quatf) {
    println!("{}", a);
}

fn main() {
    let a = Quatf::new(0.0, 0.0, 0.0, 1.0);
    test(&a);
}