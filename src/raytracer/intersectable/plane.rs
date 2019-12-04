use crate::raytracer::EPSILON;
use crate::raytracer::material::Material;
use crate::raytracer::ray::Ray;
use crate::raytracer::vector::Vec3;

use super::Intersectable;

#[derive(Debug)]
pub struct Plane {
    pub position: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl Intersectable for Plane {
    fn intersect(&self, ray: Ray) -> Option<f64> {
        let denom = self.normal.dot(ray.direction);

        if denom.abs() > EPSILON {
            let v = self.position - ray.origin;

            let distance = v.dot(self.normal) / denom;

            if distance >= 0.0 {
                Some(distance)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn material(&self) -> Material {
        self.material
    }

    fn normal(&self, _point: Vec3) -> Vec3 {
        -self.normal
    }
}
