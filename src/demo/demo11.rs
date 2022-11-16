// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use ray_tracer::{
    checker_pattern_color, color, plane_unit, point, point_light, rotation_y, rotation_z, scaling,
    sphere_unit, stripe_pattern_color, translation, vector, view_transform, Camera, Material,
    World,
};

use std::f64::consts::PI;
use std::io::{self, Write};
use std::time::Instant;

fn setup_floor(world: &mut World) {
    let mut m = Material::new();
    m.pattern = Some(checker_pattern_color(
        color(0.8, 0.8, 0.8),
        color(0.4, 0.4, 0.4),
        translation(0.0, -0.5, 0.0) * rotation_y(-PI / 9.0),
    ));

    m.specular = 0.0;
    m.reflective = 0.5;

    let mut obj = plane_unit();
    obj.set_material(&m);
    world.shapes.push(obj);
}

fn setup_right_wall(world: &mut World) {
    let mut m = Material::new();
    m.pattern = Some(stripe_pattern_color(
        color(0.3, 0.3, 0.3),
        color(0.2, 0.2, 0.2),
        scaling(0.2, 0.2, 0.2),
    ));

    m.specular = 0.0;
    m.reflective = 0.25;

    let mut obj = plane_unit();
    obj.set_material(&m);
    obj.transform(translation(8.0, 0.0, 0.0) * rotation_y(-PI / 6.0) * rotation_z(PI / 2.0));
    world.shapes.push(obj);
}

fn setup_left_wall(world: &mut World) {
    let mut m = Material::new();
    m.pattern = Some(stripe_pattern_color(
        color(0.3, 0.3, 0.3),
        color(0.2, 0.2, 0.2),
        scaling(0.2, 0.2, 0.2),
    ));

    m.specular = 0.0;
    m.reflective = 0.25;

    let mut obj = plane_unit();
    obj.set_material(&m);
    obj.transform(translation(0.0, 0.0, 8.0) * rotation_y(2.0 * PI / 6.0) * rotation_z(PI / 2.0));
    world.shapes.push(obj);
}

fn setup_sphere1(world: &mut World) {
    let mut m = Material::new();
    m.color = color(0.7, 0.2, 0.1);
    m.specular = 0.3;
    m.shininess = 5.0;

    let mut obj = sphere_unit();
    obj.set_material(&m);
    obj.transform(translation(-1.0, 1.0, 1.0));
    world.shapes.push(obj);
}

fn reflection_scene() {
    print!("Rendering demo11 reflection scene... ");
    io::stdout().flush().unwrap();
    let now = Instant::now();

    let mut world = World::empty();
    setup_floor(&mut world);
    setup_right_wall(&mut world);
    setup_left_wall(&mut world);

    setup_sphere1(&mut world);

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
    let fname = "demo11-reflection.ppm";
    canvas.save(fname).unwrap();
    println!("done. Elapsed {:.2?}. Saved {}.", elapsed, fname);
}

pub fn demo11() {
    reflection_scene();
}
