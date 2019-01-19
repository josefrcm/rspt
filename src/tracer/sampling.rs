use rand;
use rayon::prelude::*;
use std::f32;

use geometry;
use tracer::*;



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public functions
// --------------------------------------------------------------------------------------------------------------------------------------------------

pub fn sample(scene: &Scene, camera: &Camera, max_bounces: usize) -> Vec<Color>
{
    let rays = camera.make_rays();
    let sampling = rays.par_iter().map(|&r| sample_scene(scene, r, max_bounces)).collect();
    sampling
}



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Private functions
// --------------------------------------------------------------------------------------------------------------------------------------------------

///
/// 
pub fn sample_scene(scene: &Scene, ray: geometry::Ray, max_iter : usize) -> Color {
    if max_iter == 0 {
        Color::black()
    } else {
        match scene.intersect(ray) {
            None => Color::black(),
            Some(intersection) => {
                let r = intersection.material.spawn_secondary_ray(&intersection);
                let d = sample_scene(scene, r, max_iter-1);
                return intersection.material.shade(intersection.point, intersection.ray.direction, intersection.normal, r.direction, d);
            }
        }
    }
}
