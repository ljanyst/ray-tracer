use ray_tracer::{feq, intersect, peq, point, vector, Ray, Shape, Sphere};

#[test]
fn intersect_ray_and_sphere() {
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = Box::new(Sphere::new()) as Box<dyn Shape>;
    let xs = intersect(&s, &r);
    assert_eq!(xs.len(), 2);
    assert!(feq(xs[0].t(), 4.0));
    assert!(peq(xs[0].shape(), &s));
    assert!(feq(xs[1].t(), 6.0));
    assert!(peq(xs[1].shape(), &s));
}

#[test]
fn intersect_ray_and_sphere_at_tangent() {
    let r = Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = Box::new(Sphere::new()) as Box<dyn Shape>;
    let xs = intersect(&s, &r);
    assert_eq!(xs.len(), 2);
    assert!(feq(xs[0].t(), 5.0));
    assert!(feq(xs[1].t(), 5.0));
}

#[test]
fn intersect_ray_and_sphere_no_intersection() {
    let r = Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = Box::new(Sphere::new()) as Box<dyn Shape>;
    let xs = intersect(&s, &r);
    assert_eq!(xs.len(), 0);
}

#[test]
fn intersect_ray_and_sphere_origin_inside() {
    let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
    let s = Box::new(Sphere::new()) as Box<dyn Shape>;
    let xs = intersect(&s, &r);
    assert_eq!(xs.len(), 2);
    assert!(feq(xs[0].t(), -1.0));
    assert!(feq(xs[1].t(), 1.0));
}

#[test]
fn intersect_ray_and_sphere_behind() {
    let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
    let s = Box::new(Sphere::new()) as Box<dyn Shape>;
    let xs = intersect(&s, &r);
    assert_eq!(xs.len(), 2);
    assert!(feq(xs[0].t(), -6.0));
    assert!(feq(xs[1].t(), -4.0));
}
