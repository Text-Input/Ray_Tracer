use crate::aabb::*;
use crate::ray::*;

use super::*;

use std::cmp::Ordering;

#[derive(Debug)]
pub struct BvhNode {
    left: Option<Box<dyn Hitable>>,
    right: Option<Box<dyn Hitable>>,
    aa_box: AABB,
}

impl BvhNode {
    pub fn new(mut objects: Vec<Box<dyn Hitable>>) -> BvhNode {
        // find the enclosing bounding box.
        let mut main_box = objects[0].bounding_box();
        //we account for the first box in main_box, so we can skip in when iterating though the list.
        for obj in objects.iter().skip(1) {
            main_box = match main_box {
                Some(temp) => match obj.bounding_box() {
                    Some(aa_box) => Some(AABB::surrounding_box(temp, aa_box)),
                    None => Some(temp),
                },
                None => match obj.bounding_box() {
                    Some(aa_box) => Some(aa_box),
                    None => None,
                },
            }
        }
        let axis = main_box.unwrap().longest_axis();

        let comparator = match axis {
            0 => compare_box_by_x_axis,
            1 => compare_box_by_y_axis,
            2 => compare_box_by_z_axis,
            _ => {
                panic!("Got a number out of range! Recieved: {}", axis);
            }
        };

        objects.sort_by(|left, right| comparator(left, right));

        let (left, right) = match objects.len() {
            0 => (None, None),
            1 => (Some(objects.remove(0)), None),
            2 => (Some(objects.remove(0)), Some(objects.remove(0))),
            _ => {
                let mid = objects.len() / 2;

                let left_objs: Vec<_> = objects.drain(0..mid).collect();
                let right_objs = objects;

                let left: Box<dyn Hitable> = Box::new(BvhNode::new(left_objs));
                let right: Box<dyn Hitable> = Box::new(BvhNode::new(right_objs));
                (Some(left), Some(right))
            }
        };

        BvhNode {
            left,
            right,
            aa_box: main_box.unwrap(),
        }
    }

