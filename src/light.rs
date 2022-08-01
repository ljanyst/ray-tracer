// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::tuple::Tuple;

pub struct Light {
    pub position: Tuple,
    pub intensity: Tuple,
}

pub fn point_light(position: Tuple, intensity: Tuple) -> Light {
    Light {
        position: position,
        intensity: intensity,
    }
}
