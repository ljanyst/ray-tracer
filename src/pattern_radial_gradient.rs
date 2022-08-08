// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::matrix::Matrix;
use crate::pattern::{LocalPattern, Pattern, PatternImpl};
use crate::tuple::Tuple;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct RadialGradientPattern {
    color1: Tuple,
    color2: Tuple,
}

impl LocalPattern for RadialGradientPattern {
    fn local_color_at(&self, pt: Tuple) -> Tuple {
        let distance = self.color2 - self.color1;
        let mut fraction = (pt.x().powi(2) + pt.z().powi(2)).sqrt();
        while fraction >= 2.0 {
            fraction -= 2.0;
        }
        if fraction > 1.0 {
            return self.color1 + distance * (2.0 - fraction);
        }
        self.color1 + distance * fraction
    }
}

pub fn radial_gradient_pattern_unit(color1: Tuple, color2: Tuple) -> Box<dyn Pattern> {
    Box::new(PatternImpl::new(RadialGradientPattern {
        color1: color1,
        color2: color2,
    }))
}

pub fn radial_gradient_pattern(
    color1: Tuple,
    color2: Tuple,
    transform: Matrix,
) -> Box<dyn Pattern> {
    let mut s = radial_gradient_pattern_unit(color1, color2);
    s.transform(transform);
    s
}
