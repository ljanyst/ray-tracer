// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::matrix::Matrix;
use crate::pattern::{LocalPattern, Pattern, PatternImpl};
use crate::tuple::Tuple;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CheckerPattern {
    color1: Tuple,
    color2: Tuple,
}

impl LocalPattern for CheckerPattern {
    fn local_color_at(&self, pt: Tuple) -> Tuple {
        if (pt.x().floor() + pt.y().floor() + pt.z().floor()) as isize % 2 == 0 {
            return self.color1;
        }
        self.color2
    }
}

pub fn checker_pattern_unit(color1: Tuple, color2: Tuple) -> Box<dyn Pattern> {
    Box::new(PatternImpl::new(CheckerPattern {
        color1: color1,
        color2: color2,
    }))
}

pub fn checker_pattern(color1: Tuple, color2: Tuple, transform: Matrix) -> Box<dyn Pattern> {
    let mut s = checker_pattern_unit(color1, color2);
    s.transform(transform);
    s
}
