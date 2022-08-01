use ray_tracer::{color, point, point_light, vector, Material};

#[test]
fn create_default_material() {
    let m = Material::new();
    assert_eq!(m.color, color(1.0, 1.0, 1.0));
    assert_eq!(m.ambient, 0.1);
    assert_eq!(m.diffuse, 0.9);
    assert_eq!(m.specular, 0.9);
    assert_eq!(m.shininess, 200.0);
}

#[test]
fn shade_material_light_behind_eye_perpendicular_to_surface() {
    let m = Material::new();
    let pos = point(0.0, 0.0, 0.0);
    let eyev = vector(0.0, 0.0, -1.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = point_light(point(0.0, 0.0, -10.0), color(1.0, 1.0, 1.0));
    assert_eq!(
        m.lighting(&light, &pos, &eyev, &normalv),
        color(1.9, 1.9, 1.9)
    );
}

#[test]
fn shade_material_light_perpendicular_eye_45_to_surface() {
    let m = Material::new();
    let pos = point(0.0, 0.0, 0.0);
    let sq22 = 2.0_f64.sqrt() / 2.0;
    let eyev = vector(0.0, sq22, -sq22);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = point_light(point(0.0, 0.0, -10.0), color(1.0, 1.0, 1.0));
    assert_eq!(
        m.lighting(&light, &pos, &eyev, &normalv),
        color(1.0, 1.0, 1.0)
    );
}

#[test]
fn shade_material_light_45_eye_perpendicular_to_surface() {
    let m = Material::new();
    let pos = point(0.0, 0.0, 0.0);
    let eyev = vector(0.0, 0.0, -1.0);
    let normalv = vector(0.0, 0.0, -1.0);
    let light = point_light(point(0.0, 0.0, 10.0), color(1.0, 1.0, 1.0));
    assert_eq!(
        m.lighting(&light, &pos, &eyev, &normalv),
        color(0.1, 0.1, 0.1)
    );
}
