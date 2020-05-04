use super::vec3::*;

pub fn random_in_unit_sphere() -> Vec3 {
    let mut p;
    loop {
        p = 2.0 * Vec3::new(rand::random(), rand::random(), rand::random())
            - Vec3::new(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            break;
        }
    }
    p
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut p;
    loop {
        p = 2.0 * Vec3::new(rand::random(), rand::random(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        if p.dot(p) < 1.0 {
            break;
        }
    }
    p
}
