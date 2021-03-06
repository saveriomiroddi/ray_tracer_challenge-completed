use std::{f64::consts::PI, sync::Arc};

use library::{
    interface::Image,
    math::{Matrix, Tuple},
    properties::Color,
    space::Shape,
    space::{Intersection, Ray, Sphere},
    Axis,
};

use sdl2_interface::Sdl2Interface;

fn hit<'a>(ray: &Ray, sphere: &'a Sphere) -> Option<Intersection<'a>> {
    // At this stage, shapes always returned ordered hits, so we can use the first.
    //
    sphere.intersections(ray).get(0).cloned()
}

pub fn practice() {
    let display_size: u16 = 100;
    let eye_z = -50.0;
    let display_z = 50.0;

    let (center_x, center_y) = ((display_size / 2) as i16, (display_size / 2) as i16);

    let mut interface = Sdl2Interface::init("Chapter 05 exercise", display_size, display_size);
    interface.invert_y = true;
    interface.origin = (center_x, center_y);

    let hit_color = Color::new(1, 0, 0);

    // let mut sphere = Sphere::equiscaled(12.5);

    // let sphere = Sphere::new()
    //     .scale(6.25, 12.5, 12.5)
    //     .rotate(Axis::Z, -PI / 4.0)
    //     .translate(10, 0, 0);

    let mut sphere = Sphere::default();
    let transformation = Matrix::translation(10, 0, 0)
        * &Matrix::rotation(Axis::Z, -PI / 4.0)
        * &Matrix::scaling(6.25, 12.5, 12.5);
    sphere.transform = transformation;

    let sphere = Arc::new(sphere);

    let ray_origin = Tuple::point(0, 0, eye_z);

    for y in -center_y..center_y {
        println!("Computing y: {}", y);

        for x in -center_x..center_x {
            let ray_direction = Tuple::vector(x as f64, y as f64, display_z - eye_z);

            let ray = Ray {
                origin: ray_origin,
                direction: ray_direction,
            };

            if hit(&ray, &sphere).is_some() {
                interface.write_pixel(x, y, hit_color);
            };
        }
    }

    interface.update();
    interface.wait_keypress();
}
