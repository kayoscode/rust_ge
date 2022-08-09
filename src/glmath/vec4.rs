use crate::rootable::Rootable;
use crate::glmath::*;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Vec4<T: PartialOrd + Copy> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T
}

impl
    <T: PartialOrd + Copy + Rootable<T> + 
        std::ops::Mul<Output = T> + 
        std::ops::Add<Output = T> +
        std::ops::Div<Output = T> + 
        std::ops::DivAssign<T>> 
        StandardVec<T> for Vec4<T> 
{
    /// Computes the full length of the vector.
    fn length(&self) -> T {
        let len_sq: T = self.length_sq();
        len_sq.sqrt()
    }

    /// Computes the squared length of the vector2.
    fn length_sq(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    fn normalize(&mut self) -> Vec4<T> {
        let len = self.length();
        self.x /= len;
        self.y /= len;
        self.z /= len;
        self.w /= len;

        *self
    }

    fn get_normalized(&self) -> Self {
        let len = self.length();

        Vec4::<T>::new(self.x / len, self.y / len, self.z / len, self.w / len)
    }
}

impl<T: PartialOrd + Copy> Vec4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Vec4<T> {
        Vec4::<T> { x: x, y: y, z: z, w: w }
    }
}

impl<T: PartialOrd + Copy> TwoDimVec<T> for Vec4<T> {
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

impl<T: PartialOrd + Copy> ThreeDimVec<T> for Vec4<T> {
    fn z(&self) -> &T {
        &self.z
    }

    fn xyz(&self) -> Vec3<T> {
        Vec3::<T>::new(self.x, self.y, self.z)
    }

    fn yxz(&self) -> Vec3<T> {
        Vec3::<T>::new(self.y, self.x, self.z)
    }

    fn zxy(&self) -> Vec3<T> {
        Vec3::<T>::new(self.z, self.x, self.y)
    }

    fn xzy(&self) -> Vec3<T> {
        Vec3::<T>::new(self.x, self.z, self.y)
    }

    fn yzx(&self) -> Vec3<T> {
        Vec3::<T>::new(self.y, self.z, self.x)
    }

    fn zyx(&self) -> Vec3<T> {
        Vec3::<T>::new(self.z, self.y, self.x)
    }
}

impl<T: PartialOrd + Copy> FourDimVec<T> for Vec4<T> {
    fn w(&self) -> &T {
        return &self.w;
    }

    fn xyzw(&self) -> Vec4<T> {
        Vec4::<T> { x: self.x, y: self.y, z: self.z, w: self.w }
    }

    fn yxzw(&self) -> Vec4<T> {
        Vec4::<T> { x: self.y, y: self.x, z: self.z, w: self.w }
    }

    fn zxyw(&self) -> Vec4<T> {
        Vec4::<T> { x: self.z, y: self.x, z: self.y, w: self.w }
    }

    fn xzyw(&self) -> Vec4<T> {
        Vec4::<T> { x: self.x, y: self.z, z: self.y, w: self.w }
    }

    fn yzxw(&self) -> Vec4<T> {
        Vec4::<T> { x: self.y, y: self.z, z: self.x, w: self.w }
    }

    fn zyxw(&self) -> Vec4<T> {
        Vec4::<T> { x: self.z, y: self.y, z: self.x, w: self.w }
    }

    fn zywx(&self) -> Vec4<T> {
        Vec4::<T> { x: self.z, y: self.y, z: self.w, w: self.x }
    }

    fn yzwx(&self) -> Vec4<T> {
        Vec4::<T> { x: self.y, y: self.z, z: self.w, w: self.x }
    }

    fn wzyx(&self) -> Vec4<T> {
        Vec4::<T> { x: self.w, y: self.z, z: self.y, w: self.x }
    }

    fn zwyx(&self) -> Vec4<T> {
        Vec4::<T> { x: self.w, y: self.z, z: self.y, w: self.x }
    }

    fn ywzx(&self) -> Vec4<T> {
        Vec4::<T> { x: self.y, y: self.w, z: self.z, w: self.x }
    }

    fn wyzx(&self) -> Vec4<T> {
        Vec4::<T> { x: self.w, y: self.y, z: self.z, w: self.x }
    }

    fn wxzy(&self) -> Vec4<T> {
        Vec4::<T> { x: self.w, y: self.x, z: self.z, w: self.y }
    }

    fn xwzy(&self) -> Vec4<T> {
        Vec4::<T> { x: self.x, y: self.w, z: self.z, w: self.y }
    }

    fn zwxy(&self) -> Vec4<T> {
        Vec4::<T> { x: self.z, y: self.w, z: self.x, w: self.y }
    }

    fn wzxy(&self) -> Vec4<T> {
        Vec4::<T> { x: self.w, y: self.z, z: self.x, w: self.y }
    }

    fn xzwy(&self) -> Vec4<T> {
        Vec4::<T> { x: self.x, y: self.z, z: self.w, w: self.y }
    }

    fn zxwy(&self) -> Vec4<T> {
        Vec4::<T> { x: self.z, y: self.x, z: self.w, w: self.y }
    }

    fn yxwz(&self) -> Vec4<T> {
        Vec4::<T> { x: self.y, y: self.x, z: self.w, w: self.z }
    }

    fn xywz(&self) -> Vec4<T> {
        Vec4::<T> { x: self.x, y: self.y, z: self.w, w: self.z }
    }

    fn wyxz(&self) -> Vec4<T> {
        Vec4::<T> { x: self.w, y: self.y, z: self.x, w: self.z }
    }

    fn ywxz(&self) -> Vec4<T> {
        Vec4::<T> { x: self.y, y: self.w, z: self.x, w: self.z }
    }

    fn xwyz(&self) -> Vec4<T> {
        Vec4::<T> { x: self.x, y: self.w, z: self.y, w: self.z }
    }

    fn wxyz(&self) -> Vec4<T> {
        Vec4::<T> { x: self.w, y: self.x, z: self.y, w: self.z }
    }
}