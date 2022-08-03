// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use ray_tracer::{color, intersect, point, Canvas, Ray, Shape, Sphere};
use ray_tracer::{point_light, Intersections, Material};

use std::io::{self, Write};
use std::time::Instant;

pub fn demo6() {
    print!("Rendering demo6... ");
    io::stdout().flush().unwrap();

    let now = Instant::now();
    let ray_origin = point(0.0, 0.0, -5.0);
    let canvas_dim = (7.0_f64, 7.0_f64);
    let mut canvas = Canvas::new(500, 500);
    let mut m = Material::new();
    m.color = color(1.0, 0.2, 1.0);
    let mut sphere = Box::new(Sphere::unit()) as Box<dyn Shape>;
    sphere.set_material(&m);
    let light = point_light(point(-10.0, 10.0, -10.0), color(1.0, 1.0, 1.0));
    let offset = (canvas_dim.0 / 2.0, canvas_dim.1 / 2.0);
    let cz = 10.0_f64;

    for i in 0..canvas.width() {
        let cx = (i as f64 / canvas.width() as f64) * canvas_dim.0 - offset.0;
        for j in 0..canvas.height() {
            let cy = -((j as f64 / canvas.width() as f64) * canvas_dim.1 - offset.1);
            let ray_target = point(cx, cy, cz);
            let ray_direction = (ray_target - ray_origin).normalized();
            let ray = Ray::new(ray_origin, ray_direction);
            let xs = Intersections::from_vector(intersect(&sphere, &ray));
            let hit = xs.hit();
            if hit == None {
                canvas.set(i, j, &color(0.0, 0.0, 0.0));
            } else {
                let hit = hit.unwrap();
                let point = ray.position(hit.t());
                let normalv = hit.shape().normal_at(point);
                let eyev = -ray.direction();
                let c = hit
                    .shape()
                    .material()
                    .lighting(&light, &point, &eyev, &normalv, false);
                canvas.set(i, j, &c);
            }
        }
    }

    let elapsed = now.elapsed();
    let fname = "demo6.ppm";
    canvas.save(fname).unwrap();
    println!("done. Elapsed {:.2?}. Saved {}.", elapsed, fname);
}
