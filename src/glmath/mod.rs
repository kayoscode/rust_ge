pub mod rootable;

pub mod vec2;
pub mod vec3;
pub mod vec4;

pub use vec2::Vec2;
pub use vec3::Vec3;
pub use vec4::Vec4;

pub type Vec2f = Vec2<f32>;
pub type Vec3f = Vec3<f32>;
pub type Vec4f = Vec4<f32>;

pub trait StandardVec<T> {
    fn length(&self) -> T;
    fn length_sq(&self) -> T;

    fn normalize(&mut self) -> Self;
    fn get_normalized(&self) -> Self;
}

pub trait TwoDimVec<T: PartialOrd + Copy> 
{
    fn x(&self) -> &T;
    fn y(&self) -> &T;

    fn xy(&self) -> Vec2<T>;
    fn yx(&self) -> Vec2<T>;
}

pub trait ThreeDimVec<T: PartialOrd + Copy> : TwoDimVec<T> {
    fn z(&self) -> &T;

    fn xyz(&self) -> Vec3<T>;
    fn yxz(&self) -> Vec3<T>;
    fn zxy(&self) -> Vec3<T>;
    fn xzy(&self) -> Vec3<T>;
    fn yzx(&self) -> Vec3<T>;
    fn zyx(&self) -> Vec3<T>;
}

pub trait FourDimVec<T: PartialOrd + Copy> : ThreeDimVec<T> {
    fn w(&self) -> &T;

    fn xyzw(&self) -> Vec4<T>;
    fn yxzw(&self) -> Vec4<T>;
    fn zxyw(&self) -> Vec4<T>;
    fn xzyw(&self) -> Vec4<T>;
    fn yzxw(&self) -> Vec4<T>;
    fn zyxw(&self) -> Vec4<T>;

    fn zywx(&self) -> Vec4<T>;
    fn yzwx(&self) -> Vec4<T>;
    fn wzyx(&self) -> Vec4<T>;
    fn zwyx(&self) -> Vec4<T>;
    fn ywzx(&self) -> Vec4<T>;
    fn wyzx(&self) -> Vec4<T>;

    fn wxzy(&self) -> Vec4<T>;
    fn xwzy(&self) -> Vec4<T>;
    fn zwxy(&self) -> Vec4<T>;
    fn wzxy(&self) -> Vec4<T>;
    fn xzwy(&self) -> Vec4<T>;
    fn zxwy(&self) -> Vec4<T>;

    fn yxwz(&self) -> Vec4<T>;
    fn xywz(&self) -> Vec4<T>;
    fn wyxz(&self) -> Vec4<T>;
    fn ywxz(&self) -> Vec4<T>;
    fn xwyz(&self) -> Vec4<T>;
    fn wxyz(&self) -> Vec4<T>;
}