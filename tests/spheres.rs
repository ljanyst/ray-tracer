use ray_tracer::{feq, point, vector, Ray, Sphere};

#[test]
fn intersect_ray_and_sphere() {
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = s.intersect(&r);
    assert_eq!(xs.len(), 2);
    assert!(feq(xs[0], 4.0));
    assert!(feq(xs[1], 6.0));
}

#[test]
fn intersect_ray_and_sphere_at_tangent() {
    let r = Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = s.intersect(&r);
    assert_eq!(xs.len(), 2);
    assert!(feq(xs[0], 5.0));
    assert!(feq(xs[1], 5.0));
}

#[test]
fn intersect_ray_and_sphere_no_intersection() {
    let r = Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = s.intersect(&r);
    assert_eq!(xs.len(), 0);
}

#[test]
fn intersect_ray_and_sphere_origin_inside() {
    let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = s.intersect(&r);
    assert_eq!(xs.len(), 2);
    assert!(feq(xs[0], -1.0));
    assert!(feq(xs[1], 1.0));
}

#[test]
fn intersect_ray_and_sphere_behind() {
    let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let xs = s.intersect(&r);
    assert_eq!(xs.len(), 2);
    assert!(feq(xs[0], -6.0));
    assert!(feq(xs[1], -4.0));
}
