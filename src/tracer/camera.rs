use std;
use std::f32;
use std::fs::File;
use std::path::Path;

use rand;
use ron;

use crate::geometry;

// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public data types
// --------------------------------------------------------------------------------------------------------------------------------------------------

///
/// Camera definition
#[derive(Clone, Serialize, Deserialize)]
struct CameraDef {
    pub position: nalgebra::Point3<f32>,
    pub orientation: nalgebra::UnitQuaternion<f32>,
    pub focal: f32,
}

///
/// Camera
pub struct Camera {
    pub position: nalgebra::Point3<f32>,
    pub orientation: nalgebra::UnitQuaternion<f32>,
    pub width: usize,
    pub height: usize,
    pub focal: f32,
    pub aspect: f32,
}

// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public functions
// --------------------------------------------------------------------------------------------------------------------------------------------------

impl Camera {
    ///
    /// Load the camera description from a JSON file
    pub fn from_json(
        filename: &Path,
        width: usize,
        height: usize,
    ) -> Result<Camera, std::io::Error> {
        // Load the camera description from the JSON file
        let file = File::open(filename)?;
        let json: CameraDef = ron::de::from_reader(file).unwrap();

        // Build the camera
        Ok(Camera {
            position: json.position,
            orientation: json.orientation,
            width: width,
            height: height,
            focal: json.focal,
            aspect: (width as f32) / (height as f32),
        })
    }

    ///
    /// Trace rays from the camera
    pub fn make_rays(&self) -> ndarray::Array2<geometry::Ray> {
        let xbias = rand::random::<f32>() - (self.width as f32) / 2.0;
        let ybias = rand::random::<f32>() - (self.height as f32) / 2.0;
        let foobar = 1.0 / f32::min(self.width as f32, self.height as f32);

        ndarray::Array2::from_shape_fn([self.height, self.width], |(y, x)| {
            let xr = foobar * (x as f32 + xbias);
            let yr = -foobar * (y as f32 + ybias);
            let direction = nalgebra::Vector3::new(xr, self.focal, yr).normalize();
            geometry::Ray {
                origin: self.position,
                direction: direction,
            }
        })
    }
}
