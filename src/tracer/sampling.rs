use ndarray::Zip;

use super::*;
use crate::geometry;

// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public functions
// --------------------------------------------------------------------------------------------------------------------------------------------------

pub fn sample(scene: &Scene, camera: &Camera, max_bounces: usize) -> Image2D {
    Zip::from(&camera.make_rays()).par_apply_collect(|r| trace_ray(scene, *r, max_bounces))

    //let rays = camera.make_rays();
    //sample_scene(scene, &rays, max_bounces)
}

// --------------------------------------------------------------------------------------------------------------------------------------------------
// Private functions
// --------------------------------------------------------------------------------------------------------------------------------------------------

/*fn sample_scene(scene: &Scene, rays: &[geometry::Ray], max_iter : usize) -> Vec<Color> {
    if max_iter == 0 {
        vec![Color::black(); rays.len()]
    } else {
        let intersections : Vec<SceneIntersection> = rays.par_iter().map(|&r| scene.intersect(r)).collect();
        let outgoing_rays : Vec<geometry::util::Ray> = intersections.par_iter().map(|hit| hit.material.spawn_secondary_ray(&hit)).collect();
        let incoming_color = sample_scene(scene, &outgoing_rays, max_iter-1);
        return izip!(intersections, outgoing_rays, incoming_color).map(|(hit, r, c)| hit.material.shade(hit.point, hit.ray.direction, hit.normal, r.direction, c)).collect();
    }
}*/

fn trace_ray(scene: &Scene, ray: geometry::Ray, max_bounces: usize) -> Color {
    if max_bounces == 0 {
        color::black()
    } else {
        let hit = scene.intersect(ray);
        if hit.distance.is_finite() {
            let material = &scene.materials[hit.material as usize];
            let outgoing_ray = material.spawn_secondary_ray(&hit);
            let incoming_color = trace_ray(scene, outgoing_ray, max_bounces - 1);
            material.shade(&hit, outgoing_ray.direction, incoming_color)
        } else {
            color::black()
        }
    }
}
