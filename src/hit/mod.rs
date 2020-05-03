
pub mod sphere;
pub mod triangle;
pub mod hitable_list;
pub mod bvh;

use super::vec3::*;
use super::ray::*;
use super::aabb::*;
use super::material::*;


//data about a ray hitting something.
#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
	pub t: f32,
	pub position: Vec3,
	pub normal: Vec3,
	pub material: &'a Box<dyn Material>,
}

//implemented by objects in the scene, so they can be hit by the rays.
pub trait Hitable: Sync + Send + std::fmt::Debug{
	fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
	fn bounding_box(&self) -> Option<AABB>;
}