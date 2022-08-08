// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::pattern::{LocalPattern, Pattern, PatternImpl};
use crate::tuple::Tuple;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct SolidPattern {
    color: Tuple,
}

impl LocalPattern for SolidPattern {
    fn local_color_at(&self, _pt: Tuple) -> Tuple {
        self.color
    }
}

pub fn solid_pattern(color: Tuple) -> Box<dyn Pattern> {
    Box::new(PatternImpl::new(SolidPattern { color: color }))
}
