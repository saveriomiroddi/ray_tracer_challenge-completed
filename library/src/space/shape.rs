use std::{
    fmt,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc, Weak,
    },
};

use super::{BoundedShape, Bounds, Intersection, PointLight, Ray};
use crate::{
    math::{Matrix, Tuple},
    properties::{Color, Material},
};

#[cfg(test)]
use std::any::Any;

static NEXT_ID: AtomicU32 = AtomicU32::new(1);

pub(crate) fn new_shape_id() -> u32 {
    NEXT_ID.fetch_add(1, Ordering::SeqCst)
}

pub(crate) mod private {
    use super::Ray;
    use crate::{math::Tuple, space::Intersection};

    pub trait ShapeLocal {
        // point: In object space.
        //
        // See `Shape#normal()` for the `intersection` explanation.
        //
        // In the book, this is local_normal_at().
        //
        fn local_normal(&self, point: Tuple, intersection: &Intersection) -> Tuple;

        // ray: In object space.
        //
        // In the book, this is local_intersect(), and returns also the shapes.
        //
        fn local_intersections<'a>(&'a self, ray: &Ray) -> Vec<Intersection<'a>>;
    }
}

// Inheritance has been implemented in a pure-trait form (base/sub traits + macro).
// Implementing it as base type + sub trait + sub types would have simplified the code. It'd be interesting
// to see if that implementation has a measurable performance difference (likely not).
// Among the other things, a base type would have made possible/easy to implement transformation chainable
// methods (see sphere.rs history).
// If the current design had to be pushed further, an attribute macro should be written, in order to
// deduplicate the attributes; attributes macros have very little documentation around, though.
//
pub trait Shape: private::ShapeLocal + BoundedShape + fmt::Debug + Sync + Send {
    fn id(&self) -> u32;
    fn parent(&self) -> Option<Arc<dyn Shape>>;
    fn parent_mut(&mut self) -> &mut Weak<dyn Shape>;
    fn transform(&self) -> &Matrix;
    fn transform_mut(&mut self) -> &mut Matrix;
    fn material(&self) -> &Material;
    fn material_mut(&mut self) -> &mut Material;

    // The `intersection` is used only by smooth triangles, but it's not an option because it's always
    // passed when computing the IntersectionState.
    // In tests, just pass the `Intersection::default()`.
    //
    // In the book, this is normal_at().
    //
    fn normal(&self, world_point: &Tuple, intersection: &Intersection) -> Tuple {
        let local_point = self.world_to_object(world_point);
        let local_normal = self.local_normal(local_point, intersection);
        self.normal_to_world(&local_normal)
    }

    // point: In world space.
    //
    fn world_to_object(&self, point: &Tuple) -> Tuple {
        let transform_inverse = self.transform().inverse();

        if let Some(parent) = self.parent() {
            transform_inverse * &parent.world_to_object(point)
        } else {
            transform_inverse * point
        }
    }

    // normal: In object space.
    //
    fn normal_to_world(&self, normal: &Tuple) -> Tuple {
        let mut normal = self.transform().inverse().transpose() * normal;
        normal.w = 0.0;
        normal = normal.normalize();

        if let Some(parent) = self.parent() {
            parent.normal_to_world(&normal)
        } else {
            normal
        }
    }

    // Return value properties:
    //
    // - they're not guaranteed to be ordered;
    // - negative values are allowed (required to compute refraction indexes).
    //
    // A possible optimization is to pass from the top an ordered collection (e.g. BTreeSet), and add
    // the intersections while traversing the tree, instead of creating separate arrays and sorting
    // the end result. This is a valid design even without considering the performance, as it fits nicely.
    //
    fn intersections<'a>(&'a self, ray: &Ray) -> Vec<Intersection<'a>> {
        let transformed_ray = ray.inverse_transform(self.transform());
        self.local_intersections(&transformed_ray)
    }

    // Default implementation, for non-nested shapes.
    //
    fn includes(&self, object: &dyn Shape) -> bool {
        self.id() == object.id()
    }

    // Local (object-level) bounds, with the shape transformation applied.
    //
    fn bounds(&self) -> Bounds {
        let local_bounds = self.local_bounds();

        // This ugly processing (disassemble/transform/reassemble) is required because after the transformation,
        // the bounding box can change completely.
        // Some negligible processing is spared by flattening the transformations (see comment on the
        // Group type).

        let local_corners = [
            Tuple::point(local_bounds.min.x, local_bounds.min.y, local_bounds.min.z),
            Tuple::point(local_bounds.min.x, local_bounds.min.y, local_bounds.max.z),
            Tuple::point(local_bounds.min.x, local_bounds.max.y, local_bounds.min.z),
            Tuple::point(local_bounds.min.x, local_bounds.max.y, local_bounds.max.z),
            Tuple::point(local_bounds.max.x, local_bounds.min.y, local_bounds.min.z),
            Tuple::point(local_bounds.max.x, local_bounds.min.y, local_bounds.max.z),
            Tuple::point(local_bounds.max.x, local_bounds.max.y, local_bounds.min.z),
            Tuple::point(local_bounds.max.x, local_bounds.max.y, local_bounds.max.z),
        ];

        let transform = self.transform();
        let mut bounds = Bounds::default();

        for corner in local_corners.iter() {
            let transformed_corner = transform * corner;

            Bounds::update_from_tuple(&mut bounds, &transformed_corner);
        }

        bounds
    }

    // Divergence from the book design. Having the lighting method here avoids going back and forth
    // between Shape and Material, and makes World#shade_hit cleaner.
    //
    fn lighting(
        &self,
        light: &PointLight,
        world_point: &Tuple,
        eyev: &Tuple,
        normalv: &Tuple,
        in_shadow: bool,
    ) -> Color {
        let object_point = self.world_to_object(&world_point);

        self.material()
            .lighting(light, &object_point, world_point, eyev, normalv, in_shadow)
    }

    #[cfg(test)]
    fn as_any(&self) -> &dyn Any;
}

impl PartialEq for dyn Shape + '_ {
    fn eq(&self, rhs: &Self) -> bool {
        self.id() == rhs.id()
    }
}
