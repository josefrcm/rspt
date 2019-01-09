use rand;
use rayon::prelude::*;
use std::f32;

use geometry;



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public functions
// --------------------------------------------------------------------------------------------------------------------------------------------------

pub fn sample(scene: &super::Scene, camera: &super::Camera, max_bounces: usize) -> Vec<super::Color>
{
    let rays = camera.make_rays();
    let sampling = rays.par_iter().map(|&r| sample_scene(&scene, r, max_bounces)).collect();
    sampling
}



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Private functions
// --------------------------------------------------------------------------------------------------------------------------------------------------

fn sample_scene(geometry: &super::Scene, ray: geometry::Ray, max_iter : usize) -> super::Color {
    if max_iter == 0 {
        super::BLACK
    } else {
        let hit = geometry.intersect(ray);
        
        match hit {
            None => super::BLACK,
            //None => super::WHITE,
            // Recursive tracing
            Some(intersection) => {
                let s = sample_hemisphere(intersection.normal.xyz(), ray.direction.xyz());
                let s2 = nalgebra::Vector4::new(s.x, s.y, s.z, 0.0);
                let r = geometry::Ray {
                    origin: intersection.point + 1.0e-3 * s2,
                    direction: s2
                };
                let d = sample_scene(geometry, r, max_iter-1);
                intersection.material.emission + intersection.normal.dot(&r.direction) * intersection.material.diffuse * d
            }
        }
    }
}



/*fn sample_hemisphere(normal: nalgebra::Vector3<f32>, incident: nalgebra::Vector3<f32>) -> nalgebra::Vector3<f32> {
    let x = 2.0 * rand::random::<f32>() - 1.0;
    let y = 2.0 * rand::random::<f32>() - 1.0;
    let z = 2.0 * rand::random::<f32>() - 1.0;
    let d = (x*x + y*y + z*z).sqrt();

    let mut s = nalgebra::Vector4::new(x/d, y/d, z/d, 0.0);
    
    if s.dot(&normal) < 0.0 {
        s.x = -s.x;
        s.y = -s.y;
        s.z = -s.z;
    }
    
    s
}*/



fn sample_hemisphere(normal: nalgebra::Vector3<f32>, incident: nalgebra::Vector3<f32>) -> nalgebra::Vector3<f32> {
    // Create the local frame
    let z_axis : nalgebra::Vector3<f32> = normal.normalize();
    let y_axis : nalgebra::Vector3<f32> = project_vector_onto_plane(incident.normalize(), normal.normalize()).normalize();
    let x_axis : nalgebra::Vector3<f32> = y_axis.cross(&z_axis);
    let frame = nalgebra::Matrix3::from_rows(&[x_axis.transpose(), y_axis.transpose(), z_axis.transpose()]).try_inverse().unwrap();

    let u1 = rand::random::<f32>();
    let u2 = rand::random::<f32>();
    let r = u1.sqrt();
    let theta = 2.0 * 3.141592654 * u2;
 
    let x = r * theta.cos();
    let y = r * theta.sin();
    let z = (f32::max(0.0, 1.0 - u1)).sqrt();
    frame * nalgebra::Vector3::new(x, y, z)
}



fn project_vector_onto_plane(vector: nalgebra::Vector3<f32>, plane: nalgebra::Vector3<f32>) -> nalgebra::Vector3<f32> {
    vector - vector.dot(&plane) * plane
}
