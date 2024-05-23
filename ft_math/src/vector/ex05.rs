use crate::vector::Vector;

//As the dot product of two vectors is the product of their norms times the cosine of angle they make
//You can get the value of its cosine, by dividing their dot product between the product of their modules
pub fn angle_cos(u: &Vector, v: &Vector) -> f32{
    let dot_product: f32 = u.dot(v.clone());
    let angle: f32 = dot_product / (u.clone().norm() * v.clone().norm());
    return angle;
}