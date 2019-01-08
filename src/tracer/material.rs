// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public data types
// --------------------------------------------------------------------------------------------------------------------------------------------------

#
[derive(Clone, Serialize, Deserialize)]
pub struct Material {
    pub diffuse: super::Color,
        pub emission: super::Color
}



impl Material {
    pub fn none() -> Self {
        Material {
            diffuse: super::BLACK,
            emission: super::BLACK
        }
    }
}
