mod glmath;

use glmath::*;

fn main() {
    let mut v1 = Vec2f::new(0.0, 1.0);
    let mut v2 = Vec2f::new(1.0, 0.0);

    dbg!(v1);
    dbg!(v2);

    println!("Angle between: {}", v1.angle_between(&v2));

    if v1 == v2 {
        println!("{} {}", v1.length(), v2.length());
    }
}