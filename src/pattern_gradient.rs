// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::matrix::Matrix;
use crate::pattern::{LocalPattern, Pattern, PatternImpl};
use crate::tuple::Tuple;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct GradientPattern {
    color1: Tuple,
    color2: Tuple,
}

impl LocalPattern for GradientPattern {
    fn local_color_at(&self, pt: Tuple) -> Tuple {
        let distance = self.color2 - self.color1;
        let fraction = pt.x() - pt.x().floor();
        self.color1 + distance * fraction
    }
}

pub fn gradient_pattern_unit(color1: Tuple, color2: Tuple) -> Box<dyn Pattern> {
    Box::new(PatternImpl::new(GradientPattern {
        color1: color1,
        color2: color2,
    }))
}

pub fn gradient_pattern(color1: Tuple, color2: Tuple, transform: Matrix) -> Box<dyn Pattern> {
    let mut s = gradient_pattern_unit(color1, color2);
    s.transform(transform);
    s
}
