use ray_tracer::{feq, intersect, peq, point, vector, Ray, Shape, Sphere};
use ray_tracer::{Intersection, Intersections};

#[test]
fn create_intersection() {
    let s = Box::new(Sphere::unit()) as Box<dyn Shape>;
    let i = Intersection::new(3.5, &s);
    assert_eq!(i.t(), 3.5);
    assert!(peq(i.shape(), &s));
}

#[test]
fn create_intersections() {
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = Box::new(Sphere::unit()) as Box<dyn Shape>;
    let mut xs = Intersections::new();
    xs.append(intersect(&s, &r));
    assert_eq!(xs.len(), 2);
    assert!(feq(xs.at(0).t(), 4.0));
    assert!(peq(xs.at(0).shape(), &s));
    assert!(feq(xs.at(1).t(), 6.0));
    assert!(peq(xs.at(1).shape(), &s));
}

#[test]
fn create_intersections_from_a_vector() {
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = Box::new(Sphere::unit()) as Box<dyn Shape>;
    let xs = Intersections::from_vector(intersect(&s, &r));
    assert_eq!(xs.len(), 2);
    assert!(feq(xs.at(0).t(), 4.0));
    assert!(peq(xs.at(0).shape(), &s));
    assert!(feq(xs.at(1).t(), 6.0));
    assert!(peq(xs.at(1).shape(), &s));
}

#[test]
fn aggregate_intersections() {
    let s = Box::new(Sphere::unit()) as Box<dyn Shape>;
    let i1 = Intersection::new(1.0, &s);
    let i2 = Intersection::new(2.0, &s);
    let mut xs = Intersections::new();
    xs.push(i1);
    xs.push(i2);
    assert_eq!(xs.len(), 2);
    assert!(feq(xs.at(0).t(), 1.0));
    assert!(feq(xs.at(1).t(), 2.0));
}

#[test]
fn hit_intersection_all_positive() {
    let s = Box::new(Sphere::unit()) as Box<dyn Shape>;
    let mut xs = Intersections::new();
    let i1 = Intersection::new(1.0, &s);
    xs.push(i1);
    xs.push(Intersection::new(2.0, &s));
    assert_eq!(xs.hit(), Some(i1));
}

#[test]
fn hit_intersection_some_negative() {
    let s = Box::new(Sphere::unit()) as Box<dyn Shape>;
    let mut xs = Intersections::new();
    xs.push(Intersection::new(-1.0, &s));
    let i2 = Intersection::new(1.0, &s);
    xs.push(i2);
    assert_eq!(xs.hit(), Some(i2));
}

#[test]
fn hit_intersection_all_negative() {
    let s = Box::new(Sphere::unit()) as Box<dyn Shape>;
    let mut xs = Intersections::new();
    xs.push(Intersection::new(-2.0, &s));
    xs.push(Intersection::new(-1.0, &s));
    assert_eq!(xs.hit(), None);
}

#[test]
fn hit_intersection_unsorted() {
    let s = Box::new(Sphere::unit()) as Box<dyn Shape>;
    let mut xs = Intersections::new();
    xs.push(Intersection::new(5.0, &s));
    xs.push(Intersection::new(7.0, &s));
    xs.push(Intersection::new(-3.0, &s));
    let i4 = Intersection::new(2.0, &s);
    xs.push(i4);
    assert_eq!(xs.hit(), Some(i4));
}