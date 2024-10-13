mod vector;
use vector::mvec::{vec_ops, Vector};

fn main() {
    let vec1 = Vector::new(vec![23.0, 12.0, 5.0, 4.0, 5.0]);
    let vec2 = Vector::new(vec![-2.0, 4.0, 50.0, 6.0, -34.5]);

    let vec3 = (&vec1 + &vec2).unwrap();
    let vec4 = (&vec1 - &vec2).unwrap();

    println!("Vec1: {vec1}");
    println!("Vec2: {vec2}");

    println!("Dot Product: {}", vec_ops::dot(&vec1, &vec2).unwrap());
    println!("AXPY with alpha = -5: {}", vec_ops::axpy(&vec1, &vec2, -5.0).unwrap());
    println!("Vec1 + Vec2 = {vec3}");
    println!("Vec1 - Vec2 = {vec4}");
    println!("Vec1 == Vec2 -> {}", vec3 == vec4);

    let mut vec5 = Vector::new(vec![-243.0, 343.0, 1.0, 54.0, 22.0, -4.0]);
    println!("\nInitial Vec5: {vec5}");
    println!("Magnitude of Vec5: {}", vec5.magnitude());
    vec5.normalize();
    println!("Normalized Vec5: {vec5}");
    println!("Magnitude of Vec5: {}", vec5.magnitude());

}
