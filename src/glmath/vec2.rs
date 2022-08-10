use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Neg};
use std::process::Output;

use crate::vectorable::Vectorable;
use crate::glmath::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec2<T: PartialOrd + Copy> {
    pub x: T,
    pub y: T
}

impl
    <T: PartialOrd + Copy + Vectorable<T> + 
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

    fn normalize(&mut self) {
        let len = self.length();
        self.x /= len;
        self.y /= len;
    }

    fn get_normalized(&self) -> Vec2<T> {
        let len = self.length();
        Vec2::<T>::new(self.x / len, self.y / len)
    }
}

impl<T: PartialOrd + Copy> Vec2<T> {
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2::<T> { x, y }
    }
}

impl<T: PartialOrd + Copy + Vectorable<T>> PartialEq for Vec2<T> 
    where Vec2<T>: StandardVec<T>,
    T: Mul<Output = T> + Div<Output = T>
{
    fn eq(&self, other: &Self) -> bool {
        self.length() == other.length()
    }
}

impl<T: PartialOrd + Copy + Vectorable<T>> PartialOrd for Vec2<T> 
    where Vec2<T>: StandardVec<T>,
    T: Mul<Output = T> + Div<Output = T>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let len = Vec2::<T>::length(&self);
        let other_len = Vec2::<T>::length(&other);

        len.partial_cmp(&other_len)
    }
}

impl<T: PartialOrd + Copy + std::ops::Neg<Output = T>> Neg for Vec2<T> {
    type Output = Vec2<T>;

    fn neg(self) -> Self::Output {
        Vec2::<T> { x: -self.x, y: -self.y }
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

// Dot product.
impl<T: PartialOrd + Copy + 
    std::ops::Mul<Output = T> +
    std::ops::Add<Output = T>> Mul<Vec2<T>> for Vec2<T> 
{
    type Output = T;

    // Computes and returns the dot product as a scalar.
    fn mul(self, rhs: Vec2<T>) -> T {
        self.x * rhs.x + self.y * rhs.y
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
        self.clone()
    }

    fn yx(&self) -> Vec2<T> {
        Vec2::<T> { x: self.y, y: self.x }
    }
}