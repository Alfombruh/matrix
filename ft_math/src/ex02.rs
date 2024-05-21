use crate::matrix::Matrix;
use crate::vector::Vector;

pub trait Lerp{
    fn lerp(start: Self, end: Self, t: f32) -> Self;
}

impl Lerp for Matrix{
    fn lerp(u: Matrix, v: Matrix, t: f32) -> Matrix{
        if t > 1. || t < 0. {
            println!("t must be a value between [0,1]");
        }
        if u.shape() != v.shape() {
            println!("u and v have different sizes");
        }
        let mut u_scaled: Matrix = u.clone();
        u_scaled.scl(1. - t);
        let mut v_scaled: Matrix = v.clone();
        v_scaled.scl(t);
        u_scaled.add(&v_scaled);
        return u_scaled.clone();
    }

}

impl Lerp for Vector{
    fn lerp(u: Vector, v: Vector, t: f32) -> Vector{
        if t > 1. || t < 0. {
            println!("t must be a value between [0,1]");
        }
        if u.size() != v.size() {
            println!("u and v have different sizes");
        }
        let mut u_scaled: Vector = u.clone();
        let mut v_scaled: Vector = v.clone();
        u_scaled.scl(1. - t);
        v_scaled.scl(t);
        u_scaled.add(&v_scaled);
        return u_scaled.clone();
    }
}

impl Lerp for f32{
    fn lerp(u: f32, v: f32, t:f32) -> f32{
        if t > 1. || t < 0. {
            println!("t must be a value between [0,1]");
        }
        return (u * (1. - t)) + (v * t);
    }
}