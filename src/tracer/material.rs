use nalgebra;

use serde_json;

use std;
use std::f32;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::ops::Deref;



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public data types
// --------------------------------------------------------------------------------------------------------------------------------------------------

#[derive(Clone, Serialize, Deserialize)]
pub struct Material {
    pub diffuse: super::Color,
    pub emission: super::Color
}
