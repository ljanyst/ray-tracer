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

#[test]
fn construct_ppm_header() {
    let c = Canvas::new(5, 3);
    let ppm = c.ppm();
    let lines: Vec<&str> = ppm.split("\n").collect();
    assert_eq!(lines[0], "P3");
    assert_eq!(lines[1], "5 3");
    assert_eq!(lines[2], "255");
}

#[test]
fn construct_ppm_pixel_data() {
    let mut c = Canvas::new(5, 3);
    let p1 = color(1.5, 0.0, 0.0);
    let p2 = color(0.0, 0.5, 0.0);
    let p3 = color(-0.5, 0.0, 1.0);

    c.set(0, 0, &p1);
    c.set(2, 1, &p2);
    c.set(4, 2, &p3);

    let ppm = c.ppm();
    let lines: Vec<&str> = ppm.split("\n").collect();
    assert_eq!(lines[3], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
    assert_eq!(lines[4], "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
    assert_eq!(lines[5], "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
}

#[test]
fn split_ppm_lines() {
    let mut c = Canvas::new(10, 2);
    let color = color(1.0, 0.8, 0.6);
    for i in 0..10 {
        for j in 0..2 {
            c.set(i, j, &color);
        }
    }
    let ppm = c.ppm();
    let lines: Vec<&str> = ppm.split("\n").collect();
    assert_eq!(
        lines[3],
        "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
    );
    assert_eq!(
        lines[4],
        "153 255 204 153 255 204 153 255 204 153 255 204 153"
    );
    assert_eq!(
        lines[5],
        "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
    );
    assert_eq!(
        lines[6],
        "153 255 204 153 255 204 153 255 204 153 255 204 153"
    );
}

#[test]
fn ppm_ends_with_newline() {
    let mut c = Canvas::new(5, 3);
    let ppm = c.ppm();
    assert_eq!(ppm.chars().last().unwrap(), '\n');
}
