// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use ray_tracer::{
    blended_pattern, color, gradient_pattern_color, noise_pattern_unit, plane_unit, point,
    point_light, rotation_y, rotation_z, scaling, sphere, stripe_pattern_color, translation,
    vector, view_transform, Camera, Material, World,
};

use std::f64::consts::PI;
use std::io::{self, Write};
use std::time::Instant;

pub fn demo10() {
    print!("Rendering demo10... ");
    io::stdout().flush().unwrap();
    let now = Instant::now();

    let mut world = World::empty();

    let mut floor_m = Material::new();
    floor_m.pattern = Some(noise_pattern_unit(blended_pattern(
        stripe_pattern_color(
            color(1.0, 1.0, 1.0),
            color(1.0, 0.0, 0.0),
            rotation_y(PI / 4.0) * scaling(0.75, 0.75, 0.75),
        ),
        stripe_pattern_color(
            color(1.0, 1.0, 1.0),
            color(1.0, 0.0, 0.0),
            rotation_y(-PI / 4.0) * scaling(0.75, 0.75, 0.75),
        ),
        translation(0.0, -0.5, -0.0),
    )));

    floor_m.specular = 0.0;

    let mut floor = plane_unit();
    floor.set_material(&floor_m);
    world.shapes.push(floor);

    let mut s1 = sphere(translation(-0.5, 1.0, 0.5));
    let mut s1m = Material::new();

    s1m.pattern = Some(noise_pattern_unit(stripe_pattern_color(
        color(0.0, 0.7, 0.0),
        color(0.0, 1.0, 0.0),
        rotation_y(2.0 * PI / 3.0) * rotation_z(PI / 4.0) * scaling(0.225, 0.225, 0.225),
    )));
    s1m.specular = 0.3;
    s1m.diffuse = 0.7;
    s1.set_material(&s1m);
    world.shapes.push(s1);

    let mut s2 = sphere(translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5));
    let mut s2m = Material::new();
    s2m.pattern = Some(gradient_pattern_color(
        color(1.0, 0.0, 0.0),
        color(1.0, 0.8, 0.1),
        translation(-1.5, 0.0, 0.0) * scaling(2.0, 2.0, 2.0),
    ));
    s2m.specular = 0.3;
    s2m.diffuse = 0.7;
    s2.set_material(&s2m);
    world.shapes.push(s2);

    let mut s3 = sphere(translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33));
    let mut s3m = Material::new();
    s3m.pattern = Some(noise_pattern_unit(stripe_pattern_color(
        color(0.0, 0.7, 0.0),
        color(0.0, 1.0, 0.0),
        rotation_y(-PI / 2.0) * rotation_z(0.0) * scaling(0.225, 0.225, 0.225),
    )));
    s3m.specular = 0.3;
    s3m.diffuse = 0.7;
    s3.set_material(&s3m);
    world.shapes.push(s3);

    let l = point_light(point(-10.0, 10.0, -10.0), color(1.0, 1.0, 1.0));
    world.lights.push(l);

    let mut camera = Camera::new(1280, 620, PI / 3.0);
    camera.set_transform(view_transform(
        point(0.0, 1.5, -5.0),
        point(0.0, 1.0, 0.0),
        vector(0.0, 1.0, 0.0),
    ));

    let canvas = camera.render(&world);

    let elapsed = now.elapsed();
    let fname = "demo10.ppm";
    canvas.save(fname).unwrap();
    println!("done. Elapsed {:.2?}. Saved {}.", elapsed, fname);
}
