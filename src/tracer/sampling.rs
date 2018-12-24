use std::f32;
use rand; //::{Rng, thread_rng};
use geometry;


use rayon::prelude::*;
//use rayon::iter::IntoParallelRefIterator;



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public data types
// --------------------------------------------------------------------------------------------------------------------------------------------------

pub struct Sampler2D {
    rng : rand::ThreadRng
}



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public functions
// --------------------------------------------------------------------------------------------------------------------------------------------------

pub fn sample_hemisphere(normal: nalgebra::Vector4<f32>, incident: nalgebra::Vector4<f32>) -> nalgebra::Vector4<f32> {
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

/*
    // Create the local frame
    let z_axis = normal.normalize();
    let y_axis = project_vector_onto_plane(incident.normalize(), normal.normalize()).normalize();
    let x_axis = nalgebra::cross(y_axis, z_axis);
    let frame = nalgebra::Matrix3 {
        xx: x_axis.x,
        xy: x_axis.y,
        xz: x_axis.z,

        yx: y_axis.x,
        yy: y_axis.y,
        yz: y_axis.z,

        zx: z_axis.x,
        zy: z_axis.y,
        zz: z_axis.z,
    };

    let u1 = rand::random::<f32>();
    let u2 = rand::random::<f32>();
    let r = u1.sqrt();
    let theta = 2.0 * 3.141592654 * u2;
 
    let x = r * theta.cos();
    let y = r * theta.sin();
    let z = (f32::max(0.0, 1.0 - u1)).sqrt();
    frame * nalgebra::Vector3{x:x, y:y, z:z}
*/
}



pub fn sample_scene(geometry: &super::Scene, ray: geometry::Ray, max_iter : usize) -> super::Color {
    if max_iter == 0 {
        super::BLACK
    } else {
        let hit = geometry.intersect(ray);
        
        match hit {
            None => super::BLACK,
            //None => super::WHITE,
            // Recursive tracing
            Some(intersection) => {
                let s = sample_hemisphere(intersection.normal, ray.direction);
                let r = geometry::Ray {
                    origin: intersection.point + 1.0e-3 * s,
                    direction: s
                };
                let d = sample_scene(geometry, r, max_iter-1);
                intersection.material.emission + intersection.normal.dot(&r.direction) * intersection.material.diffuse * d
            }
        }
    }
}



pub fn sample(scene: &super::Scene, camera: &super::Camera, max_bounces: usize) -> Vec<super::Color>
{
    let rays = camera.make_rays();
    let sampling = rays.par_iter().map(|&r| sample_scene(&scene, r, max_bounces)).collect();
    sampling
}
