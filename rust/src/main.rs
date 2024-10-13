mod vector;
use vector::MVector;

fn main() {
    let vec1 = MVector::new(vec![4, 34, 2, 1, 5]);
    let vec2 = MVector::new(vec![34, 2, -2, 35, 4]);

    let vec3 = (&vec1 + &vec2).unwrap();
    let axpy = vec1.axpy(5, &vec2).unwrap();

    println!("Vec1: {vec1}, \nVec2: {vec2}, \nVec1 + Vec2 = {vec3}");
    println!("Vec1 * 5 + Vec2 = {axpy}");

}
