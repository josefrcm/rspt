use tracer::*;



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public data types
// --------------------------------------------------------------------------------------------------------------------------------------------------

#[derive(Clone, Serialize, Deserialize)]
pub struct Material {
    pub diffuse: Color,
        pub emission: Color
}



impl Material {
    pub fn none() -> Self {
        Material {
            diffuse: Color::black(),
            emission: Color::black()
        }
    }
}
