pub mod vector;
pub mod matrix;
pub mod ex02;
use crate::ex02::Lerp;

//The function lerp that calls the implementation of lerp of the passed argument type
pub fn lerp<T: Lerp>(start: T, end: T, t: f32) -> T {
    T::lerp(start, end, t)
}