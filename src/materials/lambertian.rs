use crate::geometry::{random_in_unit_sphere, Vec3};
use crate::materials::{Material, Scatter};
use crate::objects::HitRecord;
use crate::ray::Ray;

#[derive(Debug)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, hit: &HitRecord) -> Scatter {
        let target = hit.p + hit.normal + random_in_unit_sphere();
        let scattered = Ray::new(hit.p, target - hit.p);
        Scatter::new(self.albedo, scattered)
    }
}
