// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use crate::demo::demo5::demo5;
use crate::demo::demo6::demo6;
use crate::demo::demo7and8::demo7and8;
use crate::demo::demo9::demo9;
use crate::demo::noise::noise;

mod demo {
    pub mod demo5;
    pub mod demo6;
    pub mod demo7and8;
    pub mod demo9;
    pub mod noise;
}

use std::collections::HashMap;
use std::env;

pub fn main() {
    let mut demos = HashMap::<&str, fn()>::new();
    demos.insert("demo5", demo5);
    demos.insert("demo6", demo6);
    demos.insert("demo7and8", demo7and8);
    demos.insert("demo9", demo9);
    demos.insert("noise", noise);

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    if args.len() == 0 {
        println!("Available demos:");
        let mut kx: Vec<&&str> = demos.keys().collect();
        kx.sort();
        for k in kx.iter() {
            println!("{}", k);
        }
        return;
    }

    for arg in args.iter() {
        match demos.get(arg.as_str()) {
            None => println!("No such demo: {}", arg),
            Some(func) => func(),
        }
    }
}
