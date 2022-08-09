use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign};

use crate::rootable::Rootable;
use crate::glmath::*;

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Vec2<T: PartialOrd + Copy> {
    pub x: T,
    pub y: T
}

impl
    <T: PartialOrd + Copy + Rootable<T> + 
        std::ops::Mul<Output = T> + 
        std::ops::Add<Output = T> + 
        std::ops::Div<Output = T> +
        std::ops::DivAssign<T>>
        StandardVec<T> for Vec2<T> 
{
    /// Computes the full length of the vector.
    fn length(&self) -> T {
        let len_sq: T = self.length_sq();
        len_sq.sqrt()
    }

    /// Computes the squared length of the vector2.
    fn length_sq(&self) -> T {
        self.x * self.x + self.y * self.y
    }

    fn normalize(&mut self) -> Vec2<T> {
        let len = self.length();
        self.x /= len;
        self.y /= len;
        *self
    }

    fn get_normalized(&self) -> Vec2<T> {
        let len = self.length();
        Vec2::<T>::new(self.x / len, self.y / len)
    }
}

impl<T: PartialOrd + Copy> Vec2<T> {
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2::<T> { x: x, y: y }
    }
}

impl<T: PartialOrd + Copy + std::ops::Add<Output = T>> Add<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: Vec2<T>) -> Vec2<T> {
        let x: T = self.x + rhs.x;
        let y: T = self.y + rhs.y;

        Vec2::<T>::new(x, y)
    }
}

impl<T: PartialOrd + Copy + std::ops::AddAssign<T>> AddAssign<Vec2<T>> for Vec2<T> {
    fn add_assign(&mut self, rhs: Vec2<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: PartialOrd + Copy + std::ops::Sub<Output = T>> Sub<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn sub(self, rhs: Vec2<T>) -> Vec2<T> {
        let x: T = self.x - rhs.x;
        let y: T = self.y - rhs.y;

        Vec2::<T>::new(x, y)
    }
}

impl<T: PartialOrd + Copy + std::ops::SubAssign<T>> SubAssign<Vec2<T>> for Vec2<T> {
    fn sub_assign(&mut self, rhs: Vec2<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: PartialOrd + Copy + std::ops::Mul<Output = T>> Mul<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2::<T>::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl<T: PartialOrd + Copy + std::ops::MulAssign<T>> MulAssign<Vec2<T>> for Vec2<T> {
    fn mul_assign(&mut self, rhs: Vec2<T>) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

/// Scales the vector by a scalar value.
impl<T: PartialOrd + Copy + std::ops::Mul<Output = T>> Mul<T> for Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: T) -> Vec2<T>{
        Vec2::<T>::new(self.x * rhs, self.y * rhs)
    }
}

impl<T: PartialOrd + Copy + std::ops::MulAssign<T>> MulAssign<T> for Vec2<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T: PartialOrd + Copy +
    std::ops::Div<Output = T>> TwoDimVec<T> for Vec2<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }

    fn xy(&self) -> Vec2<T> {
        *self
    }

    fn yx(&self) -> Vec2<T> {
        Vec2::<T> { x: self.y, y: self.x }
    }
}