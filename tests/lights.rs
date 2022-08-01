use ray_tracer::{color, point, point_light};

#[test]
fn create_light() {
    let p = point(0.0, 0.0, 0.0);
    let i = color(1.0, 1.0, 1.0);
    let l = point_light(p, i);
    assert_eq!(l.position, p);
    assert_eq!(l.intensity, i);
}
