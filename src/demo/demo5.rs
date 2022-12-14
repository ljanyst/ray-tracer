// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use ray_tracer::{color, intersect, point, sphere_unit, Canvas, Ray};

use std::io::{self, Write};
use std::time::Instant;

pub fn demo5() {
    print!("Rendering demo5... ");
    io::stdout().flush().unwrap();
    let now = Instant::now();
    let ray_origin = point(0.0, 0.0, -5.0);
    let canvas_dim = (7.0_f64, 7.0_f64);
    let mut canvas = Canvas::new(500, 500);
    let sphere = sphere_unit();
    let offset = (canvas_dim.0 / 2.0, canvas_dim.1 / 2.0);
    let cz = 10.0_f64;

    for i in 0..canvas.width() {
        let cx = (i as f64 / canvas.width() as f64) * canvas_dim.0 - offset.0;
        for j in 0..canvas.height() {
            let cy = -((j as f64 / canvas.width() as f64) * canvas_dim.1 - offset.1);
            let ray_target = point(cx, cy, cz);
            let ray_direction = (ray_target - ray_origin).normalized();
            let ray = Ray::new(ray_origin, ray_direction);
            let xs = intersect(sphere.as_ref(), &ray);
            if xs.is_empty() {
                canvas.set(i, j, &color(0.0, 0.0, 0.0));
            } else {
                canvas.set(i, j, &color(1.0, 0.0, 0.0));
            }
        }
    }

    let elapsed = now.elapsed();
    let fname = "demo5.ppm";
    canvas.save(fname).unwrap();
    println!("done. Elapsed {:.2?}. Saved {}.", elapsed, fname);
}
