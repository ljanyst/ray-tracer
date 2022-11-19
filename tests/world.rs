use ray_tracer::{
    color, dummy_pattern, feq, plane, point, point_light, sphere, sphere_unit, translation, vector,
    Intersection, Intersections, Material, Ray, World,
};

use std::f64::consts::{FRAC_1_SQRT_2, SQRT_2};

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
    let p = i.properties(&r, &Intersections::new());
    let c = w.shade_hit(&p, 5);
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
    let p = i.properties(&r, &Intersections::new());
    let c = w.shade_hit(&p, 5);
    assert_eq!(c, color(0.90498, 0.90498, 0.90498));
}

#[test]
fn shade_ray_world_miss() {
    let w = World::default();
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0));
    let c = w.color_at(&r, 5);
    assert_eq!(c, color(0.0, 0.0, 0.0));
}

#[test]
fn shade_ray_world_hit() {
    let w = World::default();
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let c = w.color_at(&r, 5);
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
    let c = w.color_at(&r, 5);
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
    let p = i.properties(&r, &Intersections::new());
    let c = w.shade_hit(&p, 5);
    assert_eq!(c, color(0.1, 0.1, 0.1));
}

#[test]
fn reflected_color_nonreflective_material() {
    let mut w = World::default();
    let mut m1 = w.shapes[1].material().clone();
    m1.ambient = 1.0;
    w.shapes[1].set_material(&m1);

    let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
    let i = Intersection::new(1.0, w.shapes[1].as_ref());
    let p = i.properties(&r, &Intersections::new());
    let c = w.reflected_color(&p, 5);
    assert_eq!(c, color(0.0, 0.0, 0.0));
}

#[test]
fn reflected_color_reflective_material() {
    let mut w = World::default();
    let mut p = plane(translation(0.0, -1.0, 0.0));
    let mut m = Material::new();
    m.reflective = 0.5;
    p.set_material(&m);
    w.shapes.push(p);

    let r = Ray::new(
        point(0.0, 0.0, -3.0),
        vector(0.0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
    );
    let i = Intersection::new(SQRT_2, w.shapes[2].as_ref());
    let p = i.properties(&r, &Intersections::new());
    let c = w.reflected_color(&p, 5);
    assert_eq!(c, color(0.19032, 0.2379, 0.14274));
}

#[test]
fn reflected_color_maximum_recursion() {
    let mut w = World::default();
    let mut p = plane(translation(0.0, -1.0, 0.0));
    let mut m = Material::new();
    m.reflective = 0.5;
    p.set_material(&m);
    w.shapes.push(p);

    let r = Ray::new(
        point(0.0, 0.0, -3.0),
        vector(0.0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
    );
    let i = Intersection::new(SQRT_2, w.shapes[2].as_ref());
    let p = i.properties(&r, &Intersections::new());
    let c = w.reflected_color(&p, 0);
    assert_eq!(c, color(0.0, 0.0, 0.0));
}

#[test]
fn color_at_two_paralel_mirrors() {
    let mut w = World::empty();
    let l = point_light(point(0.0, 0.0, 0.0), color(1.0, 1.0, 1.0));
    w.lights.push(l);

    let mut lower = plane(translation(0.0, -1.0, 0.0));
    let mut m1 = Material::new();
    m1.reflective = 1.0;
    lower.set_material(&m1);
    w.shapes.push(lower);

    let mut upper = plane(translation(0.0, 1.0, 0.0));
    let mut m2 = Material::new();
    m2.reflective = 1.0;
    upper.set_material(&m2);
    w.shapes.push(upper);

    let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 1.0, 0.0));
    let _c = w.color_at(&r, 5);
}

#[test]
fn refracted_color_opaque_material() {
    let w = World::default();
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let i = Intersection::new(4.0, w.shapes[0].as_ref());
    let p = i.properties(&r, &Intersections::new());
    let c = w.refracted_color(&p, 5);
    assert_eq!(c, color(0.0, 0.0, 0.0));
}

#[test]
fn refracted_color_max_recursion() {
    let w = World::default();
    let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
    let i = Intersection::new(4.0, w.shapes[0].as_ref());
    let p = i.properties(&r, &Intersections::new());
    let c = w.refracted_color(&p, 0);
    assert_eq!(c, color(0.0, 0.0, 0.0));
}

#[test]
fn refracted_color_total_internal_reflection() {
    let mut w = World::default();
    w.shapes[0].material_mut().transparency = 1.0;
    w.shapes[0].material_mut().refractive_index = 1.52;
    let r = Ray::new(point(0.0, 0.0, FRAC_1_SQRT_2), vector(0.0, 1.0, 0.0));
    let mut xs = Intersections::new();
    xs.push(Intersection::new(-FRAC_1_SQRT_2, w.shapes[0].as_ref()));
    xs.push(Intersection::new(FRAC_1_SQRT_2, w.shapes[0].as_ref()));
    xs.sort();

    let p = xs.at(1).properties(&r, &xs);
    let c = w.refracted_color(&p, 5);
    assert_eq!(c, color(0.0, 0.0, 0.0));
}

#[test]
fn refracted_color() {
    let mut w = World::default();
    w.shapes[0].material_mut().ambient = 1.0;
    w.shapes[0].material_mut().pattern = Some(dummy_pattern());
    w.shapes[1].material_mut().transparency = 1.0;
    w.shapes[1].material_mut().refractive_index = 1.5;
    let r = Ray::new(point(0.0, 0.0, 0.1), vector(0.0, 1.0, 0.0));
    let mut xs = Intersections::new();
    xs.push(Intersection::new(-0.9899, w.shapes[0].as_ref()));
    xs.push(Intersection::new(-0.4899, w.shapes[1].as_ref()));
    xs.push(Intersection::new(0.4899, w.shapes[1].as_ref()));
    xs.push(Intersection::new(0.9899, w.shapes[0].as_ref()));
    xs.sort();

    let p = xs.at(2).properties(&r, &xs);
    let c = w.refracted_color(&p, 5);
    assert_eq!(c, color(0.0, 0.99888, 0.04725));
}

#[test]
fn shade_hit_with_transparent_material() {
    let mut w = World::default();

    let mut p = plane(translation(0.0, -1.0, 0.0));
    p.material_mut().transparency = 0.5;
    p.material_mut().refractive_index = 1.5;
    w.shapes.push(p);

    let mut s = sphere(translation(0.0, -3.5, -0.5));
    s.material_mut().color = color(1.0, 0.0, 0.0);
    s.material_mut().ambient = 0.5;
    w.shapes.push(s);

    let r = Ray::new(
        point(0.0, 0.0, -3.0),
        vector(0.0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
    );
    let mut xs = Intersections::new();
    xs.push(Intersection::new(SQRT_2, w.shapes[2].as_ref()));
    xs.sort();

    let p = xs.at(0).properties(&r, &xs);
    let c = w.shade_hit(&p, 5);
    assert_eq!(c, color(0.93642, 0.68642, 0.68642));
}
