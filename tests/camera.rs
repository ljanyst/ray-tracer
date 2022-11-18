use ray_tracer::{
    color, feq, point, rotation_y, translation, vector, view_transform, Camera, World,
};

use std::f64::consts::{FRAC_1_SQRT_2, PI};

#[test]
fn compute_camera_pixel_size() {
    let c1 = Camera::new(200, 125, PI / 2.0);
    let c2 = Camera::new(125, 200, PI / 2.0);
    assert!(feq(c1.pixel_size(), 0.01));
    assert!(feq(c2.pixel_size(), 0.01));
}

#[test]
fn construct_ray_through_center() {
    let c = Camera::new(201, 101, PI / 2.0);
    let r = c.ray_for_pixel(100, 50);
    assert_eq!(r.origin(), point(0.0, 0.0, 0.0));
    assert_eq!(r.direction(), vector(0.0, 0.0, -1.0));
}

#[test]
fn construct_ray_through_corner() {
    let c = Camera::new(201, 101, PI / 2.0);
    let r = c.ray_for_pixel(0, 0);
    assert_eq!(r.origin(), point(0.0, 0.0, 0.0));
    assert_eq!(r.direction(), vector(0.66519, 0.33259, -0.66851));
}

#[test]
fn construct_ray_through_center_with_transformed_camera() {
    let mut c = Camera::new(201, 101, PI / 2.0);
    c.set_transform(rotation_y(PI / 4.0) * translation(0.0, -2.0, 5.0));
    let r = c.ray_for_pixel(100, 50);
    assert_eq!(r.origin(), point(0.0, 2.0, -5.0));
    assert_eq!(r.direction(), vector(FRAC_1_SQRT_2, 0.0, -FRAC_1_SQRT_2));
}

#[test]
fn render_the_default_world() {
    let w = World::default();
    let mut c = Camera::new(11, 11, PI / 2.0);
    c.set_transform(view_transform(
        point(0.0, 0.0, -5.0),
        point(0.0, 0.0, 0.0),
        vector(0.0, 1.0, 0.0),
    ));
    let img = c.render(&w);
    assert_eq!(img.at(5, 5), color(0.38066, 0.47583, 0.2855));
}
