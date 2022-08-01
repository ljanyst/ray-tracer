use ray_tracer::{feq, point, vector, Ray, World};

#[test]
fn intersect_ray_with_world() {
    let w = World::default();
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let xs = w.intersect(&r);
    assert_eq!(xs.len(), 4);
    assert!(feq(xs.at(0).t(), 4.0));
    assert!(feq(xs.at(1).t(), 4.5));
    assert!(feq(xs.at(2).t(), 5.5));
    assert!(feq(xs.at(3).t(), 6.0));
}
