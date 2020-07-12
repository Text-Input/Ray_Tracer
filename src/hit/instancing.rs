use crate::hit::HitRecord;
use crate::hit::Hitable;
use crate::hit::AABB;
use crate::Ray;

#[derive(Debug)]
pub struct FlipFace<T: Hitable> {
    obj: T,
}

impl<T: Hitable> FlipFace<T> {
    pub fn new(obj: T) -> FlipFace<T> {
        FlipFace { obj }
    }
}

impl<T: Hitable> Hitable for FlipFace<T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.obj.hit(r, t_min, t_max).map(|mut rec| {
            rec.front_face = !rec.front_face;
            rec
        })
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.obj.bounding_box()
    }
}
