use std::collections::HashMap;



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Data types
// --------------------------------------------------------------------------------------------------------------------------------------------------

#[derive(Clone, Serialize, Deserialize)]
pub struct Camera {
    pub position: nalgebra::Vector4<f32>,
    pub orientation: nalgebra::UnitQuaternion<f32>
}



#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32
}


#[derive(Clone, Serialize, Deserialize)]
pub struct Material {
    pub diffuse: Color,
    pub emission: Color
}


#[derive(Clone, Serialize, Deserialize)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub nx: f32,
    pub ny: f32,
    pub nz: f32
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Triangle {
    pub v1: u32,
    pub v2: u32,
    pub v3: u32
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Triangle>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Instance {
    pub mesh: String,
    pub material: String/*,
    pub transform: linalg::Matrix4*/
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Scene {
    pub camera: Camera,
    pub materials: HashMap<String, Material>,
    pub meshes: HashMap<String, Mesh>,
    pub instances: Vec<Instance>
}
