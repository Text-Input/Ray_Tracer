pub mod bvh;
pub mod constant_medium;
pub mod hitable_list;
pub mod instancing;
pub mod rectangle;
pub mod sphere;
pub mod triangle;

use super::aabb::*;
use super::material::*;
use super::ray::*;
use super::vec3::*;

//data about a ray hitting something.
#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
    pub t: f32,
    pub position: Vec3,
    pub normal: Vec3,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        t: f32,
        position: Vec3,
        incoming: &Ray,
        outward_normal: Vec3,
        material: &'a dyn Material,
    ) -> HitRecord<'a> {
        let front_face = incoming.direction().dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        HitRecord {
            t,
            position,
            normal,
            front_face,
            material,
        }
    }
}

//implemented by objects in the scene, so they can be hit by the rays.
pub trait Hitable: Sync + Send + std::fmt::Debug {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<AABB>;
}
