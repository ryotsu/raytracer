use super::{Aabb, FlipFace, HitRecord, HittableList, Object, XYRect, XZRect, YZRect};
use crate::core::{Point, Ray};
use crate::materials::Material;

use std::ops::Range;
use std::sync::Arc;

use rand::prelude::*;

pub struct Boxx {
    min: Point,
    max: Point,
    sides: HittableList,
}

impl Boxx {
    pub fn new(min: Point, max: Point, material: Material) -> Self {
        let mut sides = HittableList::new();

        sides.add(Arc::new(XYRect::new(
            min.x(),
            max.x(),
            min.y(),
            max.y(),
            max.z(),
            material.clone(),
        )));
        sides.add(Arc::new(FlipFace::new(XYRect::new(
            min.x(),
            max.x(),
            min.y(),
            max.y(),
            min.z(),
            material.clone(),
        ))));

        sides.add(Arc::new(XZRect::new(
            min.x(),
            max.x(),
            min.z(),
            max.z(),
            max.y(),
            material.clone(),
        )));
        sides.add(Arc::new(FlipFace::new(XZRect::new(
            min.x(),
            max.x(),
            min.z(),
            max.z(),
            min.y(),
            material.clone(),
        ))));

        sides.add(Arc::new(YZRect::new(
            min.y(),
            max.y(),
            min.z(),
            max.z(),
            max.x(),
            material.clone(),
        )));
        sides.add(Arc::new(FlipFace::new(YZRect::new(
            min.y(),
            max.y(),
            min.z(),
            max.z(),
            min.x(),
            material.clone(),
        ))));

        Self { min, max, sides }
    }
}

impl Object for Boxx {
    fn hit(&self, ray: &Ray, t_range: Range<f64>, rng: &mut ThreadRng) -> Option<HitRecord> {
        self.sides.hit(ray, t_range, rng)
    }

    fn bounding_box(&self, _t_range: Range<f64>) -> Aabb {
        Aabb::new(self.min, self.max)
    }
}
