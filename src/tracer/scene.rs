use nalgebra;

use serde_json;

use std;
use std::f32;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::ops::Deref;

use geometry;
use geometry::util::Intersectable;



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
    pub geometry: geometry::BVH<geometry::Mesh>
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
        materials.push(super::Material::none());

        let mut meshes = Vec::new();
        for m in &json.meshes {
            let material_num = materials.len();
            materials.push(json.materials.get(&m.material).unwrap().clone());

            let mesh_path = base_dir.join(&m.mesh);
            let mesh = geometry::Mesh::load_ply(mesh_path.deref(), material_num as u32)?;            
            meshes.push(mesh);
        }

        // Build the acceleration structure        
        let mut bundles = Vec::new();
        for m in meshes {
            let b = m.bounds();
            bundles.push((m, b));
        }
        let tree = geometry::BVH::build(bundles);

        // Done
        Ok(Scene {
            materials: materials,
            geometry: tree
        })
    }


    ///
    /// Intersect a ray against the world
    pub fn intersect(&self, ray: geometry::Ray) -> Option<SceneIntersection> {
        match self.geometry.intersect(ray) {
            None => None,
            Some(hit) => Some(SceneIntersection {
                point: hit.point,
                normal: hit.normal,
                distance: hit.distance,
                material: &self.materials[hit.material as usize]
            })
        }
    }
}
