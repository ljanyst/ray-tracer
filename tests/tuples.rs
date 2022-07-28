use ray_tracer::{color, feq, point, vector, Tuple};

#[test]
fn tuple_point() {
    let a = Tuple::new(4.3, -4.2, 3.1, 1.0);
    assert_eq!(4.3, a.x());
    assert_eq!(-4.2, a.y());
    assert_eq!(3.1, a.z());
    assert_eq!(1.0, a.w());
    assert!(a.is_point());
    assert!(!a.is_vector());
}

#[test]
fn tuple_vector() {
    let a = Tuple::new(4.3, -4.2, 3.1, 0.0);
    assert_eq!(4.3, a.x());
    assert_eq!(-4.2, a.y());
    assert_eq!(3.1, a.z());
    assert_eq!(0.0, a.w());
    assert!(!a.is_point());
    assert!(a.is_vector());
}

#[test]
fn create_point() {
    let a = point(4.0, -4.0, 3.0);
    assert_eq!(a, Tuple::new(4.0, -4.0, 3.0, 1.0));
}

#[test]
fn create_vector() {
    let a = vector(4.0, -4.0, 3.0);
    assert_eq!(a, Tuple::new(4.0, -4.0, 3.0, 0.0));
}

#[test]
fn add_vector_to_point() {
    let a1 = point(3.0, -2.0, 5.0);
    let a2 = vector(-2.0, 3.0, 1.0);
    assert_eq!(a1 + a2, point(1.0, 1.0, 6.0));
}

#[test]
fn subtract_two_points() {
    let p1 = point(3.0, 2.0, 1.0);
    let p2 = point(5.0, 6.0, 7.0);
    assert_eq!(p1 - p2, vector(-2.0, -4.0, -6.0));
}

#[test]
fn subtract_vector_from_point() {
    let p = point(3.0, 2.0, 1.0);
    let v = vector(5.0, 6.0, 7.0);
    assert_eq!(p - v, point(-2.0, -4.0, -6.0));
}

#[test]
fn subtract_two_vectors() {
    let v1 = vector(3.0, 2.0, 1.0);
    let v2 = vector(5.0, 6.0, 7.0);
    assert_eq!(v1 - v2, vector(-2.0, -4.0, -6.0));
}

#[test]
fn subtract_vector_from_zero_vector() {
    let zero = vector(0.0, 0.0, 0.0);
    let v = vector(1.0, -2.0, 3.0);
    assert_eq!(zero - v, vector(-1.0, 2.0, -3.0));
}

#[test]
fn negate_tuple() {
    let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(-a, Tuple::new(-1.0, 2.0, -3.0, 4.0));
}

#[test]
fn multiply_tuple_and_scalar() {
    let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(3.5 * a, Tuple::new(3.5, -7.0, 10.5, -14.0));
    assert_eq!(a * 3.5, Tuple::new(3.5, -7.0, 10.5, -14.0));
}

#[test]
fn multiply_tuple_by_fraction() {
    let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(0.5 * a, Tuple::new(0.5, -1.0, 1.5, -2.0));
    assert_eq!(a * 0.5, Tuple::new(0.5, -1.0, 1.5, -2.0));
}

#[test]
fn divide_tuple_by_scalar() {
    let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
    assert_eq!(a / 2.0, Tuple::new(0.5, -1.0, 1.5, -2.0));
}

#[test]
fn vector_norm() {
    let a = vector(1.0, 0.0, 0.0);
    assert!(feq(a.norm(), 1.0));
    let a = vector(0.0, 1.0, 0.0);
    assert!(feq(a.norm(), 1.0));
    let a = vector(0.0, 0.0, 1.0);
    assert!(feq(a.norm(), 1.0));
    let a = vector(1.0, 2.0, 3.0);
    assert!(feq(a.norm(), 14.0_f64.sqrt()));
    let a = vector(-1.0, -2.0, -3.0);
    assert!(feq(a.norm(), 14.0_f64.sqrt()));
}

#[test]
fn normalize_vector() {
    let a = vector(4.0, 0.0, 0.0);
    assert_eq!(a.normalized(), vector(1.0, 0.0, 0.0));
    let a = vector(1.0, 2.0, 3.0);
    let s = 14.0_f64.sqrt();
    assert_eq!(a.normalized(), vector(1.0 / s, 2.0 / s, 3.0 / s));
    assert!(feq(a.normalized().norm(), 1.0))
}

#[test]
fn vector_dot_product() {
    let a = vector(1.0, 2.0, 3.0);
    let b = vector(2.0, 3.0, 4.0);
    assert!(feq(a.dot(&b), 20.0))
}

#[test]
fn vector_cross_product() {
    let a = vector(1.0, 2.0, 3.0);
    let b = vector(2.0, 3.0, 4.0);
    assert_eq!(a.cross(&b), vector(-1.0, 2.0, -1.0));
    assert_eq!(b.cross(&a), vector(1.0, -2.0, 1.0));
}

#[test]
fn create_color() {
    let c = color(-0.5, 0.4, 1.7);
    assert_eq!(c.r(), -0.5);
    assert_eq!(c.g(), 0.4);
    assert_eq!(c.b(), 1.7);
}

#[test]
fn add_colors() {
    let c1 = color(0.9, 0.6, 0.75);
    let c2 = color(0.7, 0.1, 0.25);
    assert_eq!(c1 + c2, color(1.6, 0.7, 1.0));
}

#[test]
fn subtract_colors() {
    let c1 = color(0.9, 0.6, 0.75);
    let c2 = color(0.7, 0.1, 0.25);
    assert_eq!(c1 - c2, color(0.2, 0.5, 0.5));
}

#[test]
fn multiply_color_by_scalar() {
    let c = color(0.2, 0.3, 0.4);
    assert_eq!(2.0 * c, color(0.4, 0.6, 0.8));
    assert_eq!(c * 2.0, color(0.4, 0.6, 0.8));
}

#[test]
fn multiply_colors() {
    let c1 = color(1.0, 0.2, 0.4);
    let c2 = color(0.9, 1.0, 0.1);
    assert_eq!(c1.hadamard(&c2), color(0.9, 0.2, 0.04));
}
