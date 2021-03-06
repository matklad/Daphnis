use rand::Rng;

use crate::materials::Scatter;
use crate::objects::HitRecord;
use crate::ray::Ray;
use crate::vector::Vector;

pub fn scatter(ref_idx: f64, r_in: &Ray, hit: &HitRecord) -> Scatter {
    let reflected = reflect(&r_in.direction, &hit.normal);
    let attenuation = Vector::new(1.0, 1.0, 1.0);

    let outward_normal;
    let ni_over_nt;
    let cosine;

    if Vector::dot(&r_in.direction, &hit.normal) > 0.0 {
        outward_normal = -hit.normal;
        ni_over_nt = ref_idx;
        cosine = ref_idx * Vector::dot(&r_in.direction, &hit.normal) / r_in.direction.length();
    } else {
        outward_normal = hit.normal;
        ni_over_nt = 1.0 / ref_idx;
        cosine = Vector::dot(&-r_in.direction, &hit.normal) / r_in.direction.length();
    };

    let mut rng = rand::thread_rng();
    let prob = rng.gen_range(0.0, 1.0);
    let reflect_prob = schlick(cosine, ref_idx);

    if prob < reflect_prob {
        Scatter::new(attenuation, Ray::new(hit.p, reflected))
    } else {
        let refraction = refract(&r_in.direction, &outward_normal, ni_over_nt);
        match refraction {
            Some(refracted) => Scatter::new(attenuation, Ray::new(hit.p, refracted)),
            None => Scatter::new(attenuation, Ray::new(hit.p, reflected)),
        }
    }
}

fn reflect(v: &Vector, n: &Vector) -> Vector {
    *v - 2.0 * Vector::dot(&v, n) * *n
}

fn refract(v: &Vector, n: &Vector, ni_over_nt: f64) -> Option<Vector> {
    let uv = v.unit();
    let dt = Vector::dot(&uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - *n * dt) - *n * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
