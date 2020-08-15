use std::f32;

use super::*;

// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public data types
// --------------------------------------------------------------------------------------------------------------------------------------------------

///
/// Axis aligned bounding box
#[derive(Clone, Copy)]
pub struct BoundingBox {
    pub lower: nalgebra::Point3<f32>,
    pub upper: nalgebra::Point3<f32>,
}

// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public functions
// --------------------------------------------------------------------------------------------------------------------------------------------------

///
/// Compute the bounding box
impl BoundingBox {
    ///
    /// Create an empty bounding box, that doesn't use any space
    pub fn empty() -> Self {
        BoundingBox {
            lower: nalgebra::Point3::new(f32::NAN, f32::NAN, f32::NAN),
            upper: nalgebra::Point3::new(f32::NAN, f32::NAN, f32::NAN),
        }
    }

    ///
    /// Extract the bounding box of a group of vertices
    pub fn build(vertices: &[Vertex]) -> Self {
        // Initialization
        let mut lower = nalgebra::Point3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut upper =
            nalgebra::Point3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);

        // Find the lower and upper values
        for v in vertices {
            lower.x = f32::min(lower.x, v.coords.x);
            lower.y = f32::min(lower.y, v.coords.y);
            lower.z = f32::min(lower.z, v.coords.z);

            upper.x = f32::max(upper.x, v.coords.x);
            upper.y = f32::max(upper.y, v.coords.y);
            upper.z = f32::max(upper.z, v.coords.z);
        }

        // Done
        BoundingBox {
            lower: lower,
            upper: upper,
        }
    }

    ///
    /// Extract the bounding box of a group of faces and its corresponding vertices
    pub fn build2(vertices: &[Vertex], faces: &[Triangle]) -> Self {
        // Compute the bounds of the geometry
        let mut lower = nalgebra::Point3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut upper =
            nalgebra::Point3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);
        for f in faces {
            let v1 = vertices[f.v1 as usize].coords;
            let v2 = vertices[f.v2 as usize].coords;
            let v3 = vertices[f.v3 as usize].coords;

            lower.x = f32::min(lower.x, v1.x);
            lower.x = f32::min(lower.x, v2.x);
            lower.x = f32::min(lower.x, v3.x);

            lower.y = f32::min(lower.y, v1.y);
            lower.y = f32::min(lower.y, v2.y);
            lower.y = f32::min(lower.y, v3.y);

            lower.z = f32::min(lower.z, v1.z);
            lower.z = f32::min(lower.z, v2.z);
            lower.z = f32::min(lower.z, v3.z);

            upper.x = f32::max(upper.x, v1.x);
            upper.x = f32::max(upper.x, v2.x);
            upper.x = f32::max(upper.x, v3.x);

            upper.y = f32::max(upper.y, v1.y);
            upper.y = f32::max(upper.y, v2.y);
            upper.y = f32::max(upper.y, v3.y);

            upper.z = f32::max(upper.z, v1.z);
            upper.z = f32::max(upper.z, v2.z);
            upper.z = f32::max(upper.z, v3.z);
        }

        // Done
        BoundingBox {
            lower: lower,
            upper: upper,
        }
    }

    ///
    /// Compute the intersection of a ray against a bounding box
    pub fn intersect(&self, ray: Ray) -> Interval {
        // Intersection on the X axis
        let x1 = (self.lower.x - ray.origin.x) / ray.direction.x;
        let x2 = (self.upper.x - ray.origin.x) / ray.direction.x;
        let x_int = Interval::new(x1, x2);

        // Intersection on the Y axis
        let y1 = (self.lower.y - ray.origin.y) / ray.direction.y;
        let y2 = (self.upper.y - ray.origin.y) / ray.direction.y;
        let y_int = Interval::new(y1, y2);

        // Intersection on the Z axis
        let z1 = (self.lower.z - ray.origin.z) / ray.direction.z;
        let z2 = (self.upper.z - ray.origin.z) / ray.direction.z;
        let z_int = Interval::new(z1, z2);

        // Global intersection
        let foo = f32::max(x_int.start, f32::max(y_int.start, z_int.start));
        let bar = f32::min(x_int.finish, f32::min(y_int.finish, z_int.finish));
        if bar >= foo {
            Interval::new(foo, bar)
        } else {
            Interval::new(f32::INFINITY, f32::INFINITY)
        }
    }
}

///
/// Compute the union of two bounding boxes
pub fn union(boxes: &[BoundingBox]) -> BoundingBox {
    // initialization
    let mut lower = nalgebra::Point3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
    let mut upper = nalgebra::Point3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);

    // Compute the bounds of the geometry
    for b in boxes {
        lower.x = f32::min(lower.x, b.lower.x);
        lower.y = f32::min(lower.y, b.lower.y);
        lower.z = f32::min(lower.z, b.lower.z);

        upper.x = f32::max(upper.x, b.upper.x);
        upper.y = f32::max(upper.y, b.upper.y);
        upper.z = f32::max(upper.z, b.upper.z);
    }

    // Done
    BoundingBox {
        lower: lower,
        upper: upper,
    }
}
