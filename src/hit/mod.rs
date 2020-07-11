pub mod bvh;
pub mod hitable_list;
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
    pub material: &'a dyn Material,
}

//implemented by objects in the scene, so they can be hit by the rays.
pub trait Hitable: Sync + Send + std::fmt::Debug {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self) -> Option<AABB>;
}
