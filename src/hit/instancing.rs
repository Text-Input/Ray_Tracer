use crate::hit::hitable_list::*;
use crate::hit::rectangle::*;
use crate::hit::HitRecord;
use crate::hit::Hitable;
use crate::hit::AABB;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

use std::sync::Arc;

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

///This shouldn't need to be used, but it's here anyway, just in case.
#[derive(Debug)]
pub struct Translate<T: Hitable> {
    obj: T,
    offset: Vec3,
}

impl<T: Hitable> Translate<T> {
    pub fn new(obj: T, offset: Vec3) -> Translate<T> {
        Translate { obj, offset }
    }
}

impl<T: Hitable> Hitable for Translate<T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let moved_r = Ray::new(r.origin() - self.offset, r.direction());

        self.obj.hit(&moved_r, t_min, t_max).map(|rec| {
            HitRecord::new(
                rec.t,
                rec.position + self.offset,
                &moved_r,
                rec.normal,
                rec.material,
            )
        })
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.obj.bounding_box().map(|output_box| {
            AABB::new(
                output_box.min() + self.offset,
                output_box.max() + self.offset,
            )
        })
    }
}

#[derive(Debug)]
pub struct RotateY<T: Hitable> {
    obj: T,
    sin_theta: f32,
    cos_theta: f32,
    aa_box: Option<AABB>,
}

impl<T: Hitable> RotateY<T> {
    pub fn new(obj: T, angle: f32) -> RotateY<T> {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let aa_box = obj.bounding_box().map(|bbox| {
            let mut min = Vec3::new(f32::MAX, f32::MAX, f32::MAX);
            let mut max = Vec3::new(f32::MIN, f32::MIN, f32::MIN);

            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let x = i as f32 * bbox.max().x() + (1 - i) as f32 * bbox.min().x();
                        let y = j as f32 * bbox.max().y() + (1 - j) as f32 * bbox.min().y();
                        let z = k as f32 * bbox.max().z() + (1 - k) as f32 * bbox.min().z();

                        let x = cos_theta * x + sin_theta * z;
                        let z = -sin_theta * x + cos_theta * z;

                        let tester = Vec3::new(x, y, z);

                        for c in 0..3 {
                            min.set(c, min.get(c).min(tester.get(c)));
                            max.set(c, max.get(c).max(tester.get(c)));
                        }
                    }
                }
            }
            AABB::new(min, max)
        });

        RotateY {
            obj,
            sin_theta,
            cos_theta,
            aa_box,
        }
    }
}

impl<T: Hitable> Hitable for RotateY<T> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin.set(
            0,
            self.cos_theta * r.origin().get(0) - self.sin_theta * r.origin().get(2),
        );
        origin.set(
            2,
            self.sin_theta * r.origin().get(0) + self.cos_theta * r.origin().get(2),
        );

        direction.set(
            0,
            self.cos_theta * r.direction().get(0) - self.sin_theta * r.direction().get(2),
        );
        direction.set(
            2,
            self.sin_theta * r.direction().get(0) + self.cos_theta * r.direction().get(2),
        );

        let r_rotated = Ray::new(origin, direction);

        self.obj.hit(&r_rotated, t_min, t_max).map(|rec| {
            let mut position = rec.position;
            let mut normal = rec.normal;

            position.set(
                0,
                self.cos_theta * rec.position.get(0) + self.sin_theta * rec.position.get(2),
            );
            position.set(
                2,
                -self.sin_theta * rec.position.get(0) + self.cos_theta * rec.position.get(2),
            );

            normal.set(
                0,
                self.cos_theta * rec.normal.get(0) + self.sin_theta * rec.normal.get(2),
            );
            normal.set(
                2,
                -self.sin_theta * rec.normal.get(0) + self.cos_theta * rec.normal.get(2),
            );

            HitRecord::new(rec.t, position, &r_rotated, normal, rec.material)
        })
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.aa_box
    }
}

#[derive(Debug)]
pub struct RectangularBox {
    box_min: Vec3,
    box_max: Vec3,
    sides: HitableList,
}

impl RectangularBox {
    pub fn new(p0: Vec3, p1: Vec3, material: Arc<dyn Material>) -> RectangularBox {
        let sides = HitableList::new(vec![
            Box::new(XyRectangle::new(
                p0.x(),
                p1.x(),
                p0.y(),
                p1.y(),
                p1.z(),
                Arc::clone(&material),
            )),
            Box::new(FlipFace::new(XyRectangle::new(
                p0.x(),
                p1.x(),
                p0.y(),
                p1.y(),
                p0.z(),
                Arc::clone(&material),
            ))),
            Box::new(XzRectangle::new(
                p0.x(),
                p1.x(),
                p0.z(),
                p1.z(),
                p1.y(),
                Arc::clone(&material),
            )),
            Box::new(FlipFace::new(XzRectangle::new(
                p0.x(),
                p1.x(),
                p0.z(),
                p1.z(),
                p0.y(),
                Arc::clone(&material),
            ))),
            Box::new(YzRectangle::new(
                p0.y(),
                p1.y(),
                p0.z(),
                p1.z(),
                p1.x(),
                Arc::clone(&material),
            )),
            Box::new(FlipFace::new(YzRectangle::new(
                p0.y(),
                p1.y(),
                p0.z(),
                p1.z(),
                p0.x(),
                Arc::clone(&material),
            ))),
        ]);
        RectangularBox {
            box_min: p0,
            box_max: p1,
            sides,
        }
    }
}

impl Hitable for RectangularBox {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self) -> Option<AABB> {
        self.sides.bounding_box()
    }
}
