use super::vec3::*;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
    inv_direction: Vec3, // stores the inverse of the direction, for speed.
    dir_sign: [bool; 3], //stores the sign of the direction, for speed.
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        let inv_direction = Vec3::new(
            1.0 / direction.x(),
            1.0 / direction.y(),
            1.0 / direction.z(),
        );
        let dir_sign = [
            direction.x() < 0.0,
            direction.y() < 0.0,
            direction.z() < 0.0,
        ];
        Ray {
            origin,
            direction,
            inv_direction,
            dir_sign,
        }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn inv_direction(&self) -> Vec3 {
        self.inv_direction
    }

    pub fn dir_sign(&self) -> [bool; 3] {
        self.dir_sign
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}
