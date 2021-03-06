use std::{f64::consts::PI, sync::Arc};

use library::{
    math::{Matrix, Tuple},
    space::*,
    Axis,
};
use sdl2_interface::Sdl2Interface;

const SCREEN_WIDTH: u16 = 400; // height is half

const LIGHT_POSITION: (i32, i32, i32) = (-8, 10, -10);

fn hexagon_corner() -> Arc<dyn Shape> {
    Arc::new(Sphere {
        transform: Matrix::scaling(0.25, 0.25, 0.25).translate(0, 0, -1),
        ..Sphere::default()
    })
}

fn hexagon_edge() -> Arc<dyn Shape> {
    Arc::new(Cylinder {
        minimum: 0.0,
        maximum: 1.0,
        transform: Matrix::scaling(0.25, 1.0, 0.25)
            .rotate(Axis::Z, -PI / 2.0)
            .rotate(Axis::Y, -PI / 6.0)
            .translate(0, 0, -1),
        ..Cylinder::default()
    })
}

fn hexagon_side(transform: Matrix) -> Arc<dyn Shape> {
    let children = vec![hexagon_corner(), hexagon_edge()];

    Group::new(transform, children)
}

fn hexagon() -> Arc<dyn Shape> {
    let sides = (0..6)
        .map(|n| {
            let transform = Matrix::rotation(Axis::Y, n as f64 * PI / 3.0);
            hexagon_side(transform)
        })
        .collect::<Vec<_>>();

    // Transformation added to make it look nicer.
    //
    Group::new(
        Matrix::rotation(Axis::X, -PI / 6.0)
            .rotate(Axis::Y, PI / 6.0)
            .translate(-0.35, 1.0, 0.0),
        sides,
    )
}

fn add_objects(objects: &mut Vec<Arc<dyn Shape>>) {
    let hexagon = hexagon();

    objects.push(hexagon);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// WORLD/CAMERA
////////////////////////////////////////////////////////////////////////////////////////////////////

fn prepare_world() -> World {
    let light_source = PointLight::new(LIGHT_POSITION, (1, 1, 1));

    let mut objects = vec![];

    add_objects(&mut objects);

    World {
        objects,
        light_source,
    }
}

fn prepare_camera() -> Camera {
    let mut camera = Camera::new(SCREEN_WIDTH, SCREEN_WIDTH / 2, PI / 3.0);

    camera.transform = Matrix::view_transform(
        &Tuple::point(0, 1.5, -5),
        &Tuple::point(0, 1, 0),
        &Tuple::vector(0, 1, 0),
    );

    camera
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// MAIN
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn practice() {
    let world = prepare_world();
    let camera = prepare_camera();

    let mut interface: Sdl2Interface = camera.render(&world);

    interface.wait_keypress();
}
