use std::ops;
use scene::Color;


pub const BLACK: Color = Color{
    r: 0.0,
    g: 0.0,
    b: 0.0
};




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
