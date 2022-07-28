use ray_tracer::{color, Canvas};

#[test]
fn create_canvas() {
    let c = Canvas::new(10, 20);
    assert_eq!(c.width(), 10);
    assert_eq!(c.height(), 20);
    for x in 0..10 {
        for y in 0..20 {
            assert_eq!(c.at(x, y), color(0.0, 0.0, 0.0));
        }
    }
}

#[test]
fn set_canvas_pixels() {
    let mut c = Canvas::new(10, 20);
    let red = color(1.0, 0.0, 0.0);
    c.set(2, 3, &red);
    assert_eq!(c.at(2, 3), red);
}
