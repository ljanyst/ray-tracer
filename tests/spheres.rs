use ray_tracer::{
    feq, intersect, peq, point, rotation_z, scaling, sphere, sphere_glass, sphere_unit,
    translation, vector, Material, Matrix, Ray,
};

use std::f64::consts::{FRAC_1_SQRT_2, PI};

#[test]
fn intersect_ray_and_sphere() {
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = sphere_unit();
    let xs = intersect(s.as_ref(), &r);
    assert_eq!(xs.len(), 2);
    assert!(feq(xs[0].t(), 4.0));
    assert!(peq(xs[0].shape(), s.as_ref()));
    assert!(feq(xs[1].t(), 6.0));
    assert!(peq(xs[1].shape(), s.as_ref()));
}

#[test]
fn intersect_ray_and_sphere_at_tangent() {
    let r = Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = sphere_unit();
    let xs = intersect(s.as_ref(), &r);
    assert_eq!(xs.len(), 2);
    assert!(feq(xs[0].t(), 5.0));
    assert!(feq(xs[1].t(), 5.0));
}

#[test]
fn intersect_ray_and_sphere_no_intersection() {
    let r = Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
    let s = sphere_unit();
    let xs = intersect(s.as_ref(), &r);
    assert_eq!(xs.len(), 0);
}

#[test]
fn intersect_ray_and_sphere_origin_inside() {
    let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
    let s = sphere_unit();
    let xs = intersect(s.as_ref(), &r);
    assert_eq!(xs.len(), 2);
    assert!(feq(xs[0].t(), -1.0));
    assert!(feq(xs[1].t(), 1.0));
}

#[test]
fn intersect_ray_and_sphere_behind() {
    let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
    let s = sphere_unit();
    let xs = intersect(s.as_ref(), &r);
    assert_eq!(xs.len(), 2);
    assert!(feq(xs[0].t(), -6.0));
    assert!(feq(xs[1].t(), -4.0));
}

#[test]
fn construct_unit_sphere() {
    let s = sphere_unit();
    assert_eq!(*s.current_transform(), Matrix::one());
}

#[test]
fn move_unit_sphere() {
    let mut s = sphere_unit();
    let t = translation(2.0, 3.0, 4.0);
    s.transform(t);
    assert_eq!(*s.current_transform(), t);
}

#[test]
fn intersect_ray_and_scaled_sphere() {
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let mut s = sphere_unit();
    s.transform(scaling(2.0, 2.0, 2.0));
    let xs = intersect(s.as_ref(), &r);
    assert_eq!(xs.len(), 2);
    assert!(feq(xs[0].t(), 3.0));
    assert!(feq(xs[1].t(), 7.0));
}

#[test]
fn intersect_ray_and_translated_sphere() {
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let mut s = sphere_unit();
    s.transform(translation(5.0, 0.0, 0.0));
    let xs = intersect(s.as_ref(), &r);
    assert_eq!(xs.len(), 0);
}

#[test]
fn compute_unit_sphere_normal() {
    let s = sphere_unit();
    let sq33 = 3.0_f64.sqrt() / 3.0;
    let sqv = vector(sq33, sq33, sq33);
    assert_eq!(s.normal_at(point(1.0, 0.0, 0.0)), vector(1.0, 0.0, 0.0));
    assert_eq!(s.normal_at(point(0.0, 1.0, 0.0)), vector(0.0, 1.0, 0.0));
    assert_eq!(s.normal_at(point(0.0, 0.0, 1.0)), vector(0.0, 0.0, 1.0));
    assert_eq!(s.normal_at(point(sq33, sq33, sq33)), sqv);
    assert_eq!(sqv.normalized(), sqv);
}

#[test]
fn compute_translated_sphere_normal() {
    let s = sphere(translation(0.0, 1.0, 0.0));
    assert_eq!(
        s.normal_at(point(0.0, 1.70711, -FRAC_1_SQRT_2)),
        vector(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2)
    );
}

#[test]
fn compute_transformed_sphere_normal() {
    let s = sphere(scaling(1.0, 0.5, 1.0) * rotation_z(PI / 5.0));

    assert_eq!(
        s.normal_at(point(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2)),
        vector(0.0, 0.97014, -0.24254)
    );
}

#[test]
fn assign_material_to_sphere() {
    let mut s = sphere_unit();
    assert_eq!(s.material(), &Material::new());
    let mut m = Material::new();
    m.ambient = 1.0;
    s.set_material(&m);
    assert_eq!(s.material(), &m);
}

#[test]
fn glass_sphere() {
    let s = sphere_glass();
    let m = s.material();
    assert_eq!(m.transparency, 1.0);
    assert_eq!(m.refractive_index, 1.52);
}
