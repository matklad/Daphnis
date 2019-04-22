use std::fmt::Debug;

use crate::geometry::Vec3;
use crate::objects::HitRecord;
use crate::ray::Ray;

mod dielectric;
mod lambertian;
mod metal;

pub use crate::materials::dielectric::Dielectric;
pub use crate::materials::lambertian::Lambertian;
pub use crate::materials::metal::Metal;

#[derive(Copy, Clone, Debug)]
pub struct Scatter {
    pub attenuation: Vec3,
    pub ray: Ray,
}

impl Scatter {
    pub fn new(attenuation: Vec3, ray: Ray) -> Scatter {
        Scatter { attenuation, ray }
    }
}

pub trait Material: Debug {
    fn scatter(&self, r_in: &Ray, hit: &HitRecord) -> Scatter;
}
