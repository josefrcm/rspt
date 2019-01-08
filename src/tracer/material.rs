// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public data types
// --------------------------------------------------------------------------------------------------------------------------------------------------

#[derive(Clone, Serialize, Deserialize)]
pub struct Material {
    pub diffuse: super::Color,
    pub emission: super::Color
}
