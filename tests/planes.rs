use ray_tracer::Ray;
use ray_tracer::{feq, intersect, peq, plane_unit, point, vector};

#[test]
fn compute_plane_normal() {
    let p = plane_unit();
    let n1 = p.normal_at(point(0.0, 0.0, 0.0));
    let n2 = p.normal_at(point(10.0, 0.0, -10.0));
    let n3 = p.normal_at(point(-50.0, 0.0, 150.0));

    assert_eq!(n1, vector(0.0, 1.0, 0.0));
    assert_eq!(n2, vector(0.0, 1.0, 0.0));
    assert_eq!(n3, vector(0.0, 1.0, 0.0));
}

#[test]
fn incersect_ray_and_plane_parallel() {
    let r = Ray::new(point(0.0, 10.0, 0.0), vector(0.0, 0.0, 1.0));
    let p = plane_unit();
    let xs = intersect(&p, &r);
    assert_eq!(xs.len(), 0);
}

#[test]
fn incersect_ray_and_plane_coplanar() {
    let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
    let p = plane_unit();
    let xs = intersect(&p, &r);
    assert_eq!(xs.len(), 0);
}

#[test]
fn incersect_ray_above_and_plane() {
    let r = Ray::new(point(0.0, 1.0, 0.0), vector(0.0, -1.0, 0.0));
    let p = plane_unit();
    let xs = intersect(&p, &r);
    assert_eq!(xs.len(), 1);
    assert!(feq(xs[0].t(), 1.0));
    assert!(peq(xs[0].shape(), &p));
}

#[test]
fn incersect_ray_below_and_plane() {
    let r = Ray::new(point(0.0, -1.0, 0.0), vector(0.0, 1.0, 0.0));
    let p = plane_unit();
    let xs = intersect(&p, &r);
    assert_eq!(xs.len(), 1);
    assert!(feq(xs[0].t(), 1.0));
    assert!(peq(xs[0].shape(), &p));
}
