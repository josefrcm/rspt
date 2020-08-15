use rayon::prelude::*;

use crate::geometry;
use super::*;

// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public functions
// --------------------------------------------------------------------------------------------------------------------------------------------------

pub fn sample(scene: &Scene, camera: &Camera, max_bounces: usize) -> Vec<Color> {
    camera
        .make_rays()
        .par_iter()
        .map(|r| trace_ray(scene, *r, max_bounces))
        .collect()

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
        Color::black()
    } else {
        let hit = scene.intersect(ray);
        let outgoing_ray = hit.material.spawn_secondary_ray(&hit.point);
        let incoming_color = trace_ray(scene, outgoing_ray, max_bounces - 1);
        hit.material
            .shade(&hit.point, outgoing_ray.direction, incoming_color)
    }
}
