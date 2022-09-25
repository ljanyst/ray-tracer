// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use ray_tracer::{point, Noise};

use std::fs::File;
use std::io::{self, Write};
use std::time::Instant;

pub fn noise() {
    print!("Rendering noise... ");
    io::stdout().flush().unwrap();
    let fname = "noise.txt";
    let now = Instant::now();

    let mut output = File::create(fname).unwrap();

    let n = Noise::new();

    let mut t = -1.0_f64;
    while t <= 1.0 {
        let r = n.octave_noise(point(t, t, t), 6, 0.5);
        writeln!(output, "{} {}", t, r).unwrap();
        t += 0.01;
    }

    let elapsed = now.elapsed();
    println!("done. Elapsed {:.2?}. Saved {}.", elapsed, fname);
}
