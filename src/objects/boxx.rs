use super::{Aabb, FlipFace, HitRecord, Hittable, HittableList, XYRect, XZRect, YZRect};
use crate::core::{Point, Ray};
use crate::materials::Material;

use std::sync::Arc;

pub struct Boxx {
    min: Point,
    max: Point,
    sides: HittableList,
}

impl Boxx {
    pub fn new(min: Point, max: Point, material: Box<dyn Material>) -> Self {
        let mut sides = HittableList::new();

        sides.add(Arc::new(XYRect::new(
            min.x(),
            max.x(),
            min.y(),
            max.y(),
            max.z(),
            material.box_clone(),
        )));
        sides.add(Arc::new(FlipFace::new(Arc::new(XYRect::new(
            min.x(),
            max.x(),
            min.y(),
            max.y(),
            min.z(),
            material.box_clone(),
        )))));

        sides.add(Arc::new(XZRect::new(
            min.x(),
            max.x(),
            min.z(),
            max.z(),
            max.y(),
            material.box_clone(),
        )));
        sides.add(Arc::new(FlipFace::new(Arc::new(XZRect::new(
            min.x(),
            max.x(),
            min.z(),
            max.z(),
            min.y(),
            material.box_clone(),
        )))));

        sides.add(Arc::new(YZRect::new(
            min.y(),
            max.y(),
            min.z(),
            max.z(),
            max.x(),
            material.box_clone(),
        )));
        sides.add(Arc::new(FlipFace::new(Arc::new(YZRect::new(
            min.y(),
            max.y(),
            min.z(),
            max.z(),
            min.x(),
            material.box_clone(),
        )))));

        Self { min, max, sides }
    }
}

impl Hittable for Boxx {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        self.sides.hit(ray, t_min, t_max, rec)
    }

    fn bounding_box(&self, _t_min: f64, _t_max: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb::new(self.min, self.max);
        true
    }
}
