use std::f32;



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public data types
// --------------------------------------------------------------------------------------------------------------------------------------------------

///
/// Ray
#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: nalgebra::Vector4<f32>,
    pub direction: nalgebra::Vector4<f32>
}



///
/// Half-open interval [a,b)
pub struct Interval {
    pub start: f32,
    pub finish: f32
}


impl Interval {
    pub fn new(s: f32, f: f32) -> Self {
        Interval {
            start: f32::min(s, f),
            finish: f32::max(s, f)
        }
    }
}



///
/// Ray
#[derive(Clone, Copy)]
pub struct Segment {
    pub origin: nalgebra::Vector4<f32>,
    pub direction: nalgebra::Vector4<f32>,
    pub start: f32,
    pub finish: f32
}



///
/// Intersection against the tree
pub struct Intersection {
    pub distance: f32,
    pub alpha: f32,
    pub beta: f32,
    pub gamma: f32,
    pub v1: u32,
    pub v2: u32,
    pub v3: u32,
    pub material: u32
}
