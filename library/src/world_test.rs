use demonstrate::demonstrate;

demonstrate! {
    describe "World" {
        use crate::*;

        before {
            let world = World {
                objects: vec![
                    Sphere::from(
                        None,
                        Some(Material {
                            color: Color::new(0.8, 1.0, 0.6),
                            ambient: 0.1,
                            diffuse: 0.7,
                            specular: 0.2,
                            shininess: 200.0,
                        }),
                    ),
                    Sphere::from(
                        Some(Matrix::scaling(0.5, 0.5, 0.5)),
                        None,
                    )
                ],
                light_source: PointLight {
                    position: Tuple::point(-10, 10, -10),
                    intensity: Color::new(1, 1, 1)
                },
            };
        }

        it "should intersect with a ray" {
            let ray = Ray {
                origin: Tuple::point(0, 0, -5),
                direction: Tuple::vector(0, 0, 1),
            };

            let intersections = world
                .intersections(&ray)
                .iter()
                .map(|intersection| intersection.t).collect::<Vec<_>>();

            let expected_intersections = vec![4.0, 4.5, 5.5, 6.0];

            assert_eq!(intersections, expected_intersections);
        }
    }
}
