use ray_tracer::{
    cylinder_min_max, cylinder_unit, feq, intersect, point, vector, Ray, Shape, Tuple,
};

#[test]
fn intersect_ray_and_cylinder() {
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

    let run_tests = |td: Vec<TestData>, c: &dyn Shape| {
        for t in td.iter() {
            let r = Ray::new(t.origin, t.direction.normalized());
            let xs = intersect(c, &r);
            if t.hit {
                assert_eq!(xs.len(), 2);
                assert!(feq(xs[0].t(), t.t1));
                assert!(feq(xs[1].t(), t.t2));
            } else {
                assert_eq!(xs.len(), 0);
            }
        }
    };

    let td = vec![
        TestData::new_m(point(1.0, 0.0, 0.0), vector(0.0, 1.0, 0.0)),
        TestData::new_m(point(0.0, 0.0, 0.0), vector(0.0, 1.0, 0.0)),
        TestData::new_m(point(0.0, 0.0, -5.0), vector(1.0, 1.0, 1.0)),
        TestData::new_h(point(1.0, 0.0, -5.0), vector(0.0, 0.0, 1.0), 5.0, 5.0),
        TestData::new_h(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0), 4.0, 6.0),
        TestData::new_h(
            point(0.5, 0.0, -5.0),
            vector(0.1, 1.0, 1.0),
            6.80798,
            7.08872,
        ),
    ];

    let c = cylinder_unit();
    run_tests(td, c.as_ref());

    let td = vec![
        TestData::new_m(point(0.0, 1.5, 0.0), vector(0.1, 1.0, 0.0)),
        TestData::new_m(point(0.0, -3.0, -5.0), vector(0.0, 0.0, 1.0)),
        TestData::new_m(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0)),
        TestData::new_m(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0)),
        TestData::new_m(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0)),
        TestData::new_h(point(0.0, 1.5, -2.0), vector(0.0, 0.0, 1.0), 1.0, 3.0),
    ];

    let c = cylinder_min_max(1.0, 2.0, false);
    run_tests(td, c.as_ref());

    let td = vec![
        TestData::new_h(point(0.0, 3.0, 0.0), vector(0.0, -1.0, 0.0), 1.0, 2.0),
        TestData::new_h(
            point(0.0, 3.0, -2.0),
            vector(0.0, -1.0, 2.0),
            2.23607,
            3.35410,
        ),
        TestData::new_h(
            point(0.0, 3.0, -2.0),
            vector(0.0, -1.0, 1.0),
            std::f64::consts::SQRT_2,
            2.82843,
        ),
        TestData::new_h(
            point(0.0, 0.0, -2.0),
            vector(0.0, 1.0, 2.0),
            2.23607,
            3.35410,
        ),
        TestData::new_h(
            point(0.0, -1.0, -2.0),
            vector(0.0, 1.0, 1.0),
            2.82843,
            4.24264,
        ),
    ];

    let c = cylinder_min_max(1.0, 2.0, true);
    run_tests(td, c.as_ref());
}

#[test]
fn compute_unit_cylinder_normal() {
    struct TestData {
        pub point: Tuple,
        pub normal: Tuple,
    }

    impl TestData {
        pub fn new(point: Tuple, normal: Tuple) -> TestData {
            TestData { point, normal }
        }
    }

    let td = vec![
        TestData::new(point(1.0, 0.0, 0.0), vector(1.0, 0.0, 0.0)),
        TestData::new(point(0.0, 5.0, -1.0), vector(0.0, 0.0, -1.0)),
        TestData::new(point(0.0, -2.0, 1.0), vector(0.0, 0.0, 1.0)),
        TestData::new(point(-1.0, 1.0, 0.0), vector(-1.0, 0.0, 0.0)),
    ];

    let c = cylinder_unit();
    for t in td.iter() {
        let normal = c.normal_at(t.point);
        assert_eq!(normal, t.normal);
    }
}
