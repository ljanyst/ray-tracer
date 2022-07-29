use ray_tracer::{feq, Matrix, Tuple};

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

#[test]
fn multiply_matrix_by_identity_matrix() {
    let m = Matrix::new(
        0.0, 1.0, 2.0, 4.0, //
        1.0, 2.0, 4.0, 8.0, //
        2.0, 4.0, 8.0, 16.0, //
        4.0, 8.0, 16.0, 32.0, //
    );
    assert_eq!(m * Matrix::one(), m);
}

#[test]
fn multiply_identitn_matrix_by_tuple() {
    let t = Tuple::new(1.0, 2.0, 3.0, 4.0);
    assert_eq!(Matrix::one() * t, t);
}

#[test]
fn transpose_matrix() {
    let m = Matrix::new(
        0.0, 9.0, 3.0, 0.0, //
        9.0, 8.0, 0.0, 8.0, //
        1.0, 8.0, 5.0, 3.0, //
        0.0, 0.0, 5.0, 8.0, //
    );
    let mt = Matrix::new(
        0.0, 9.0, 1.0, 0.0, //
        9.0, 8.0, 8.0, 0.0, //
        3.0, 0.0, 5.0, 5.0, //
        0.0, 8.0, 3.0, 8.0, //
    );
    assert_eq!(m.transposed(), mt);
}

#[test]
fn compute_determinant_2x2() {
    let m = Matrix::new2(
        1.0, 5.0, //
        -3.0, 2.0, //
    );
    assert_eq!(m.det(), 17.0);
}

#[test]
fn compute_determinant_3x3() {
    let m = Matrix::new3(
        1.0, 2.0, 6.0, //
        -5.0, 8.0, -4.0, //
        2.0, 6.0, 4.0, //
    );
    assert_eq!(m.cofactor(0, 0), 56.0);
    assert_eq!(m.cofactor(0, 1), 12.0);
    assert_eq!(m.cofactor(0, 2), -46.0);
    assert_eq!(m.det(), -196.0);
}

#[test]
fn compute_determinant_4x4() {
    let m = Matrix::new(
        -2.0, -8.0, 3.0, 5.0, //
        -3.0, 1.0, 7.0, 3.0, //
        1.0, 2.0, -9.0, 6.0, //
        -6.0, 7.0, 7.0, -9.0, //
    );
    assert_eq!(m.cofactor(0, 0), 690.0);
    assert_eq!(m.cofactor(0, 1), 447.0);
    assert_eq!(m.cofactor(0, 2), 210.0);
    assert_eq!(m.cofactor(0, 3), 51.0);
    assert_eq!(m.det(), -4071.0);
}

#[test]
fn get_submatrix_3x3() {
    let m1 = Matrix::new3(
        1.0, 5.0, 0.0, //
        -3.0, 2.0, 7.0, //
        0.0, 6.0, -3.0, //
    );
    let m2 = Matrix::new2(
        -3.0, 2.0, //
        0.0, 6.0, //
    );
    assert_eq!(m1.submatrix(0, 2), m2);
}

#[test]
fn get_submatrix_4x4() {
    let m1 = Matrix::new(
        -6.0, 1.0, 1.0, 6.0, //
        -8.0, 5.0, 8.0, 6.0, //
        -1.0, 0.0, 8.0, 2.0, //
        -7.0, 1.0, -1.0, 1.0, //
    );
    let m2 = Matrix::new3(
        -6.0, 1.0, 6.0, //
        -8.0, 8.0, 6.0, //
        -7.0, -1.0, 1.0, //
    );
    assert_eq!(m1.submatrix(2, 1), m2);
}

#[test]
fn compute_minor_3x3() {
    let m = Matrix::new3(
        3.0, 5.0, 0.0, //
        2.0, -1.0, -7.0, //
        6.0, -1.0, 5.0, //
    );
    let b = m.submatrix(1, 0);
    assert_eq!(b.det(), 25.0);
    assert_eq!(m.minor(1, 0), 25.0);
}