    pub fn new_sah(mut l: Vec<Box<dyn Hitable>>) -> BvhNode {
        let n = l.len();

        let mut left_area = Vec::with_capacity(n);
        let mut right_area = Vec::with_capacity(n);

        // find the enclosing bounding box.
        let mut main_box = l[0].bounding_box();
        //we account for the first box in main_box, so we can skip in when iterating though the list.
        for obj in l.iter().skip(1) {
            main_box = match main_box {
                Some(temp) => match obj.bounding_box() {
                    Some(aa_box) => Some(AABB::surrounding_box(temp, aa_box)),
                    None => Some(temp),
                },
                None => match obj.bounding_box() {
                    Some(aa_box) => Some(aa_box),
                    None => None,
                },
            }
        }

        //check to make sure we have enough items in the list.
        match l.len() {
            0 => {
                return BvhNode {
                    right: None,
                    left: None,
                    aa_box: main_box.unwrap(),
                }
            }
            1 => {
                return BvhNode {
                    right: Some(l.remove(0)),
                    left: None,
                    aa_box: main_box.unwrap(),
                }
            }
            2 => {
                return BvhNode {
                    right: Some(l.remove(0)),
                    left: Some(l.remove(0)),
                    aa_box: main_box.unwrap(),
                }
            }
            _ => {}
        }

        assert!(l.len() > 2, "Something went wrong with the match!");

        let axis = main_box
            .expect("can't have objects with no bounding box!")
            .longest_axis();

        let comparator = match axis {
            0 => compare_box_by_x_axis,
            1 => compare_box_by_y_axis,
            2 => compare_box_by_z_axis,
            _ => {
                panic!("Got a number out of range! Recieved: {}", axis);
            }
        };

        l.sort_by(|left, right| comparator(left, right));
        /*
        for i in 0..n {
            boxes.push(l[i].bounding_box());
        } */
        let boxes: Vec<_> = l
            .iter()
            .map(|i| {
                i.bounding_box()
                    .expect("items with no bounding box are not allowed!")
            })
            .collect();

        assert!(boxes.len() == l.len(), "Boxes is shorter than the input!");

        left_area.push(boxes[0].area());
        let mut left_box = boxes[0];

        for i in 1..n - 1 {
            left_box = AABB::surrounding_box(left_box, boxes[i]);
            left_area.push(left_box.area());
        }
        assert!(
            left_area.len() == n - 1,
            "left area length was {}, but should have been {}",
            left_area.len(),
            n - 1
        );

        right_area.push(boxes[n - 1].area());
        let mut right_box = boxes[n - 1];
        for i in (1..n - 1).rev() {
            right_box = AABB::surrounding_box(right_box, boxes[i]);
            right_area.insert(0, right_box.area());
        }
        assert!(
            left_area.len() == n - 1,
            "right area length was {}, but should have been {}",
            right_area.len(),
            n - 1
        );

        let mut min_sah = f32::MAX;
        let mut min_sah_idx = 0;
        for i in 0..(n - 1) {
            let sah = (i as f32) * left_area[i] + ((n - i - 1) as f32) * right_area[i];
            if sah < min_sah {
                min_sah_idx = i;
                min_sah = sah;
            }
        }
        let left: Box<dyn Hitable>;
        let right: Box<dyn Hitable>;

        if min_sah_idx == 0 {
            //left = l[0];
            left = l.remove(0);
        } else {
            left = Box::new(BvhNode::new_sah(l.drain(0..min_sah_idx + 1).collect()));
        }

        if min_sah_idx == n - 2 {
            //right = l[min_sah_idx + 1 - left.len()];
            right = l.pop().unwrap(); // removes the last element.
        } else {
            right = Box::new(BvhNode::new_sah(l));
        }

        let left = Some(left);
        let right = Some(right);

        BvhNode {
            left,
            right,
            aa_box: main_box.unwrap(),
        }
    }
}

impl Hitable for BvhNode {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !self.aa_box.hit(r, t_min, t_max) {
            return None;
        }

        let hit_left = match &self.left {
            Some(obj) => obj.hit(r, t_min, t_max),
            None => None,
        };
        let hit_right = match &self.right {
            Some(obj) => obj.hit(r, t_min, t_max),
            None => None,
        };

        match (hit_left, hit_right) {
            (None, None) => None,
            (None, Some(h)) => Some(h),
            (Some(h), None) => Some(h),
            (Some(left), Some(right)) => {
                if left.t < right.t {
                    Some(left)
                } else {
                    Some(right)
                }
            }
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(self.aa_box)
    }
}

fn get_boxes(left: &Box<dyn Hitable>, right: &Box<dyn Hitable>) -> (AABB, AABB) {
    let l_box = left.bounding_box();
    let r_box = right.bounding_box();

    let l_box = match l_box {
        Some(bounds) => bounds,
        None => panic!("Encountered an object with no bounding box!"),
    };

    let r_box = match r_box {
        Some(bounds) => bounds,
        None => panic!("Encountered an object with no bounding box!"),
    };
    (l_box, r_box)
}

fn compare_box_by_x_axis(left: &Box<dyn Hitable>, right: &Box<dyn Hitable>) -> Ordering {
    let (l_box, r_box) = get_boxes(left, right);

    l_box.min().x().partial_cmp(&r_box.min().x()).unwrap()
}

fn compare_box_by_y_axis(left: &Box<dyn Hitable>, right: &Box<dyn Hitable>) -> Ordering {
    let (l_box, r_box) = get_boxes(left, right);

    l_box.min().z().partial_cmp(&r_box.min().z()).unwrap()
}

fn compare_box_by_z_axis(left: &Box<dyn Hitable>, right: &Box<dyn Hitable>) -> Ordering {
    let (l_box, r_box) = get_boxes(left, right);

    l_box.min().z().partial_cmp(&r_box.min().z()).unwrap()
}
