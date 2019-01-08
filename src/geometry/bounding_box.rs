use std::f32;



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public data types
// --------------------------------------------------------------------------------------------------------------------------------------------------

///
/// Axis aligned bounding box
pub struct BoundingBox {
    pub lower: nalgebra::Vector3<f32>,
    pub upper: nalgebra::Vector3<f32>
}



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public functions
// --------------------------------------------------------------------------------------------------------------------------------------------------

///
/// Compute the bounding box
impl super::BoundingBox {
    pub fn build(vertices: &Vec<super::Vertex>) -> Self {
        // Initialization
        let mut lower = nalgebra::Vector3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut upper = nalgebra::Vector3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);

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
            upper: upper
        }
    }


    pub fn build2(vertices: &Vec<super::Vertex>, faces: &Vec<super::Triangle>) -> Self {
        // Compute the bounds of the geometry
        let mut lower = nalgebra::Vector3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut upper = nalgebra::Vector3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);
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
            upper: upper
        }
    }


    pub fn intersect(&self, ray: super::Ray) -> super::Interval {
        // Intersection on the X axis
        let x1 = (self.lower.x - ray.origin.x) / ray.direction.x;
        let x2 = (self.upper.x - ray.origin.x) / ray.direction.x;
        let x_int = super::Interval::new(x1, x2);

        // Intersection on the Y axis
        let y1 = (self.lower.y - ray.origin.y) / ray.direction.y;
        let y2 = (self.upper.y - ray.origin.y) / ray.direction.y;
        let y_int = super::Interval::new(y1, y2);

        // Intersection on the Z axis
        let z1 = (self.lower.z - ray.origin.z) / ray.direction.z;
        let z2 = (self.upper.z - ray.origin.z) / ray.direction.z;
        let z_int = super::Interval::new(z1, z2);

        // Global intersection
        let foo = f32::max(x_int.start, f32::max(y_int.start, z_int.start));
        let bar = f32::min(x_int.finish, f32::min(y_int.finish, z_int.finish));
        if bar >= foo {
            super::Interval::new(foo, bar)
        } else {
            super::Interval::new(f32::INFINITY, f32::INFINITY)
        }
    }
}



pub trait Boundable {
    fn bounds(&self) -> BoundingBox;
}
