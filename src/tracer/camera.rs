use std;
use std::f32;
use std::fs::File;
use std::path::Path;

use rand;
use serde_json;

use geometry;
use tracer::*;



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public data types
// --------------------------------------------------------------------------------------------------------------------------------------------------

///
/// Camera definition
#[derive(Clone, Serialize, Deserialize)]
struct CameraDef {
    pub position: nalgebra::Point3<f32>,
    pub orientation: nalgebra::UnitQuaternion<f32>
}



///
/// Camera
pub struct Camera {
    pub position: nalgebra::Point3<f32>,
    pub orientation: nalgebra::UnitQuaternion<f32>,
    pub resolution: usize
}



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public functions
// --------------------------------------------------------------------------------------------------------------------------------------------------

impl Camera {
    ///
    /// Load the camera description from a JSON file
    pub fn from_json(filename: &Path, resolution: usize) -> Result<Camera, std::io::Error> {
        // Load the camera description from the JSON file
        let file = File::open(filename)?;
        let json : CameraDef = serde_json::from_reader(file)?;

        // Build the camera
        Ok(Camera {
            position: json.position,
            orientation: json.orientation,
            resolution: resolution
        })
    }



    ///
    /// Trace rays from the camera
    /// TODO: add 
    pub fn make_rays(&self) -> Vec<geometry::Ray> {
        let mut rays: Vec<geometry::Ray> = Vec::new();
        let xbias = rand::random::<f32>();
        let ybias = rand::random::<f32>();
        for y in 0..self.resolution {
            for x in 0..self.resolution {
                let xr = 2.0*(x as f32 + xbias)/(self.resolution as f32) - 1.0;
                let yr = -2.0*(y as f32 + ybias)/(self.resolution as f32) + 1.0;
                let direction = nalgebra::Vector3::new(xr, 1.0, yr).normalize();
                rays.push(geometry::Ray {
                    origin: self.position,
                    direction: direction
                });
            }
        }
        rays
    }
}
