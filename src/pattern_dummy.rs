// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::pattern::{LocalPattern, Pattern, PatternImpl};
use crate::tuple::Tuple;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct DummyPattern {}

impl LocalPattern for DummyPattern {
    fn local_color_at(&self, pt: Tuple) -> Tuple {
        pt
    }
}

pub fn dummy_pattern() -> Box<dyn Pattern> {
    Box::new(PatternImpl::new(DummyPattern {}))
}
