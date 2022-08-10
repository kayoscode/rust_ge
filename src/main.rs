extern crate glmath;
use glmath::*;

fn main() {
    let a = Quatf::new(0.0, 0.0, 0.0, 1.0);
    dbg!(a);

    dbg!(a.length());
}