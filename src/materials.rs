use crate::model::Hit;
use crate::vec::{random_in_unit_sphere, Ray, Vec3};
use rand::random;

#[derive(Debug)]
pub struct Scatter {
    pub color: Vec3,
    pub ray: Option<Ray>,
}

pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material {
    pub fn scatter(&self, r_in: &Ray, rec: &Hit) -> Scatter {
        match self {
            Material::Lambertian(l) => scatter_lambertian(l, r_in, rec),
            Material::Metal(m) => scatter_metal(m, r_in, rec),
            Material::Dielectric(d) => scatter_dielectric(d, r_in, rec),
        }
    }
}

pub struct Lambertian {
    pub albedo: Vec3,
}

fn scatter_lambertian(lambertian: &Lambertian, _r_in: &Ray, hit: &Hit) -> Scatter {
    let target = hit.p + hit.normal + random_in_unit_sphere();
    Scatter {
        color: lambertian.albedo,
        ray: Some(Ray::new(hit.p, target - hit.p)),
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(&n) * n
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

fn scatter_metal(metal: &Metal, r_in: &Ray, hit: &Hit) -> Scatter {
    let reflected = reflect(r_in.direction, hit.normal);
    let scattered = Ray::new(hit.p, reflected + metal.fuzz * random_in_unit_sphere());

    Scatter {
        color: metal.albedo,
        ray: if scattered.direction.dot(&hit.normal) <= 0.0 {
            None
        } else {
            Some(scattered)
        },
    }
}

pub struct Dielectric {
    // Technically, this is not the index of refaction but the ratio of the
    // index of refraction inside the material to the index of refraction
    // outside.  But if the material outside is air, its index of refraction is
    // 1 and so it amounts to the same thing.
    pub index: f32,
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.to_unit_vector();

    let dt = uv.dot(&n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - dt * n) - discriminant.sqrt() * n)
    } else {
        None
    }
}

/// Christophe Schlick's approximation for the reflectivity of glass,
/// as a function of the angle of incidence and index of refraction.
fn schlick(cosine: f32, index: f32) -> f32 {
    let r0 = (1.0 - index) / (1.0 + index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

const WHITE: Vec3 = Vec3(1.0, 1.0, 1.0);

fn scatter_dielectric(dielectric: &Dielectric, r_in: &Ray, hit: &Hit) -> Scatter {
    let outward_normal: Vec3;

    let (ni_over_nt, cosine) = if r_in.direction.dot(&hit.normal) > 0.0 {
        outward_normal = -&hit.normal;
        (
            dielectric.index,
            dielectric.index * r_in.direction.dot(&hit.normal) / r_in.direction.length(),
        )
    } else {
        outward_normal = hit.normal;
        (
            1.0 / dielectric.index,
            -r_in.direction.dot(&hit.normal) / r_in.direction.length(),
        )
    };

    if let Some(refracted) = refract(r_in.direction, outward_normal, ni_over_nt) {
        if random::<f32>() > schlick(cosine, dielectric.index) {
            return Scatter {
                color: WHITE,
                ray: Some(Ray::new(hit.p, refracted)),
            };
        }
    }

    Scatter {
        color: WHITE,
        ray: Some(Ray::new(hit.p, reflect(r_in.direction, hit.normal))),
    }
}
