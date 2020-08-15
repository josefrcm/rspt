use std;
use std::collections::HashMap;
use std::f32;
use std::fs::File;
use std::ops::Deref;
use std::path::Path;

use nalgebra;
use ron;

use crate::geometry;
use crate::geometry::util::Intersectable;
use super::*;

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
    pub geometry: geometry::BVH<geometry::Mesh>,
}

///
/// Point in space where the intersection took place
pub struct IntersectionPoint {
    pub ray: geometry::Ray,
    pub point: nalgebra::Point3<f32>,
    pub normal: nalgebra::Vector3<f32>,
    pub distance: f32,
}

///
/// Intersection against the world
pub struct SceneIntersection<'a> {
    pub point: IntersectionPoint,
    pub material: &'a Material,
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
        materials.push(Material::none());

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
        let tree = geometry::BVH::build(&bundles);

        // Done
        Ok(Scene {
            materials: materials,
            geometry: tree,
        })
    }

    ///
    /// Intersect a ray against the world
    pub fn intersect(&self, ray: geometry::Ray) -> SceneIntersection {
        if ray.direction.x.is_nan() || ray.direction.y.is_nan() || ray.direction.z.is_nan() {
            SceneIntersection {
                point: IntersectionPoint {
                    ray: ray,
                    point: nalgebra::Point3::new(f32::NAN, f32::NAN, f32::NAN),
                    normal: nalgebra::Vector3::new(f32::NAN, f32::NAN, f32::NAN),
                    distance: f32::INFINITY,
                },
                material: &self.materials[0],
            }
        } else {
            match self.geometry.intersect(ray) {
                None => SceneIntersection {
                    point: IntersectionPoint {
                        ray: ray,
                        point: nalgebra::Point3::new(f32::NAN, f32::NAN, f32::NAN),
                        normal: nalgebra::Vector3::new(f32::NAN, f32::NAN, f32::NAN),
                        distance: f32::INFINITY,
                    },
                    material: &self.materials[0],
                },
                Some(hit) => SceneIntersection {
                    point: IntersectionPoint {
                        ray: ray,
                        point: hit.point,
                        normal: hit.normal,
                        distance: hit.distance,
                    },
                    material: &self.materials[hit.material as usize],
                },
            }
        }
    }
}
