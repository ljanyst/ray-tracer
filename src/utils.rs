// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::constants::EPSILON;

pub fn feq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}
