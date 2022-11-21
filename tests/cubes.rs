use ray_tracer::{cube_unit, feq, intersect, point, vector, Ray, Tuple};

#[test]
fn intersect_ray_and_cube() {
    struct TestData {
        pub origin: Tuple,
        pub direction: Tuple,
        pub t1: f64,
        pub t2: f64,
    }

    impl TestData {
        pub fn new(origin: Tuple, direction: Tuple, t1: f64, t2: f64) -> TestData {
            TestData {
                origin,
                direction,
                t1,
                t2,
            }
        }
    }

    let td = vec![
        TestData::new(point(5.0, 0.5, 0.0), vector(-1.0, 0.0, 0.0), 4.0, 6.0),
        TestData::new(point(-5.0, 0.5, 0.0), vector(1.0, 0.0, 0.0), 4.0, 6.0),
        TestData::new(point(0.5, 5.0, 0.0), vector(0.0, -1.0, 0.0), 4.0, 6.0),
        TestData::new(point(0.5, -5.0, 0.0), vector(0.0, 1.0, 0.0), 4.0, 6.0),
        TestData::new(point(0.5, 0.0, 5.0), vector(0.0, 0.0, -1.0), 4.0, 6.0),
        TestData::new(point(0.5, 0.0, -5.0), vector(0.0, 0.0, 1.0), 4.0, 6.0),
        TestData::new(point(0.0, 0.5, 0.0), vector(0.0, 0.0, 1.0), -1.0, 1.0),
    ];

    let c = cube_unit();
    for t in td.iter() {
        let r = Ray::new(t.origin, t.direction);
        let xs = intersect(c.as_ref(), &r);
        assert_eq!(xs.len(), 2);
        assert!(feq(xs[0].t(), t.t1));
        assert!(feq(xs[1].t(), t.t2));
    }
}
