use std::ops::{Add, Div, Mul};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r: r, g: g, b: b }
    }

    pub fn black() -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn white() -> Color {
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }

    pub fn red() -> Color {
        Color {
            r: 1.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn green() -> Color {
        Color {
            r: 0.0,
            g: 1.0,
            b: 0.0,
        }
    }

    pub fn blue() -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 1.0,
        }
    }

    pub fn gamma2(&self) -> Color {
        Color::new(self.r.sqrt(), self.g.sqrt(), self.b.sqrt())
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, color: Color) -> Color {
        Color::new(self.r * color.r, self.g * color.g, self.b * color.b)
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, val: f64) -> Color {
        Color::new(self.r * val, self.g * val, self.b * val)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, color: Color) -> Color {
        color * self
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, val: f64) -> Color {
        Color::new(self.r / val, self.g / val, self.b / val)
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, color: Color) -> Color {
        Color::new(self.r + color.r, self.g + color.g, self.b + color.b)
    }
}
