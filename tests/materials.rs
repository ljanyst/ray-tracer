use ray_tracer::{color, Material};

#[test]
fn create_default_material() {
    let m = Material::new();
    assert_eq!(m.color, color(1.0, 1.0, 1.0));
    assert_eq!(m.ambient, 0.1);
    assert_eq!(m.diffuse, 0.9);
    assert_eq!(m.specular, 0.9);
    assert_eq!(m.shininess, 200.0);
}
