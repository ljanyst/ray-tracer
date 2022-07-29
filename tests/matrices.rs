use ray_tracer::Matrix;

#[test]
fn create_matrix() {
    let m = Matrix::new(
        1.0, 2.0, 3.0, 4.0, //
        5.5, 6.5, 7.5, 8.5, //
        9.0, 10.0, 11.0, 12.0, //
        13.5, 14.5, 15.5, 16.5, //
    );
    assert_eq!(m.at(0, 0), 1.0);
    assert_eq!(m.at(0, 3), 4.0);
    assert_eq!(m.at(1, 0), 5.5);
    assert_eq!(m.at(1, 2), 7.5);
    assert_eq!(m.at(2, 2), 11.0);
    assert_eq!(m.at(3, 0), 13.5);
    assert_eq!(m.at(3, 2), 15.5);
}

#[test]
fn create_matrix_2x2() {
    let m = Matrix::new2(
        1.0, 2.0, //
        5.5, 6.5, //
    );
    assert_eq!(m.at(0, 0), 1.0);
    assert_eq!(m.at(0, 1), 2.0);
    assert_eq!(m.at(1, 0), 5.5);
    assert_eq!(m.at(1, 1), 6.5);
}

#[test]
fn create_matrix_3x3() {
    let m = Matrix::new3(
        -3.0, 5.0, 0.0, //
        1.0, -2.0, -7.0, //
        0.0, 1.0, 1.0, //
    );
    assert_eq!(m.at(0, 0), -3.0);
    assert_eq!(m.at(1, 1), -2.0);
    assert_eq!(m.at(2, 2), 1.0);
}

#[test]
fn compare_matrix() {
    let m1 = Matrix::new(
        1.0, 2.0, 3.0, 4.0, //
        5.0, 6.0, 7.0, 8.0, //
        9.0, 8.0, 7.0, 6.0, //
        5.0, 4.0, 3.0, 2.0, //
    );
    let m2 = Matrix::new(
        1.0, 2.0, 3.0, 4.0, //
        5.0, 6.0, 7.0, 8.0, //
        9.0, 8.0, 7.0, 6.0, //
        5.0, 4.0, 3.0, 2.0, //
    );
    let m3 = Matrix::new(
        2.0, 3.0, 4.0, 5.0, //
        6.0, 7.0, 8.0, 9.0, //
        8.0, 7.0, 6.0, 5.0, //
        4.0, 3.0, 2.0, 1.0, //
    );
    assert_eq!(m1, m2);
    assert_ne!(m1, m3);
}
