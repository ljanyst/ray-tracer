use ray_tracer::{
    feq, intersect, peq, plane_unit, point, scaling, sphere, sphere_glass, sphere_unit,
    translation, vector, Intersection, Intersections, Ray, EPSILON,
};

use std::f64::consts::{FRAC_1_SQRT_2, SQRT_2};

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
    xs.sort();
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
    let mut xs = Intersections::from_vector(intersect(s.as_ref(), &r));
    xs.sort();
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
    xs.sort();
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
    xs.sort();
    assert_eq!(xs.hit(), Some(&i1));
}

#[test]
fn hit_intersection_some_negative() {
    let s = sphere_unit();
    let mut xs = Intersections::new();
    xs.push(Intersection::new(-1.0, s.as_ref()));
    let i2 = Intersection::new(1.0, s.as_ref());
    xs.push(i2);
    xs.sort();
    assert_eq!(xs.hit(), Some(&i2));
}

#[test]
fn hit_intersection_all_negative() {
    let s = sphere_unit();
    let mut xs = Intersections::new();
    xs.push(Intersection::new(-2.0, s.as_ref()));
    xs.push(Intersection::new(-1.0, s.as_ref()));
    xs.sort();
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
    xs.sort();
    assert_eq!(xs.hit(), Some(&i4));
}

#[test]
fn compute_intersection_properties() {
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = sphere_unit();
    let i = Intersection::new(4.0, s.as_ref());
    let p = i.properties(&r, &Intersections::new());
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
    let p = i.properties(&r, &Intersections::new());
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
    let p = i.properties(&r, &Intersections::new());
    assert!(p.over_point.z() < -EPSILON / 2.0);
    assert!(p.point.z() > p.over_point.z());
}

#[test]
fn compute_intersection_properties_underpoint() {
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let mut s = sphere_glass();
    s.transform(translation(0.0, 0.0, 1.0));
    let i = Intersection::new(5.0, s.as_ref());
    let p = i.properties(&r, &Intersections::new());
    assert!(p.under_point.z() > EPSILON / 2.0);
    assert!(p.point.z() < p.under_point.z());
}

#[test]
fn compute_intersection_properties_reflection_vector() {
    let r = Ray::new(
        point(0.0, 0.1, -1.0),
        vector(0.0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
    );
    let p = plane_unit();
    let i = Intersection::new(SQRT_2, p.as_ref());
    let p = i.properties(&r, &Intersections::new());
    assert_eq!(p.reflectv, vector(0.0, FRAC_1_SQRT_2, FRAC_1_SQRT_2));
}

#[test]
fn compute_intersection_properties_refraction_indices() {
    let mut a = sphere_glass();
    a.transform(scaling(2.0, 2.0, 2.0));
    a.material_mut().refractive_index = 1.5;

    let mut b = sphere_glass();
    b.transform(translation(0.0, 0.0, -0.25));
    b.material_mut().refractive_index = 2.0;

    let mut c = sphere_glass();
    c.transform(translation(0.0, 0.0, 0.25));
    c.material_mut().refractive_index = 2.5;

    let r = Ray::new(point(0.0, 0.0, -4.0), vector(0.0, 0.0, 1.0));
    let mut xs = Intersections::new();
    xs.push(Intersection::new(2.0, a.as_ref()));
    xs.push(Intersection::new(2.75, b.as_ref()));
    xs.push(Intersection::new(3.25, c.as_ref()));
    xs.push(Intersection::new(4.75, b.as_ref()));
    xs.push(Intersection::new(5.25, c.as_ref()));
    xs.push(Intersection::new(6.0, a.as_ref()));

    struct TestData {
        pub index: usize,
        pub n1: f64,
        pub n2: f64,
    }

    impl TestData {
        pub fn new(i: usize, n1: f64, n2: f64) -> TestData {
            TestData { index: i, n1, n2 }
        }
    }

    let td = vec![
        TestData::new(0, 1.0, 1.5),
        TestData::new(1, 1.5, 2.0),
        TestData::new(2, 2.0, 2.5),
        TestData::new(3, 2.5, 2.5),
        TestData::new(4, 2.5, 1.5),
        TestData::new(5, 1.5, 1.0),
    ];

    for d in td.iter() {
        let props = xs.at(d.index).properties(&r, &xs);
        let (n1, n2) = props.refraction_indices;
        assert_eq!(n1, d.n1);
        assert_eq!(n2, d.n2);
    }
}
