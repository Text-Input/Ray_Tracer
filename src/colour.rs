#[derive(Debug, Clone, Copy)]
pub struct Colour {
    r: f32,
    g: f32,
    b: f32,
}

impl Colour {
    pub fn new(r: f32, g: f32, b: f32) -> Colour {
        Colour { r, g, b }
    }

    pub fn r(&self) -> f32 {
        self.r
    }
    pub fn g(&self) -> f32 {
        self.g
    }
    pub fn b(&self) -> f32 {
        self.b
    }
}

impl std::ops::Add for Colour {
    type Output = Colour;

    fn add(self, other: Colour) -> Colour {
        Colour::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}

impl std::ops::Sub for Colour {
    type Output = Colour;

    fn sub(self, other: Colour) -> Colour {
        Colour::new(self.r - other.r, self.g - other.g, self.b - other.b)
    }
}

impl std::ops::Mul<Colour> for Colour {
    type Output = Colour;

    fn mul(self, other: Colour) -> Colour {
        Colour::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }
}

impl std::ops::Mul<f32> for Colour {
    type Output = Colour;

    fn mul(self, other: f32) -> Colour {
        Colour::new(self.r * other, self.g * other, self.b * other)
    }
}

impl std::ops::Mul<Colour> for f32 {
    type Output = Colour;

    fn mul(self, other: Colour) -> Colour {
        Colour::new(self * other.r, self * other.g, self * other.b)
    }
}

impl std::ops::Div<f32> for Colour {
    type Output = Colour;

    fn div(self, other: f32) -> Colour {
        Colour::new(self.r / other, self.g / other, self.b / other)
    }
}
