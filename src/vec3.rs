use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

#[pymethods]
impl Vec3 {
    #[new]
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }
}

impl Vec3 {
    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
        self.y
    }
    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn get(&self, index: usize) -> f32 {
        if index == 0 {
            return self.x;
        } else if index == 1 {
            return self.y;
        } else if index == 2 {
            return self.z;
        }
        panic!(
            "Attempted to access Vec3 out of bounds with index: {} !",
            index
        );
    }

    pub fn set(&mut self, index: usize, value: f32) {
        if index == 0 {
            self.x = value;
        } else if index == 1 {
            self.y = value;
        } else if index == 2 {
            self.z = value;
        } else {
            panic!(
                "Attempted to access Vec3 out of bounds with index: {} !",
                index
            );
        }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn make_unit_vector(&mut self) {
        let k = 1.0 / self.length();
        self.x *= k;
        self.y *= k;
        self.z *= k;
    }

    pub fn unit_vector(&self) -> Vec3 {
        let k = 1.0 / self.length();
        Vec3::new(self.x * k, self.y * k, self.z * k)
    }

    pub fn dot(&self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            -(self.x * other.z - self.z * other.x),
            self.x * other.y - self.y * other.x,
        )
    }

    //Reflect this vector around vector n. used for metals.
    pub fn reflect(&self, n: Vec3) -> Vec3 {
        *self - 2.0 * self.dot(n) * n
    }

    //Refract this vector. used for dielectrics (such as glass).
    pub fn refract(&self, normal: Vec3, ni_over_nt: f32) -> Option<Vec3> {
        let uv = &self.unit_vector();
        let dt = uv.dot(normal);
        let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
        if discriminant > 0.0 {
            Some(ni_over_nt * (*uv - normal * dt) - normal * discriminant.sqrt())
        } else {
            None
        }
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3::new(self.x * other, self.y * other, self.z * other)
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self * other.x, self * other.y, self * other.z)
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Vec3 {
        Vec3::new(self.x / other, self.y / other, self.z / other)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_set() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v.set(0, 4.0);
        assert_eq!(v.x(), 4.0);
    }
}
