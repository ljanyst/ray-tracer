use ray_tracer::{color, point, point_light, sphere_unit, vector};
use ray_tracer::{gradient_pattern_unit, stripe_pattern_unit};
use ray_tracer::{Material, Tuple};

const BLACK: Tuple = color(0.0, 0.0, 0.0);
const WHITE: Tuple = color(1.0, 1.0, 1.0);

#[test]
fn create_default_material() {
    let m = Material::new();
    assert_eq!(m.color, color(1.0, 1.0, 1.0));
    assert_eq!(m.ambient, 0.1);
    assert_eq!(m.diffuse, 0.9);
    assert_eq!(m.specular, 0.9);
    assert_eq!(m.shininess, 200.0);
    assert_eq!(m.reflective, 0.0);
}

#[test]
fn shade_material_light_behind_eye_perpendicular_to_surface() {
    let m = Material::new();
    let s = sphere_unit();
    let pos = point(0.0, 0.0, 0.0);
    let eyev = vector(0.0, 0.0, -1.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = point_light(point(0.0, 0.0, -10.0), color(1.0, 1.0, 1.0));
    assert_eq!(
        m.lighting(s.as_ref(), &light, &pos, &eyev, &normalv, false),
        color(1.9, 1.9, 1.9)
    );
}

#[test]
fn shade_material_light_perpendicular_eye_45_to_surface() {
    let m = Material::new();
    let s = sphere_unit();
    let pos = point(0.0, 0.0, 0.0);
    let sq22 = 2.0_f64.sqrt() / 2.0;
    let eyev = vector(0.0, sq22, -sq22);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = point_light(point(0.0, 0.0, -10.0), color(1.0, 1.0, 1.0));
    assert_eq!(
        m.lighting(s.as_ref(), &light, &pos, &eyev, &normalv, false),
        color(1.0, 1.0, 1.0)
    );
}

#[test]
fn shade_material_light_45_eye_perpendicular_to_surface() {
    let m = Material::new();
    let s = sphere_unit();
    let pos = point(0.0, 0.0, 0.0);
    let eyev = vector(0.0, 0.0, -1.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = point_light(point(0.0, 0.0, 10.0), color(1.0, 1.0, 1.0));
    assert_eq!(
        m.lighting(s.as_ref(), &light, &pos, &eyev, &normalv, false),
        color(0.1, 0.1, 0.1)
    );
}

#[test]
fn shade_material_in_shadow() {
    let m = Material::new();
    let s = sphere_unit();
    let pos = point(0.0, 0.0, 0.0);
    let eyev = vector(0.0, 0.0, -1.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = point_light(point(0.0, 0.0, 10.0), color(1.0, 1.0, 1.0));
    assert_eq!(
        m.lighting(s.as_ref(), &light, &pos, &eyev, &normalv, true),
        color(0.1, 0.1, 0.1)
    );
}

#[test]
fn compare_material_patterns() {
    let p1 = stripe_pattern_unit(WHITE, BLACK);
    let p2 = stripe_pattern_unit(WHITE, color(0.0, 1.0, 0.0));
    let p3 = gradient_pattern_unit(WHITE, BLACK);

    let mut m1 = Material::new();
    let mut m2 = Material::new();
    assert_eq!(m1, m2);

    m1.pattern = Some(p1.clone());
    assert_ne!(m1, m2);

    m2.pattern = Some(p1);
    assert_eq!(m1, m2);

    m2.pattern = Some(p2);
    assert_ne!(m1, m2);

    m2.pattern = Some(p3);
    assert_ne!(m1, m2);
}
