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

pub const BLACK: super::Color = super::Color{
    r: 0.0,
    g: 0.0,
    b: 0.0
};



pub const WHITE: super::Color = super::Color{
    r: 1.0,
    g: 1.0,
    b: 1.0
};



impl ops::Add<super::Color> for Color {
    type Output = super::Color;

    fn add(self, _rhs: super::Color) -> super::Color {
        super::Color {
            r : self.r + _rhs.r,
            g : self.g + _rhs.g,
            b : self.b + _rhs.b
        }
    }
}



impl ops::Mul<super::Color> for f32 {
    type Output = super::Color;

    fn mul(self, _rhs: super::Color) -> super::Color {
        super::Color {
            r : self * _rhs.r,
            g : self * _rhs.g,
            b : self * _rhs.b
        }
    }
}



impl ops::Mul<super::Color> for Color {
    type Output = super::Color;

    fn mul(self, _rhs: super::Color) -> super::Color {
        super::Color {
            r : self.r * _rhs.r,
            g : self.g * _rhs.g,
            b : self.b * _rhs.b
        }
    }
}
