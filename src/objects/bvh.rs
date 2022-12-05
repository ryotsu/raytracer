use super::{Aabb, HitRecord, Object};
use crate::core::Ray;

use std::cmp::Ordering;
use std::ops::Range;

use rand::prelude::*;

pub struct Bvh {
    contents: BvhContents,
    bounds: Aabb,
}

enum BvhContents {
    Node { left: Box<Bvh>, right: Box<Bvh> },
    Leaf(Box<dyn Object>),
}

impl Bvh {
    pub fn new(
        mut objects: Vec<Box<dyn Object>>,
        t_range: Range<f64>,
        rng: &mut ThreadRng,
    ) -> Self {
        use BvhContents::*;

        let axis = rng.gen_range(0, 3);
        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            _ => box_z_compare,
        };

        let object_span = objects.len();

        match object_span {
            1 => Bvh {
                bounds: objects[0].bounding_box(t_range),
                contents: Leaf(objects.pop().unwrap()),
            },
            _ => {
                objects.sort_by(comparator);
                let right = Self::new(
                    objects.drain(object_span / 2..).collect(),
                    t_range.clone(),
                    rng,
                );
                let left = Self::new(objects, t_range.clone(), rng);
                let bounds = Aabb::surrounding_box(
                    &left.bounding_box(t_range.clone()),
                    &right.bounding_box(t_range),
                );

                Bvh {
                    bounds,
                    contents: Node {
                        left: Box::new(left),
                        right: Box::new(right),
                    },
                }
            }
        }
    }
}

impl Object for Bvh {
    fn hit(&self, ray: &Ray, mut t_range: Range<f64>, rng: &mut ThreadRng) -> Option<HitRecord> {
        if self.bounds.hit(ray, t_range.start, t_range.end) {
            match &self.contents {
                BvhContents::Node { left, right } => {
                    let hit_left = left.hit(ray, t_range.clone(), rng);

                    if let Some(ref hl) = hit_left {
                        t_range.end = hl.t;
                    }

                    let hit_right = right.hit(ray, t_range, rng);

                    match (hit_left, hit_right) {
                        (h, None) | (None, h) => h,
                        (Some(hl), Some(hr)) => {
                            if hl.t < hr.t {
                                Some(hl)
                            } else {
                                Some(hr)
                            }
                        }
                    }
                }
                BvhContents::Leaf(obj) => obj.hit(ray, t_range, rng),
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, _t_range: Range<f64>) -> Aabb {
        self.bounds.clone()
    }
}

fn box_compare(a: &Box<dyn Object>, b: &Box<dyn Object>, axis: usize) -> Ordering {
    let box_a = a.bounding_box(0.0..0.0);
    let box_b = b.bounding_box(0.0..0.0);

    box_a.min[axis].partial_cmp(&box_b.min[axis]).unwrap()
}

fn box_x_compare(a: &Box<dyn Object>, b: &Box<dyn Object>) -> Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Box<dyn Object>, b: &Box<dyn Object>) -> Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Box<dyn Object>, b: &Box<dyn Object>) -> Ordering {
    box_compare(a, b, 2)
}
