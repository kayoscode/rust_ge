mod glmath;

use glmath::*;

fn main() {
    let mut v1 = Vec3f::new(3.0, 4.0, 100.0);
    v1.normalize();

    println!("The length squared is: {}", v1.length_sq());
    println!("The length is: {}", v1.length());
    println!("{} {} {}", v1.x, v1.y, v1.z);
}