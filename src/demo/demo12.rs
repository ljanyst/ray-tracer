// Copyright 2022 Lukasz Janyst <lukasz@jany.st>
// Licensed under the MIT license, see the LICENSE file for details.

use ray_tracer::{
    checker_pattern_color, color, cube, noise_pattern_unit, point, point_light, rotation_y,
    scaling, stripe_pattern_color, translation, vector, view_transform, Camera, Material, Matrix,
    World,
};

use std::f64::consts::PI;
use std::io::{self, Write};
use std::time::Instant;

fn setup_floor(world: &mut World) {
    let mut m = Material::new();
    m.pattern = Some(checker_pattern_color(
        color(0.93, 0.93, 0.93),
        color(0.29, 0.25, 0.23),
        translation(0.0, -0.05, 0.0) * scaling(0.1, 0.1, 0.1),
    ));

    m.specular = 0.0;
    m.reflective = 0.05;

    let mut obj = cube(scaling(5.0, 0.1, 5.0));
    obj.set_material(&m);
    world.shapes.push(obj);
}

fn setup_walls(world: &mut World, transform: Matrix, pattern_transform: Matrix) {
    let mut m = Material::new();
    m.pattern = Some(noise_pattern_unit(stripe_pattern_color(
        color(0.6, 0.60, 0.35),
        color(0.65, 0.33, 0.25),
        pattern_transform * scaling(0.05, 0.05, 0.05),
    )));

    m.specular = 0.1;
    m.reflective = 0.05;

    let mut obj = cube(transform * scaling(5.001, 5.0, 5.0));
    obj.set_material(&m);
    world.shapes.push(obj);
}

fn setup_mirror(world: &mut World) {
    let mut obj1 = cube(translation(5.0, 2.0, 0.0) * scaling(0.1, 1.0, 3.0));
    obj1.material_mut().reflective = 0.6;
    obj1.material_mut().specular = 0.0;
    obj1.material_mut().color = color(0.0, 0.0, 0.0);
    world.shapes.push(obj1);

    let mut obj2 = cube(translation(5.0, 2.0, 0.0) * scaling(0.09, 1.1, 3.1));
    obj2.material_mut().color = color(0.29, 0.25, 0.23);
    world.shapes.push(obj2);
}

fn setup_table(world: &mut World) {
    let mut m = Material::new();
    m.color = color(0.7, 0.2, 0.1);
    m.specular = 0.3;
    m.shininess = 5.0;

    let mut obj1 = cube(translation(0.0, 1.0, 0.0) * scaling(1.0, 0.05, 2.0));
    obj1.set_material(&m);
    world.shapes.push(obj1);

    let mut leg = |transform| {
        let mut obj2 = cube(transform * scaling(0.05, 0.5, 0.05));
        obj2.set_material(&m);
        world.shapes.push(obj2);
    };
    leg(translation(0.9, 0.5, -1.9));
    leg(translation(-0.9, 0.5, -1.9));
    leg(translation(0.9, 0.5, 1.9));
    leg(translation(-0.9, 0.5, 1.9));
}

fn setup_paintings(world: &mut World) {
    let mut m = Material::new();
    m.specular = 0.3;
    m.shininess = 5.0;
    m.reflective = 0.2;

    let mut obj =
        cube(translation(-0.05, 2.0, 5.0) * scaling(0.5, 0.5, 0.05) * translation(-1.0, 0.0, 0.0));
    obj.set_material(&m);
    obj.material_mut().color = color(0.7, 0.1, 0.1);
    world.shapes.push(obj);

    let mut obj =
        cube(translation(0.05, 1.9, 5.0) * scaling(0.2, 0.2, 0.05) * translation(1.0, -1.0, 0.0));
    obj.set_material(&m);
    obj.material_mut().color = color(0.1, 0.7, 0.1);
    world.shapes.push(obj);

    let mut obj =
        cube(translation(0.05, 2.1, 5.0) * scaling(0.2, 0.2, 0.05) * translation(1.0, 1.0, 0.0));
    obj.set_material(&m);
    obj.material_mut().color = color(0.1, 0.1, 0.7);
    world.shapes.push(obj);
}

