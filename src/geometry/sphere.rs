use crate::types::Point3;
use crate::types::Vec3;
use crate::types::Ray;

use super::Geometry;

pub struct Sphere {
    center: Point3,
    radius: f64,
    radius_squared: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { 
            center, radius, radius_squared: radius * radius
        }
    }
}

impl Geometry for Sphere {
    // Ray(t) = O + tD
    // Sphere surface = (X - C)^2 = r^2
    // (O + tD - C)^2 = r^2
    // let O - C = L
    // (tD + L)^2 = r^2
    // D^2 t^2 + 2DLt + L^2- r^2 = 0
    // a = D^2, b = 2(DL), c = L^2 - r^2
    // Delta = b^2 - 4ac = 4(DL)^2 - 4 D^2 (L^2 - r2)
    // So, check (DL)^2 - D^2(L^2 - r^2)
    // root is
    fn hit_by_ray(&self, r: &Ray) -> Option<Point3> {
        let l = &r.origin - &self.center;
        let dl = r.direction.dot(&l);
        let dl2 = dl.powi(2);
        let d2 = r.direction.length_squared();
        let l2 = l.length_squared();
        let delta = dl2 - d2 * (l2 - self.radius_squared);
        
        if delta > 0.0 {
            Some(r.at((-dl - delta.sqrt()) / d2))
        } else {
            None
        }
    }

    fn normal(&self, p: &Point3) -> Vec3 {
        (p - &self.center).unit()
    }
}