#[test]
fn compute_cofactor_3x3() {
    let m = Matrix::new3(
        3.0, 5.0, 0.0, //
        2.0, -1.0, -7.0, //
        6.0, -1.0, 5.0, //
    );
    assert_eq!(m.minor(0, 0), -12.0);
    assert_eq!(m.cofactor(0, 0), -12.0);
    assert_eq!(m.minor(1, 0), 25.0);
    assert_eq!(m.cofactor(1, 0), -25.0);
}

#[test]
fn test_matrix_invertability() {
    let m1 = Matrix::new(
        6.0, 4.0, 4.0, 4.0, //
        5.0, 5.0, 7.0, 6.0, //
        4.0, -9.0, 3.0, -7.0, //
        9.0, 1.0, 7.0, -6.0, //
    );
    let m2 = Matrix::new(
        -4.0, 2.0, -2.0, -3.0, //
        9.0, 6.0, 2.0, 6.0, //
        0.0, -5.0, 1.0, -5.0, //
        0.0, 0.0, 0.0, 0.0, //
    );
    assert!(m1.is_invertible());
    assert!(!m2.is_invertible());
}

#[test]
fn compute_matrix_inverse() {
    let m1 = Matrix::new(
        -5.0, 2.0, 6.0, -8.0, //
        1.0, -5.0, 1.0, 8.0, //
        7.0, 7.0, -6.0, -7.0, //
        1.0, -3.0, 7.0, 4.0, //
    );
    let m1i = Matrix::new(
        0.21805, 0.45113, 0.24060, -0.04511, //
        -0.80827, -1.45677, -0.44361, 0.52068, //
        -0.07895, -0.22368, -0.05263, 0.19737, //
        -0.52256, -0.81391, -0.30075, 0.30639, //
    );
    assert_eq!(m1.det(), 532.0);
    assert_eq!(m1.cofactor(2, 3), -160.0);
    assert!(feq(m1i.at(3, 2), -160.0 / 532.0));
    assert_eq!(m1.cofactor(3, 2), 105.0);
    assert!(feq(m1i.at(2, 3), 105.0 / 532.0));
    assert_eq!(m1.inverted(), m1i);

    let m2 = Matrix::new(
        8.0, -5.0, 9.0, 2.0, //
        7.0, 5.0, 6.0, 1.0, //
        -6.0, 0.0, 9.0, 6.0, //
        -3.0, 0.0, -9.0, -4.0, //
    );
    let m2i = Matrix::new(
        -0.15385, -0.15385, -0.28205, -0.53846, //
        -0.07692, 0.12308, 0.02564, 0.03077, //
        0.35897, 0.35897, 0.43590, 0.92308, //
        -0.69231, -0.69231, -0.76923, -1.92308, //
    );
    assert_eq!(m2.inverted(), m2i);

    let m3 = Matrix::new(
        9.0, 3.0, 0.0, 9.0, //
        -5.0, -2.0, -6.0, -3.0, //
        -4.0, 9.0, 6.0, 4.0, //
        -7.0, 6.0, 6.0, 2.0, //
    );
    let m3i = Matrix::new(
        -0.04074, -0.07778, 0.14444, -0.22222, //
        -0.07778, 0.03333, 0.36667, -0.33333, //
        -0.02901, -0.14630, -0.10926, 0.12963, //
        0.17778, 0.06667, -0.26667, 0.33333, //
    );
    assert_eq!(m3.inverted(), m3i);
}

#[test]
fn multiply_matrix_product_by_inverse() {
    let m1 = Matrix::new(
        3.0, -9.0, 7.0, 3.0, //
        3.0, -8.0, 2.0, -9.0, //
        -4.0, 4.0, 4.0, 1.0, //
        -6.0, 5.0, -1.0, 1.0, //
    );
    let m2 = Matrix::new(
        8.0, 2.0, 2.0, 2.0, //
        3.0, -1.0, 7.0, 0.0, //
        7.0, 0.0, 5.0, 4.0, //
        6.0, -2.0, 0.0, 5.0, //
    );
    let m3 = m1 * m2;
    assert_eq!(m3 * m2.inverted(), m1);
}
