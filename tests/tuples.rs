use ray_tracer::{point, vector, Tuple};

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
