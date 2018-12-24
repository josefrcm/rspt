use nalgebra;

use serde_json;

use std;
use std::f32;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::ops::Deref;

use geometry;



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public data types
// --------------------------------------------------------------------------------------------------------------------------------------------------

#[derive(Serialize, Deserialize)]
pub struct InstanceDef {
    pub mesh: String,
    pub material: String/*,
    pub transform: linalg::Matrix4*/
}

#[derive(Serialize, Deserialize)]
pub struct SceneDef {
    pub materials: HashMap<String, super::Material>,
    pub meshes: Vec<InstanceDef>
}



///
/// World geometry
pub struct Scene {    
    pub materials: Vec<super::Material>,
    pub meshes: Vec<geometry::Mesh>
}



///
/// Intersection against the world
pub struct SceneIntersection<'a> {
    pub point: nalgebra::Vector4<f32>,
    pub normal: nalgebra::Vector4<f32>,
    pub distance: f32,
    pub material: &'a super::Material
}



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public functions
// --------------------------------------------------------------------------------------------------------------------------------------------------

// TODO: check keys
impl Scene {
    pub fn from_json(filename: &Path) -> Result<Self, std::io::Error> {
        // Load the scene description from the JSON file
        let file = File::open(filename)?;
        let json : SceneDef = serde_json::from_reader(file)?;

        // Load each model
        let base_dir = filename.parent().unwrap();
        let mut materials = Vec::new();
        let mut meshes = Vec::new();
        for m in &json.meshes {
            let material_num = materials.len();
            materials.push(json.materials.get(&m.material).unwrap().clone());

            let mesh_path = base_dir.join(&m.mesh);
            let mesh = geometry::Mesh::load_ply(mesh_path.deref(), material_num as u32)?;            
            meshes.push(mesh);
        }

        // Done
        Ok(Scene {
            materials: materials,
            meshes: meshes
        })
    }


    ///
    /// Intersect a ray against the world
    pub fn intersect(&self, ray: geometry::Ray) -> Option<SceneIntersection> {
        /*let segment = geometry::Segment {
            origin: ray.origin,
            direction: ray.direction,
            start: 0.0,
            finish: f32::INFINITY
        };*/

        let mut min_dist = f32::INFINITY;
        let mut best_hit = None;
        for m in self.meshes.iter() {
            match m.intersect(ray) {
                // If a triangle was hit, compute the intersection parameters: coordinates, normal, material, etc.

                Some(intersection) => {
                    if intersection.distance < min_dist {
                        min_dist = intersection.distance;
                        best_hit = Some(intersection);
                    }
                },
                None => {}
            }
        }

        match best_hit {
            Some(hit) => Some(SceneIntersection {
                    point: hit.point,
                    normal: hit.normal,
                    distance: hit.distance,
                    material: &self.materials[hit.material as usize]
                }),
            None => None
        }
    }
}
