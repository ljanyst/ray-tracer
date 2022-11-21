use ray_tracer::{cube_unit, feq, intersect, point, vector, Ray, Tuple};

#[test]
fn intersect_ray_and_cube() {
    struct TestData {
        pub origin: Tuple,
        pub direction: Tuple,
        pub t1: f64,
        pub t2: f64,
        pub hit: bool,
    }

    impl TestData {
        pub fn new_h(origin: Tuple, direction: Tuple, t1: f64, t2: f64) -> TestData {
            TestData {
                origin,
                direction,
                t1,
                t2,
                hit: true,
            }
        }
        pub fn new_m(origin: Tuple, direction: Tuple) -> TestData {
            TestData {
                origin,
                direction,
                t1: 0.0,
                t2: 0.0,
                hit: false,
            }
        }
    }

    let td = vec![
        TestData::new_h(point(5.0, 0.5, 0.0), vector(-1.0, 0.0, 0.0), 4.0, 6.0),
        TestData::new_h(point(-5.0, 0.5, 0.0), vector(1.0, 0.0, 0.0), 4.0, 6.0),
        TestData::new_h(point(0.5, 5.0, 0.0), vector(0.0, -1.0, 0.0), 4.0, 6.0),
        TestData::new_h(point(0.5, -5.0, 0.0), vector(0.0, 1.0, 0.0), 4.0, 6.0),
        TestData::new_h(point(0.5, 0.0, 5.0), vector(0.0, 0.0, -1.0), 4.0, 6.0),
        TestData::new_h(point(0.5, 0.0, -5.0), vector(0.0, 0.0, 1.0), 4.0, 6.0),
        TestData::new_h(point(0.0, 0.5, 0.0), vector(0.0, 0.0, 1.0), -1.0, 1.0),
        TestData::new_m(point(-2.0, 0.0, 0.0), vector(0.2673, 0.5345, 0.8018)),
        TestData::new_m(point(0.0, -2.0, 0.0), vector(0.8018, 0.2673, 0.5345)),
        TestData::new_m(point(0.0, 0.0, -2.0), vector(0.5345, 0.8018, 0.2673)),
        TestData::new_m(point(2.0, 0.0, 2.0), vector(0.0, 0.0, -1.0)),
        TestData::new_m(point(0.0, 2.0, 2.0), vector(0.0, -1.0, 0.0)),
        TestData::new_m(point(2.0, 2.0, 0.0), vector(-1.0, 0.0, 0.0)),
    ];

    let c = cube_unit();
    for t in td.iter() {
        let r = Ray::new(t.origin, t.direction);
        let xs = intersect(c.as_ref(), &r);
        if t.hit {
            assert_eq!(xs.len(), 2);
            assert!(feq(xs[0].t(), t.t1));
            assert!(feq(xs[1].t(), t.t2));
        } else {
            assert_eq!(xs.len(), 0);
        }
    }
}
