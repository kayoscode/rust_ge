pub trait Rootable<T> {
    fn sqrt(&self) -> T;
}

impl Rootable<f32> for f32 {
    fn sqrt(&self) -> f32 {
        f32::sqrt(*self)
    }
}

impl Rootable<f64> for f64 {
    fn sqrt(&self) -> f64 {
        f64::sqrt(*self)
    }
}

impl Rootable<i32> for i32 {
    fn sqrt(&self) -> i32 {
        f64::sqrt(*self as f64) as i32
    }
}

impl Rootable<u32> for u32 {
    fn sqrt(&self) -> u32 {
        f64::sqrt(*self as f64) as u32
    }
}

impl Rootable<i64> for i64 {
    fn sqrt(&self) -> i64 {
        f64::sqrt(*self as f64) as i64
    }
}

impl Rootable<u64> for u64 {
    fn sqrt(&self) -> u64 {
        f64::sqrt(*self as f64) as u64
    }
}