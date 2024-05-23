use crate::vector::Vector;


pub fn cross_product(u: &Vector, v: &Vector) -> Vector{
    if u.size() != 3 {
        println!("Cross product of a vector with different dimensions from 3 cannot be computed");
        return Vector::new(v.size());
    }
    if u.size() != v.size(){
        println!("Vectors of a cross product have to be of the same size");
        return Vector::new(v.size());
    }
    let i_component: f32 = (u.array[1] * v.array[2]) - (u.array[2] * v.array[1]);
    let j_component: f32 = ((u.array[0] * v.array[2]) - (u.array[2] * v.array[0])) * -1.;
    let k_component: f32 = (u.array[0] * v.array[1]) - (u.array[1] * v.array[0]);
    return Vector::from(&[i_component, j_component, k_component]);
}