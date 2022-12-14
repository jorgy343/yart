use crate::geometries::{
    bound_by_box::BoundByBox, bounding_box::BoundingBox, bounding_volume::BoundingVolume, intersectable::Intersectable,
    intersection::Intersection, ray::Ray,
};

#[derive(Debug)]
pub struct BoundingGeometry {
    bounding_volume: Box<dyn BoundingVolume>,
    child: Box<dyn Intersectable>,
}

impl BoundingGeometry {
    pub fn new(bounding_volume: Box<dyn BoundingVolume>, child: Box<dyn Intersectable>) -> Self {
        Self { bounding_volume, child }
    }
}

impl Intersectable for BoundingGeometry {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        if self.bounding_volume.ray_intersects(ray) {
            self.child.intersect(ray)
        } else {
            None
        }
    }
}

impl BoundByBox for BoundingGeometry {
    fn calculate_bounding_box(&self) -> BoundingBox {
        self.bounding_volume.calculate_bounding_box()
    }
}
