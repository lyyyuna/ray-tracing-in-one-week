mod sphere;

use crate::types::{Point3, Ray, Vec3};

pub use sphere::Sphere;

pub trait Geometry {
    fn check_ray_hits(&self, r: &Ray) -> bool;
}
