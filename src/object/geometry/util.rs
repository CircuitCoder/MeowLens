use nalgebra::*;
use super::triangle::Triangle;

pub fn create_box(lower: Vector3<f64>, upper: Vector3<f64>) -> Vec<Triangle> {
    // Creater upper face

    let upper_a = Triangle::new([
        Vector3::new(lower[0], upper[1], lower[2]),
        Vector3::new(upper[0], upper[1], lower[2]),
        Vector3::new(lower[0], upper[1], upper[2]),
    ]);

    let upper_b = Triangle::new([
        Vector3::new(upper[0], upper[1], lower[2]),
        Vector3::new(upper[0], upper[1], upper[2]),
        Vector3::new(lower[0], upper[1], upper[2]),
    ]);


    // Create bottom face

    let lower_a = Triangle::new([
        Vector3::new(upper[0], lower[1], lower[2]),
        Vector3::new(lower[0], lower[1], lower[2]),
        Vector3::new(lower[0], lower[1], upper[2]),
    ]);

    let lower_b = Triangle::new([
        Vector3::new(upper[0], lower[1], upper[2]),
        Vector3::new(upper[0], lower[1], lower[2]),
        Vector3::new(lower[0], lower[1], upper[2]),
    ]);

    // Create front face

    let front_a = Triangle::new([
        Vector3::new(lower[0], lower[1], lower[2]),
        Vector3::new(lower[0], upper[1], lower[2]),
        Vector3::new(lower[0], lower[1], upper[2]),
    ]);

    let front_b = Triangle::new([
        Vector3::new(lower[0], upper[1], lower[2]),
        Vector3::new(lower[0], upper[1], upper[2]),
        Vector3::new(lower[0], lower[1], upper[2]),
    ]);

    // Back

    let back_a = Triangle::new([
        Vector3::new(upper[0], upper[1], lower[2]),
        Vector3::new(upper[0], lower[1], lower[2]),
        Vector3::new(upper[0], lower[1], upper[2]),
    ]);


    let back_b = Triangle::new([
        Vector3::new(upper[0], upper[1], upper[2]),
        Vector3::new(upper[0], upper[1], lower[2]),
        Vector3::new(upper[0], lower[1], upper[2]),
    ]);

    // Left

    let left_a = Triangle::new([
        Vector3::new(upper[0], lower[1], lower[2]),
        Vector3::new(upper[0], upper[1], lower[2]),
        Vector3::new(lower[0], lower[1], lower[2]),
    ]);

    let left_b = Triangle::new([
        Vector3::new(upper[0], upper[1], lower[2]),
        Vector3::new(lower[0], upper[1], lower[2]),
        Vector3::new(lower[0], lower[1], lower[2]),
    ]);

    // Right

    let right_a = Triangle::new([
        Vector3::new(upper[0], upper[1], upper[2]),
        Vector3::new(upper[0], lower[1], upper[2]),
        Vector3::new(lower[0], lower[1], upper[2]),
    ]);

    let right_b = Triangle::new([
        Vector3::new(lower[0], upper[1], upper[2]),
        Vector3::new(upper[0], upper[1], upper[2]),
        Vector3::new(lower[0], lower[1], upper[2]),
    ]);

    vec![
        upper_a,
        upper_b,
        lower_a,
        lower_b,
        front_a,
        front_b,
        back_a,
        back_b,
        left_a,
        left_b,
        right_a,
        right_b,
    ]
}
