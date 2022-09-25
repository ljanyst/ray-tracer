use ray_tracer::{color, feq, point, point_light, vector, Intersection, Ray, World};
use ray_tracer::{sphere, sphere_unit, translation};

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

#[test]
fn shade_ray_world_intersection() {
    let w = World::default();
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let i = Intersection::new(4.0, w.shapes[0].as_ref());
    let p = i.properties(&r);
    let c = w.shade_hit(p);
    assert_eq!(c, color(0.38066, 0.47583, 0.2855));
}

#[test]
fn shade_ray_world_intersection_inside() {
    let mut w = World::default();
    let l = point_light(point(0.0, 0.25, 0.0), color(1.0, 1.0, 1.0));
    w.lights.clear();
    w.lights.push(l);

    let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
    let i = Intersection::new(0.5, w.shapes[1].as_ref());
    let p = i.properties(&r);
    let c = w.shade_hit(p);
    assert_eq!(c, color(0.90498, 0.90498, 0.90498));
}

#[test]
fn shade_ray_world_miss() {
    let w = World::default();
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0));
    let c = w.color_at(&r);
    assert_eq!(c, color(0.0, 0.0, 0.0));
}

#[test]
fn shade_ray_world_hit() {
    let w = World::default();
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let c = w.color_at(&r);
    assert_eq!(c, color(0.38066, 0.47583, 0.2855));
}

#[test]
fn shade_ray_world_hit_intersection_behind_day() {
    let mut w = World::default();
    let mut m0 = w.shapes[0].material().clone();
    let mut m1 = w.shapes[1].material().clone();
    m0.ambient = 1.0;
    m1.ambient = 1.0;
    w.shapes[0].set_material(&m0);
    w.shapes[1].set_material(&m1);

    let r = Ray::new(point(0.0, 0.0, 0.75), vector(0.0, 0.0, -1.0));
    let c = w.color_at(&r);
    assert_eq!(c, w.shapes[1].material().color);
}

#[test]
fn check_point_in_shadow() {
    let w = World::default();
    let p1 = point(10.0, -10.0, 10.0);
    let p2 = point(-20.0, 20.0, -20.0);
    let p3 = point(-2.0, 2.0, -2.0);

    assert!(w.is_shadowed(&w.lights[0], p1));
    assert!(!w.is_shadowed(&w.lights[0], p2));
    assert!(!w.is_shadowed(&w.lights[0], p3));
}

#[test]
fn shade_hit_in_shadow() {
    let mut w = World::empty();
    let l = point_light(point(0.0, 0.0, -10.0), color(1.0, 1.0, 1.0));
    w.lights.push(l);
    w.shapes.push(sphere_unit());
    w.shapes.push(sphere(translation(0.0, 0.0, 10.0)));

    let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
    let i = Intersection::new(4.0, w.shapes[1].as_ref());
    let p = i.properties(&r);
    let c = w.shade_hit(p);
    assert_eq!(c, color(0.1, 0.1, 0.1));
}
