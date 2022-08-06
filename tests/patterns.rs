use ray_tracer::Tuple;
use ray_tracer::{checker_pattern_unit, ring_pattern_unit};
use ray_tracer::{color, gradient_pattern_unit, point, stripe_pattern_unit};

const BLACK: Tuple = color(0.0, 0.0, 0.0);
const WHITE: Tuple = color(1.0, 1.0, 1.0);

#[test]
fn verify_stripe_pattern() {
    let p = stripe_pattern_unit(WHITE, BLACK);

    // constant in y
    assert_eq!(p.local_color_at(point(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(p.local_color_at(point(0.0, 1.0, 0.0)), WHITE);
    assert_eq!(p.local_color_at(point(0.0, 2.0, 0.0)), WHITE);

    // constant in z
    assert_eq!(p.local_color_at(point(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(p.local_color_at(point(0.0, 0.0, 1.0)), WHITE);
    assert_eq!(p.local_color_at(point(0.0, 0.0, 2.0)), WHITE);

    // alternates in x
    assert_eq!(p.local_color_at(point(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(p.local_color_at(point(0.9, 0.0, 0.0)), WHITE);
    assert_eq!(p.local_color_at(point(1.0, 0.0, 0.0)), BLACK);
    assert_eq!(p.local_color_at(point(-0.1, 0.0, 0.0)), BLACK);
    assert_eq!(p.local_color_at(point(-1.0, 0.0, 0.0)), BLACK);
    assert_eq!(p.local_color_at(point(-1.1, 0.0, 0.0)), WHITE);
}

#[test]
fn verify_gradient_pattern() {
    let p = gradient_pattern_unit(WHITE, BLACK);

    assert_eq!(p.local_color_at(point(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(
        p.local_color_at(point(0.25, 0.0, 0.0)),
        color(0.75, 0.75, 0.75)
    );
    assert_eq!(p.local_color_at(point(0.5, 0.0, 0.0)), color(0.5, 0.5, 0.5));
    assert_eq!(
        p.local_color_at(point(0.75, 0.0, 0.0)),
        color(0.25, 0.25, 0.25)
    );
}

#[test]
fn compare_patterns() {
    let p1 = stripe_pattern_unit(WHITE, BLACK);
    let p2 = stripe_pattern_unit(WHITE, color(0.0, 1.0, 0.0));
    let p3 = gradient_pattern_unit(WHITE, BLACK);

    assert_eq!(&p1, &p1);
    assert_ne!(&p1, &p2);
    assert_ne!(&p1, &p3);
}

#[test]
fn verify_ring_pattern() {
    let p = ring_pattern_unit(WHITE, BLACK);
    assert_eq!(p.local_color_at(point(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(p.local_color_at(point(1.0, 0.0, 0.0)), BLACK);
    assert_eq!(p.local_color_at(point(0.0, 0.0, 1.0)), BLACK);
    assert_eq!(p.local_color_at(point(0.708, 0.0, 0.708)), BLACK);
}

#[test]
fn verify_checker_pattern() {
    let p = checker_pattern_unit(WHITE, BLACK);
    assert_eq!(p.local_color_at(point(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(p.local_color_at(point(0.99, 0.0, 0.0)), WHITE);
    assert_eq!(p.local_color_at(point(1.01, 0.0, 0.0)), BLACK);

    assert_eq!(p.local_color_at(point(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(p.local_color_at(point(0.0, 0.99, 0.0)), WHITE);
    assert_eq!(p.local_color_at(point(0.0, 1.01, 0.0)), BLACK);

    assert_eq!(p.local_color_at(point(0.0, 0.0, 0.0)), WHITE);
    assert_eq!(p.local_color_at(point(0.0, 0.0, 0.99)), WHITE);
    assert_eq!(p.local_color_at(point(0.0, 0.0, 1.01)), BLACK);
}
