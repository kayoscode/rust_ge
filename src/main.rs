extern crate glmath;
use glmath::*;

fn main() {
    let a = Vec3f::new(10.0, 5.0, 2.0);
    let b = Vec3f::new(2.0, 1.0, 3.0);

    println!("{} {}", a, b);
    dbg!(a % b);
    dbg!(b % a);
}