fn setup_cubes(world: &mut World) {
    let mut m = Material::new();
    m.specular = 0.3;
    m.shininess = 5.0;

    let mut obj =
        cube(translation(0.0, 1.05, 0.0) * scaling(0.2, 0.2, 0.2) * translation(0.0, 1.0, 0.0));
    obj.material_mut().color = color(0.1, 0.1, 0.7);
    obj.material_mut().transparency = 0.4;
    obj.material_mut().reflective = 0.02;
    obj.material_mut().ambient = 0.05;
    obj.material_mut().diffuse = 0.45;
    world.shapes.push(obj);

    let mut obj = cube(
        translation(0.5, 1.05, -1.0)
            * scaling(0.04, 0.12, 0.04)
            * translation(0.0, 1.0, 0.0)
            * rotation_y(0.43 * PI),
    );
    obj.set_material(&m);
    obj.material_mut().color = color(1.0, 0.44, 0.0);
    world.shapes.push(obj);

    let mut obj = cube(
        translation(-0.5, 1.05, -0.7)
            * scaling(0.12, 0.12, 0.12)
            * translation(0.0, 1.0, 0.0)
            * rotation_y(0.82 * PI),
    );
    obj.set_material(&m);
    obj.material_mut().color = color(0.06, 0.63, 0.62);
    obj.material_mut().reflective = 0.2;
    world.shapes.push(obj);

    let mut obj = cube(
        translation(-0.4, 1.05, 0.9)
            * scaling(0.04, 0.12, 0.04)
            * translation(0.0, 1.0, 0.0)
            * rotation_y(0.27 * PI),
    );
    obj.set_material(&m);
    obj.material_mut().color = color(0.33, 0.01, 0.46);
    world.shapes.push(obj);

    let mut obj = cube(
        translation(0.1, 1.05, 1.1)
            * scaling(0.12, 0.04, 0.12)
            * translation(0.0, 1.0, 0.0)
            * rotation_y(0.35 * PI),
    );
    obj.set_material(&m);
    obj.material_mut().color = color(1.0, 0.75, 0.0);
    obj.material_mut().reflective = 0.3;
    world.shapes.push(obj);

    let mut obj = cube(
        translation(0.5, 1.05, -0.4)
            * scaling(0.04, 0.04, 0.12)
            * translation(0.0, 1.0, 0.0)
            * rotation_y(0.64 * PI),
    );
    obj.set_material(&m);
    obj.material_mut().color = color(0.1, 0.75, 0.1);
    world.shapes.push(obj);
}

pub fn demo12() {
    print!("Rendering demo12... ");
    io::stdout().flush().unwrap();
    let now = Instant::now();

    let mut world = World::empty();
    setup_floor(&mut world);
    setup_walls(&mut world, Matrix::one(), translation(0.25, 0.0, 0.0));
    setup_walls(&mut world, rotation_y(PI / 2.0), Matrix::one());
    setup_mirror(&mut world);
    setup_table(&mut world);
    setup_paintings(&mut world);
    setup_cubes(&mut world);

    let l = point_light(point(-1.0, 4.5, -4.5), color(0.8, 0.8, 0.8));
    world.lights.push(l);

    let mut camera = Camera::new(1280, 620, PI / 3.0);
    camera.set_transform(view_transform(
        point(-4.8, 3.0, -4.8),
        point(0.0, 1.0, 0.0),
        vector(0.0, 1.0, 0.0),
    ));

    let canvas = camera.render(&world);

    let elapsed = now.elapsed();
    let fname = "demo12.ppm";
    canvas.save(fname).unwrap();
    println!("done. Elapsed {:.2?}. Saved {}.", elapsed, fname);
}
