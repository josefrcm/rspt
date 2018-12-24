use scene;

use std::f32;
use rand; //::{Rng, thread_rng};



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Data types
// --------------------------------------------------------------------------------------------------------------------------------------------------

pub struct Camera {
    pub position: nalgebra::Vector4<f32>,
    pub orientation: nalgebra::UnitQuaternion<f32>,
    pub resolution: usize
}



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Functions
// --------------------------------------------------------------------------------------------------------------------------------------------------

impl Camera {
    pub fn new(scene: &scene::Scene, resolution: usize) -> Camera {
        Camera {
            position: scene.camera.position,
            orientation: scene.camera.orientation,
            resolution: resolution
        }
    }

    pub fn make_rays(&self) -> Vec<super::Ray> {
        let mut rays: Vec<super::Ray> = Vec::new();
        let xbias = rand::random::<f32>();
        let ybias = rand::random::<f32>();
        for y in 0..self.resolution {
            for x in 0..self.resolution {
                let xr = 2.0*(x as f32 + xbias)/(self.resolution as f32) - 1.0;
                let yr = -2.0*(y as f32 + ybias)/(self.resolution as f32) + 1.0;
                let direction = nalgebra::Vector4::new(xr, 1.0, yr, 0.0).normalize();
                rays.push(super::Ray {
                    origin: self.position,
                    direction: direction
                });
            }
        }
        rays
    }
}
