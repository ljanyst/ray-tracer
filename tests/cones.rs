use ray_tracer::{cone_min_max, cone_unit, feq, intersect, point, vector, Ray, Shape, Tuple};

#[test]
fn intersect_ray_and_come() {
    struct TestData {
        pub origin: Tuple,
        pub direction: Tuple,
        pub t: Vec<f64>,
        pub check_hit: bool,
        pub hit_count: usize,
    }

    impl TestData {
        pub fn new_h(origin: Tuple, direction: Tuple, t: Vec<f64>) -> TestData {
            let len = t.len();
            TestData {
                origin,
                direction,
                t,
                check_hit: true,
                hit_count: len,
            }
        }

        pub fn new_c(origin: Tuple, direction: Tuple, hit_count: usize) -> TestData {
            TestData {
                origin,
                direction,
                t: vec![],
                check_hit: false,
                hit_count,
            }
        }
    }

    let run_tests = |td: Vec<TestData>, c: &dyn Shape| {
        for t in td.iter() {
            let r = Ray::new(t.origin, t.direction.normalized());
            let xs = intersect(c, &r);
            assert_eq!(xs.len(), t.hit_count);
            if t.check_hit {
                for (ta, te) in xs.iter().zip(&t.t) {
                    assert!(feq(ta.t(), *te));
                }
            }
        }
    };

    let td = vec![
        TestData::new_h(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0), vec![5.0, 5.0]),
        TestData::new_h(
            point(1.0, 1.0, -5.0),
            vector(-0.5, -1.0, 1.0),
            vec![4.55006, 49.44994],
        ),
        TestData::new_h(
            point(0.0, 0.0, -5.0),
            vector(1.0, 1.0, 1.0),
            vec![8.66025, 8.66025],
        ),
        TestData::new_h(
            point(0.0, 0.0, -1.0),
            vector(0.0, 1.0, 1.0),
            vec![std::f64::consts::FRAC_1_SQRT_2],
        ),
    ];

    let c = cone_unit();
    run_tests(td, c.as_ref());

    let td = vec![
        TestData::new_c(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0), 0),
        TestData::new_c(point(0.0, 0.0, -0.25), vector(0.0, 1.0, 1.0), 2),
        TestData::new_c(point(0.0, 0.0, -0.25), vector(0.0, 1.0, 0.0), 4),
    ];

    let c = cone_min_max(-0.5, 0.5, true);
    run_tests(td, c.as_ref());
}

#[test]
fn compute_unit_coner_normal() {
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
        TestData::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 0.0)),
        TestData::new(
            point(1.0, 1.0, 1.0),
            vector(1.0, -std::f64::consts::SQRT_2, 1.0),
        ),
        TestData::new(point(-1.0, -1.0, 0.0), vector(-1.0, 1.0, 0.0)),
    ];

    let c = cone_unit();
    for t in td.iter() {
        let normal = c.normal_at(t.point);
        assert_eq!(normal, t.normal.normalized());
    }
}
