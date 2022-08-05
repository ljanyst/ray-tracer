// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::demo::demo5::demo5;
use crate::demo::demo6::demo6;
use crate::demo::demo7and8::demo7and8;
use crate::demo::demo9::demo9;

mod demo {
    pub mod demo5;
    pub mod demo6;
    pub mod demo7and8;
    pub mod demo9;
}

pub fn main() {
    demo5();
    demo6();
    demo7and8();
    demo9();
}
