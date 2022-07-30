use ray_tracer::{feq, intersect, peq, point, vector, Ray, Shape, Sphere};
use ray_tracer::{scaling, translation, Matrix};

#[test]
fn intersect_ray_and_sphere() {
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = Box::new(Sphere::unit()) as Box<dyn Shape>;
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
    let s = Box::new(Sphere::unit()) as Box<dyn Shape>;
    let xs = intersect(&s, &r);
    assert_eq!(xs.len(), 2);
    assert!(feq(xs[0].t(), 5.0));
    assert!(feq(xs[1].t(), 5.0));
}

#[test]
fn intersect_ray_and_sphere_no_intersection() {
    let r = Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = Box::new(Sphere::unit()) as Box<dyn Shape>;
    let xs = intersect(&s, &r);
    assert_eq!(xs.len(), 0);
}

#[test]
fn intersect_ray_and_sphere_origin_inside() {
    let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
    let s = Box::new(Sphere::unit()) as Box<dyn Shape>;
    let xs = intersect(&s, &r);
    assert_eq!(xs.len(), 2);
    assert!(feq(xs[0].t(), -1.0));
    assert!(feq(xs[1].t(), 1.0));
}

#[test]
fn intersect_ray_and_sphere_behind() {
    let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
    let s = Box::new(Sphere::unit()) as Box<dyn Shape>;
    let xs = intersect(&s, &r);
    assert_eq!(xs.len(), 2);
    assert!(feq(xs[0].t(), -6.0));
    assert!(feq(xs[1].t(), -4.0));
}

#[test]
fn construct_unit_sphere() {
    let s = Sphere::unit();
    assert_eq!(*s.current_transform(), Matrix::one());
}

#[test]
fn move_unit_sphere() {
    let mut s = Sphere::unit();
    let t = translation(2.0, 3.0, 4.0);
    s.transform(t);
    assert_eq!(*s.current_transform(), t);
}

#[test]
fn intersect_ray_and_scaled_sphere() {
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let mut s = Box::new(Sphere::unit()) as Box<dyn Shape>;
    s.transform(scaling(2.0, 2.0, 2.0));
    let xs = intersect(&s, &r);
    assert_eq!(xs.len(), 2);
    assert!(feq(xs[0].t(), 3.0));
    assert!(feq(xs[1].t(), 7.0));
}

#[test]
fn intersect_ray_and_translated_sphere() {
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let mut s = Box::new(Sphere::unit()) as Box<dyn Shape>;
    s.transform(translation(5.0, 0.0, 0.0));
    let xs = intersect(&s, &r);
    assert_eq!(xs.len(), 0);
}
