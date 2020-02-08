use crate::materials::Material;
use crate::vec::{Ray, Vec3};

#[derive(Clone)]
pub struct Hit<'obj> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'obj Material,
}

pub enum Model {
    Sphere(Sphere),
    ModelVec(Vec<Model>),
}

impl Model {
    pub fn hit(&self, ray: &Ray) -> Option<Hit> {
        match self {
            Model::Sphere(sphere) => hit_sphere(sphere, ray),
            Model::ModelVec(models) => hit_vec_of_models(models, ray),
        }
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

/// Minimum distance a ray must travel before we'll consider a possible hit.
///
/// If we try to use 0 here, we get a really strange bug. When a ray hits an object
/// and bounces, we'll sometimes register another hit on the same sphere,
/// at some tiny but positive distance, due to floating-point error.
///
const T_MIN: f32 = 0.0001;

fn hit_sphere<'a>(sphere: &'a Sphere, ray: &Ray) -> Option<Hit<'a>> {
    let oc = ray.origin - sphere.center;
    let a = ray.direction.dot(&ray.direction);
    let hb = oc.dot(&ray.direction);
    let c = oc.dot(&oc) - sphere.radius * sphere.radius;
    let discriminant = hb * hb - a * c;
    if discriminant > 0.0 {
        let t = (-hb - discriminant.sqrt()) / a;
        if t >= T_MIN {
            let p = ray.point_at_parameter(t);
            return Some(Hit {
                t,
                p,
                normal: (p - sphere.center) / sphere.radius,
                material: &sphere.material,
            });
        }
        let t = (-hb + discriminant.sqrt()) / a;
        if t >= T_MIN {
            let p = ray.point_at_parameter(t);
            return Some(Hit {
                t,
                p,
                normal: (p - sphere.center) / sphere.radius,
                material: &sphere.material,
            });
        }
    }
    None
}

fn hit_vec_of_models<'a>(models: &'a [Model], r: &Ray) -> Option<Hit<'a>> {
    let mut best = None;
    for child in models {
        if let Some(hit) = child.hit(r) {
            match best.clone() {
                None => best = Some(hit),
                Some(prev) => {
                    if hit.t < prev.t {
                        best = Some(hit)
                    }
                }
            }
        }
    }
    best
}
