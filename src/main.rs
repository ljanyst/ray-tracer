// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::demo::demo4::demo4;
use crate::demo::demo5::demo5;
use crate::demo::demo6::demo6;

mod demo {
    pub mod demo4;
    pub mod demo5;
    pub mod demo6;
}

pub fn main() {
    demo4();
    demo5();
    demo6();
}
