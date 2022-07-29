use ray_tracer::{Matrix, Tuple};

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

#[test]
fn multiply_two_matrices() {
    let m1 = Matrix::new(
        1.0, 2.0, 3.0, 4.0, //
        5.0, 6.0, 7.0, 8.0, //
        9.0, 8.0, 7.0, 6.0, //
        5.0, 4.0, 3.0, 2.0, //
    );
    let m2 = Matrix::new(
        -2.0, 1.0, 2.0, 3.0, //
        3.0, 2.0, 1.0, -1.0, //
        4.0, 3.0, 6.0, 5.0, //
        1.0, 2.0, 7.0, 8.0, //
    );
    let m3 = Matrix::new(
        20.0, 22.0, 50.0, 48.0, //
        44.0, 54.0, 114.0, 108.0, //
        40.0, 58.0, 110.0, 102.0, //
        16.0, 26.0, 46.0, 42.0, //
    );
    assert_eq!(m1 * m2, m3);
}

#[test]
fn multiply_matrix_by_tuple() {
    let m = Matrix::new(
        1.0, 2.0, 3.0, 4.0, //
        2.0, 4.0, 4.0, 2.0, //
        8.0, 6.0, 4.0, 1.0, //
        0.0, 0.0, 0.0, 1.0, //
    );
    let t1 = Tuple::new(1.0, 2.0, 3.0, 1.0);
    let t2 = Tuple::new(18.0, 24.0, 33.0, 1.0);
    assert_eq!(m * t1, t2);
}
