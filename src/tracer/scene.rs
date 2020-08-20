use ron;
use std;
use std::collections::HashMap;
use std::fs::File;
use std::ops::Deref;
use std::path::Path;

use super::*;
use crate::geometry;

// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public data types
// --------------------------------------------------------------------------------------------------------------------------------------------------

#[derive(Serialize, Deserialize)]
pub struct InstanceDef {
    pub mesh: String,
    pub material: String, /*,
                          pub transform: linalg::Matrix4*/
}

#[derive(Serialize, Deserialize)]
pub struct SceneDef {
    pub materials: HashMap<String, Material>,
    pub meshes: Vec<InstanceDef>,
}

///
/// World geometry
pub struct Scene {
    pub materials: Vec<Material>,
    //pub geometry: geometry::BVH,
    pub geometry: Vec<geometry::Mesh>,
}

// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public functions
// --------------------------------------------------------------------------------------------------------------------------------------------------

// TODO: check keys
impl Scene {
    pub fn from_json(filename: &Path) -> Result<Self, std::io::Error> {
        // Load the scene description from the JSON file
        let file = File::open(filename)?;
        let json: SceneDef = ron::de::from_reader(file).unwrap();

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

        // Build the acceleration structure
        //let tree = geometry::BVH::build_world(meshes);

        // Done
        Ok(Scene {
            materials: materials,
            geometry: meshes,
        })
    }

    ///
    /// Intersect a ray against the world
    pub fn intersect(&self, ray: geometry::Ray) -> geometry::MeshIntersection {
        let mut result = geometry::MeshIntersection::empty();

        if ray.direction.x.is_finite() && ray.direction.y.is_finite() && ray.direction.z.is_finite() {
            for m in &self.geometry {
                let hit = m.intersect(ray);
                if hit.distance < result.distance {
                    result = hit;
                }
            }
        }

        return result;
    }
}
