// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::demo::demo10::demo10;
use crate::demo::demo11::*;
use crate::demo::demo5::demo5;
use crate::demo::demo6::demo6;
use crate::demo::demo7and8::demo7and8;
use crate::demo::demo9::demo9;
use crate::demo::noise::noise;

mod demo {
    pub mod demo10;
    pub mod demo11;
    pub mod demo5;
    pub mod demo6;
    pub mod demo7and8;
    pub mod demo9;
    pub mod noise;
}

use std::collections::HashMap;
use std::env;

pub fn main() {
    let demos: Vec<(&str, fn())> = vec![
        ("demo5", demo5),
        ("demo6", demo6),
        ("demo7and8", demo7and8),
        ("demo9", demo9),
        ("demo10", demo10),
        ("demo11-reflection-scene", demo11_reflection_scene),
        ("demo11-refraction-scene", demo11_refraction_scene),
        ("noise", noise),
    ];

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    if args.is_empty() {
        println!("Available demos:");
        for demo in demos.iter() {
            println!("{}", demo.0);
        }
        return;
    }

    if args.len() == 1 && args[0] == "all" {
        for demo in demos.iter() {
            demo.1();
        }
        return;
    }

    let mut demos_map = HashMap::<&str, fn()>::new();
    for demo in demos.iter() {
        demos_map.insert(demo.0, demo.1);
    }

    for arg in args.iter() {
        match demos_map.get(arg.as_str()) {
            None => println!("No such demo: {}", arg),
            Some(func) => func(),
        }
    }
}
