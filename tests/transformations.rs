use ray_tracer::{point, scaling, shearing, translation, vector};
use ray_tracer::{rotation_x, rotation_y, rotation_z};

use std::f64::consts::PI;

#[test]
fn apply_translation_matrix_to_point() {
    let t = translation(5.0, -3.0, 2.0);
    let p = point(-3.0, 4.0, 5.0);
    assert_eq!(t * p, point(2.0, 1.0, 7.0));
}

#[test]
fn apply_inverse_translation_matrix_to_point() {
    let t = translation(5.0, -3.0, 2.0);
    let p = point(-3.0, 4.0, 5.0);
    assert_eq!(t.inverted() * p, point(-8.0, 7.0, 3.0));
}

#[test]
fn apply_translation_matrix_to_vector() {
    let t = translation(5.0, -3.0, 2.0);
    let v = vector(-3.0, 4.0, 5.0);
    assert_eq!(t * v, v);
}

#[test]
fn apply_scaling_matrix_to_point() {
    let t = scaling(2.0, 3.0, 4.0);
    let p = point(-4.0, 6.0, 8.0);
    assert_eq!(t * p, point(-8.0, 18.0, 32.0));
}

#[test]
fn apply_scaling_matrix_to_vector() {
    let t = scaling(2.0, 3.0, 4.0);
    let v = vector(-4.0, 6.0, 8.0);
    assert_eq!(t * v, vector(-8.0, 18.0, 32.0));
}

#[test]
fn apply_inverse_scaling_matrix_to_vector() {
    let t = scaling(2.0, 3.0, 4.0);
    let v = vector(-4.0, 6.0, 8.0);
    assert_eq!(t.inverted() * v, vector(-2.0, 2.0, 2.0));
}

#[test]
fn apply_reflection_scaling_matrix_to_point() {
    let t = scaling(-1.0, 1.0, 1.0);
    let p = point(2.0, 3.0, 4.0);
    assert_eq!(t * p, point(-2.0, 3.0, 4.0));
}

#[test]
fn apply_rotation_x_matrix_to_point() {
    let p = point(0.0, 1.0, 0.0);
    let half_qarter = rotation_x(PI / 4.0);
    let full_qarter = rotation_x(PI / 2.0);
    assert_eq!(
        half_qarter * p,
        point(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
    );
    assert_eq!(full_qarter * p, point(0.0, 0.0, 1.0));
}

#[test]
fn apply_inverse_rotation_x_matrix_to_point() {
    let p = point(0.0, 1.0, 0.0);
    let half_qarter = rotation_x(PI / 4.0);
    assert_eq!(
        half_qarter.inverted() * p,
        point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0)
    );
}

#[test]
fn apply_rotation_y_matrix_to_point() {
    let p = point(0.0, 0.0, 1.0);
    let half_qarter = rotation_y(PI / 4.0);
    let full_qarter = rotation_y(PI / 2.0);
    assert_eq!(
        half_qarter * p,
        point(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0)
    );
    assert_eq!(full_qarter * p, point(1.0, 0.0, 0.0));
}

#[test]
fn apply_rotation_z_matrix_to_point() {
    let p = point(0.0, 1.0, 0.0);
    let half_qarter = rotation_z(PI / 4.0);
    let full_qarter = rotation_z(PI / 2.0);
    assert_eq!(
        half_qarter * p,
        point(-2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0)
    );
    assert_eq!(full_qarter * p, point(-1.0, 0.0, 0.0));
}

#[test]
fn apply_shearing_matrix_to_point() {
    let p = point(2.0, 3.0, 4.0);
    assert_eq!(
        shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0) * p,
        point(5.0, 3.0, 4.0)
    );
    assert_eq!(
        shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0) * p,
        point(6.0, 3.0, 4.0)
    );
    assert_eq!(
        shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0) * p,
        point(2.0, 5.0, 4.0)
    );
    assert_eq!(
        shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0) * p,
        point(2.0, 7.0, 4.0)
    );
    assert_eq!(
        shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0) * p,
        point(2.0, 3.0, 6.0)
    );
    assert_eq!(
        shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0) * p,
        point(2.0, 3.0, 7.0)
    );
}

#[test]
fn apply_chain_of_transformations_to_point() {
    let p = point(1.0, 0.0, 1.0);
    let rot = rotation_x(PI / 2.0);
    let sc = scaling(5.0, 5.0, 5.0);
    let tr = translation(10.0, 5.0, 7.0);
    assert_eq!(tr * sc * rot * p, point(15.0, 0.0, 7.0));
}