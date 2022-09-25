use ray_tracer::{feq, intersect, peq, point, sphere, sphere_unit, vector, Ray};
use ray_tracer::{translation, Intersection, Intersections, EPSILON};

#[test]
fn create_intersection() {
    let s = sphere_unit();
    let i = Intersection::new(3.5, s.as_ref());
    assert_eq!(i.t(), 3.5);
    assert!(peq(i.shape(), s.as_ref()));
}

#[test]
fn create_intersections() {
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = sphere_unit();
    let mut xs = Intersections::new();
    xs.append(intersect(s.as_ref(), &r));
    assert_eq!(xs.len(), 2);
    assert!(feq(xs.at(0).t(), 4.0));
    assert!(peq(xs.at(0).shape(), s.as_ref()));
    assert!(feq(xs.at(1).t(), 6.0));
    assert!(peq(xs.at(1).shape(), s.as_ref()));
}

#[test]
fn create_intersections_from_a_vector() {
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = sphere_unit();
    let xs = Intersections::from_vector(intersect(s.as_ref(), &r));
    assert_eq!(xs.len(), 2);
    assert!(feq(xs.at(0).t(), 4.0));
    assert!(peq(xs.at(0).shape(), s.as_ref()));
    assert!(feq(xs.at(1).t(), 6.0));
    assert!(peq(xs.at(1).shape(), s.as_ref()));
}

#[test]
fn aggregate_intersections() {
    let s = sphere_unit();
    let i1 = Intersection::new(1.0, s.as_ref());
    let i2 = Intersection::new(2.0, s.as_ref());
    let mut xs = Intersections::new();
    xs.push(i1);
    xs.push(i2);
    assert_eq!(xs.len(), 2);
    assert!(feq(xs.at(0).t(), 1.0));
    assert!(feq(xs.at(1).t(), 2.0));
}

#[test]
fn hit_intersection_all_positive() {
    let s = sphere_unit();
    let mut xs = Intersections::new();
    let i1 = Intersection::new(1.0, s.as_ref());
    xs.push(i1);
    xs.push(Intersection::new(2.0, s.as_ref()));
    assert_eq!(xs.hit(), Some(i1));
}

#[test]
fn hit_intersection_some_negative() {
    let s = sphere_unit();
    let mut xs = Intersections::new();
    xs.push(Intersection::new(-1.0, s.as_ref()));
    let i2 = Intersection::new(1.0, s.as_ref());
    xs.push(i2);
    assert_eq!(xs.hit(), Some(i2));
}

#[test]
fn hit_intersection_all_negative() {
    let s = sphere_unit();
    let mut xs = Intersections::new();
    xs.push(Intersection::new(-2.0, s.as_ref()));
    xs.push(Intersection::new(-1.0, s.as_ref()));
    assert_eq!(xs.hit(), None);
}

#[test]
fn hit_intersection_unsorted() {
    let s = sphere_unit();
    let mut xs = Intersections::new();
    xs.push(Intersection::new(5.0, s.as_ref()));
    xs.push(Intersection::new(7.0, s.as_ref()));
    xs.push(Intersection::new(-3.0, s.as_ref()));
    let i4 = Intersection::new(2.0, s.as_ref());
    xs.push(i4);
    assert_eq!(xs.hit(), Some(i4));
}

#[test]
fn compute_intersection_properties() {
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = sphere_unit();
    let i = Intersection::new(4.0, s.as_ref());
    let p = i.properties(&r);
    assert_eq!(p.t, i.t());
    assert!(peq(p.shape, i.shape()));
    assert_eq!(p.point, point(0.0, 0.0, -1.0));
    assert_eq!(p.eyev, vector(0.0, 0.0, -1.0));
    assert_eq!(p.normalv, vector(0.0, 0.0, -1.0));
    assert!(!p.inside);
}

#[test]
fn compute_intersection_properties_hit_inside() {
    let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
    let s = sphere_unit();
    let i = Intersection::new(1.0, s.as_ref());
    let p = i.properties(&r);
    assert_eq!(p.t, i.t());
    assert!(peq(p.shape, i.shape()));
    assert_eq!(p.point, point(0.0, 0.0, 1.0));
    assert_eq!(p.eyev, vector(0.0, 0.0, -1.0));
    assert_eq!(p.normalv, vector(0.0, 0.0, -1.0));
    assert!(p.inside);
}

#[test]
fn compute_intersection_properties_overpoint() {
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = sphere(translation(0.0, 0.0, 1.0));
    let i = Intersection::new(5.0, s.as_ref());
    let p = i.properties(&r);
    assert!(p.over_point.z() < -EPSILON / 2.0);
    assert!(p.point.z() > p.over_point.z());
}
