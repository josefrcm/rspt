use std::ops;



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public data types
// --------------------------------------------------------------------------------------------------------------------------------------------------

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32
}



// --------------------------------------------------------------------------------------------------------------------------------------------------
// Public functions
// --------------------------------------------------------------------------------------------------------------------------------------------------

impl Color {
    pub fn black() -> Self {
        Color{
            r: 0.0,
            g: 0.0,
            b: 0.0
        }
    }


    pub fn white() -> Self {
        Color{
            r: 1.0,
            g: 1.0,
            b: 1.0
        }
    }
}



impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, _rhs: Color) -> Color {
        Color {
            r : self.r + _rhs.r,
            g : self.g + _rhs.g,
            b : self.b + _rhs.b
        }
    }
}



impl ops::Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, _rhs: Color) -> Color {
        Color {
            r : self * _rhs.r,
            g : self * _rhs.g,
            b : self * _rhs.b
        }
    }
}



impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, _rhs: Color) -> Color {
        Color {
            r : self.r * _rhs.r,
            g : self.g * _rhs.g,
            b : self.b * _rhs.b
        }
    }
}
