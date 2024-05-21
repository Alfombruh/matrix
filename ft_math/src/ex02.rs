use crate::matrix::Matrix;
use crate::vector::Vector;

trait Lerp{
    fn lerp(start: Self, end: Self, t: f32) -> Self;
}

impl Lerp for Matrix{

}

impl Lerp for Vector{

}

impl Lerp for f32{

}