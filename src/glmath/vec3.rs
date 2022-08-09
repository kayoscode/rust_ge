use crate::rootable::Rootable;
use crate::glmath::*;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Vec3<T: PartialOrd + Copy> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl
    <T: PartialOrd + Copy + Rootable<T> + 
        std::ops::Mul<Output = T> + 
        std::ops::Add<Output = T> + 
        std::ops::Div<Output = T> + 
        std::ops::DivAssign<T>> 
        StandardVec<T> for Vec3<T> 
{
    /// Computes the full length of the vector.
    fn length(&self) -> T {
        let len_sq: T = self.length_sq();
        len_sq.sqrt()
    }

    /// Computes the squared length of the vector2.
    fn length_sq(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn normalize(&mut self) -> Vec3<T> {
        let len = self.length();
        self.x /= len;
        self.y /= len;
        self.z /= len;

        *self
    }

    fn get_normalized(&self) -> Vec3<T> {
        let len = self.length();
        Vec3::<T>::new(self.x / len, self.y / len, self.z / len)
    }
}

impl<T: PartialOrd + Copy> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3::<T> { x: x, y: y, z: z}
    }
}

impl<T: PartialOrd + Copy> TwoDimVec<T> for Vec3<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }

    fn xy(&self) -> Vec2<T> {
        Vec2::<T>::new(self.x, self.y)
    }

    fn yx(&self) -> Vec2<T> {
        Vec2::<T>::new(self.y, self.x)
    }
}

impl<T: PartialOrd + Copy> ThreeDimVec<T> for Vec3<T> {
    fn z(&self) -> &T {
        &self.z
    }

    fn xyz(&self) -> Vec3<T> {
        Vec3::<T> { x: self.x, y: self.y, z: self.z }
    }

    fn yxz(&self) -> Vec3<T> {
        Vec3::<T> { x: self.y, y: self.x, z: self.z }
    }

    fn zxy(&self) -> Vec3<T> {
        Vec3::<T> { x: self.z, y: self.x, z: self.y }
    }

    fn xzy(&self) -> Vec3<T> {
        Vec3::<T> { x: self.x, y: self.z, z: self.y }
    }

    fn yzx(&self) -> Vec3<T> {
        Vec3::<T> { x: self.y, y: self.z, z: self.x }
    }

    fn zyx(&self) -> Vec3<T> {
        Vec3::<T> { x: self.z, y: self.y, z: self.x }
    }
}