// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::constants::EPSILON;

pub fn feq(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

pub fn peq<T: ?Sized>(left: &Box<T>, right: &Box<T>) -> bool {
    let left: *const T = left.as_ref();
    let right: *const T = right.as_ref();
    left == right
}

#[macro_export]
macro_rules! pattern_boilerplate_2p {
    ($cls:ident, $unit:ident, $color:ident, $full:ident) => {
        pub fn $unit(color1: Tuple, color2: Tuple) -> Box<dyn Pattern> {
            Box::new(PatternImpl::new($cls {
                pattern1: solid_pattern(color1),
                pattern2: solid_pattern(color2),
            }))
        }

        pub fn $color(color1: Tuple, color2: Tuple, transform: Matrix) -> Box<dyn Pattern> {
            let mut s = $unit(color1, color2);
            s.transform(transform);
            s
        }

        pub fn $full(
            pattern1: Box<dyn Pattern>,
            pattern2: Box<dyn Pattern>,
            transform: Matrix,
        ) -> Box<dyn Pattern> {
            let mut p = Box::new(PatternImpl::new($cls {
                pattern1: pattern1,
                pattern2: pattern2,
            }));
            p.transform(transform);
            p
        }

        impl PartialEq for $cls {
            fn eq(&self, other: &Self) -> bool {
                self.pattern1.eq(&other.pattern1) && self.pattern2.eq(&other.pattern2)
            }
        }

        impl Eq for $cls {}
    };
}
