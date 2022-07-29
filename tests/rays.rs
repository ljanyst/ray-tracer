use ray_tracer::{point, vector, Ray};

#[test]
fn create_ray() {
    let origin = point(1.0, 2.0, 3.0);
    let direction = vector(4.0, 5.0, 6.0);
    let ray = Ray::new(origin, direction);
    assert_eq!(ray.origin(), origin);
    assert_eq!(ray.direction(), direction);
}

#[test]
fn compute_point_at_distance() {
    let ray = Ray::new(point(2.0, 3.0, 4.0), vector(1.0, 0.0, 0.0));
    assert_eq!(ray.position(0.0), point(2.0, 3.0, 4.0));
    assert_eq!(ray.position(1.0), point(3.0, 3.0, 4.0));
    assert_eq!(ray.position(-1.0), point(1.0, 3.0, 4.0));
    assert_eq!(ray.position(2.5), point(4.5, 3.0, 4.0));
}
