use std::f32;
use nalgebra;

use geometry;
use tracer::*;



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public data types
// --------------------------------------------------------------------------------------------------------------------------------------------------

#[derive(Clone, Serialize, Deserialize)]
pub enum Material {
    Light {
        emission: Color
    },
    Standard {
        emission: Color,
        diffuse: Color,
    }    
}



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public functions
// --------------------------------------------------------------------------------------------------------------------------------------------------

impl Material {
    pub fn none() -> Self {
        Material::Light {
            emission: Color::black()
        }
    }


    pub fn spawn_secondary_ray(&self, intersection: &SceneIntersection) -> geometry::Ray {
        match self {
            Material::Light { emission } => {
                geometry::Ray {
                    origin: nalgebra::Point3::new(f32::NAN, f32::NAN, f32::NAN),
                    direction: nalgebra::Vector3::new(f32::NAN, f32::NAN, f32::NAN),
                }
            },
            Material::Standard { emission, diffuse } => {
                sample_hemisphere(intersection.point, intersection.normal, intersection.ray)
            }
        }      
    }


    pub fn shade(&self, point: nalgebra::Point3<f32>, incident: nalgebra::Vector3<f32>, normal: nalgebra::Vector3<f32>, outgoing: nalgebra::Vector3<f32>, foobar: Color) -> Color {
        match self {
            Material::Light { emission } => {
                *emission
            },
            Material::Standard { emission, diffuse } => {
                (*emission) + (-normal.dot(&incident) * (*diffuse) * foobar)
            }
        }
    }
}




// --------------------------------------------------------------------------------------------------------------------------------------------------
// Private functions
// --------------------------------------------------------------------------------------------------------------------------------------------------

fn sample_hemisphere(point: nalgebra::Point3<f32>, normal: nalgebra::Vector3<f32>, incident: geometry::Ray) -> geometry::Ray {
    let x = 2.0 * rand::random::<f32>() - 1.0;
    let y = 2.0 * rand::random::<f32>() - 1.0;
    let z = 2.0 * rand::random::<f32>() - 1.0;
    let s = (x*x + y*y + z*z).sqrt();

    let mut d = nalgebra::Vector3::new(x/s, y/s, z/s);
    
    if d.dot(&normal) < 0.0 {
        d.x = -d.x;
        d.y = -d.y;
        d.z = -d.z;
    }
    
    geometry::Ray {
        origin: point + 1.0e-3 * d,
        direction: d
    }
}



/*fn sample_hemisphere(point: nalgebra::Point3<f32>, normal: nalgebra::Vector3<f32>, incident: geometry::Ray) -> geometry::Ray {
    // Create the local frame
    let z_axis : nalgebra::Vector3<f32> = normal.normalize();
    let y_axis : nalgebra::Vector3<f32> = project_vector_onto_plane(incident.direction.normalize(), normal.normalize()).normalize();
    let x_axis : nalgebra::Vector3<f32> = y_axis.cross(&z_axis);
    let frame = nalgebra::Matrix3::from_rows(&[x_axis.transpose(), y_axis.transpose(), z_axis.transpose()]).try_inverse().unwrap();

    let u1 = rand::random::<f32>();
    let u2 = rand::random::<f32>();
    let r = u1.sqrt();
    let theta = 2.0 * 3.141592654 * u2;
 
    let x = r * theta.cos();
    let y = r * theta.sin();
    let z = (f32::max(0.0, 1.0 - u1)).sqrt();
    let d = frame * nalgebra::Vector3::new(x, y, z);

    geometry::Ray {
        origin: point + 1.0e-3 * d,
        direction: d
    }
}



fn project_vector_onto_plane(vector: nalgebra::Vector3<f32>, plane: nalgebra::Vector3<f32>) -> nalgebra::Vector3<f32> {
    vector - vector.dot(&plane) * plane
}*/
