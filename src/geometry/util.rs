use std::f32;
use geometry::*;



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public data types
// --------------------------------------------------------------------------------------------------------------------------------------------------

///
/// Ray
#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: nalgebra::Point3<f32>,
    pub direction: nalgebra::Vector3<f32>
}



///
/// Half-open interval [a,b)
pub struct Interval {
    pub start: f32,
    pub finish: f32
}



/*///
/// Ray
#[derive(Clone, Copy)]
pub struct Segment {
    pub origin: nalgebra::Vector4<f32>,
    pub direction: nalgebra::Vector4<f32>,
    pub start: f32,
    pub finish: f32
}*/


///
/// Barycentric coordinates on a triangle
pub struct Barycentric {
    pub alpha: f32,
    pub beta: f32,
    pub gamma: f32
}



///
/// Result of a mesh/world-ray intersection
pub struct Intersection {
    pub distance: f32,
    pub alpha: f32,
    pub beta: f32,
    pub gamma: f32,
    pub v1: u32,
    pub v2: u32,
    pub v3: u32,
    pub material: u32,
    pub point: nalgebra::Point3<f32>,
    pub normal: nalgebra::Vector3<f32>
}



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public traits
// --------------------------------------------------------------------------------------------------------------------------------------------------

pub trait Intersectable {
    fn intersect(&self, ray: Ray) -> Option<Intersection>;
}



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public methods
// --------------------------------------------------------------------------------------------------------------------------------------------------

impl Interval {
    pub fn new(s: f32, f: f32) -> Self {
        Interval {
            start: f32::min(s, f),
            finish: f32::max(s, f)
        }
    }
}



impl Intersection {
    pub fn empty() -> Self {
        Intersection {
            distance: std::f32::INFINITY,
            alpha: 0.0,
            beta: 0.0,
            gamma: 0.0,
            v1: 0,
            v2: 0,
            v3: 0,
            material: 0,
            point: nalgebra::origin(),
            normal: nalgebra::zero()
        }
    }     
}
