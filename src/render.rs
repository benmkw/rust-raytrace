use lodepng::RGB;
use rand::{thread_rng, Rng};

use crate::camera::Camera;
use crate::model::Model;
use crate::vec::{Ray, Vec3};
use std::f32::consts::PI;

fn color(mut ray: Ray, model: &Model) -> Vec3 {
    const WHITE: Vec3 = Vec3(1.0, 1.0, 1.0);
    let sky_blue = 0.3 * Vec3(0.5, 0.7, 1.0) + 0.7 * WHITE;

    let mut attenuation = WHITE;
    let mut depth = 0;
    while let Some(hit) = model.hit(&ray) {
        let scattered = hit.material.scatter(&ray, &hit);
        attenuation = attenuation * scattered.color;
        if let Some(bounce) = scattered.ray {
            ray = bounce;
        } else {
            break;
        }

        depth += 1;
        if depth >= 50 {
            break;
        }
    }
    let sun_direction = Vec3(1.0, 1.0, 1.0).to_unit_vector();
    let unit_direction = ray.direction.to_unit_vector();
    if sun_direction.dot(&unit_direction) >= (5.0 * PI / 180.0).cos() {
        Vec3(5.0, 5.0, 3.0) * attenuation // SUPER BRIGHT
    } else {
        let t = 0.5 * (unit_direction.y() + 1.0);
        let orig_color = (1.0 - t) * WHITE + t * sky_blue;
        orig_color * attenuation
    }
}

pub fn render(
    scene: &Model,
    camera: &Camera,
    width: usize,
    height: usize,
    samples: usize,
) -> Vec<RGB<u8>> {
    let mut pixels: Vec<RGB<u8>> = Vec::with_capacity(width * height);
    let mut rng = thread_rng();

    for y in 0..height {
        let j = height - 1 - y;
        for i in 0..width {
            let mut col = Vec3(0.0, 0.0, 0.0);
            for _ in 0..samples {
                let u = (i as f32 + rng.gen::<f32>()) / width as f32;
                let v = (j as f32 + rng.gen::<f32>()) / height as f32;

                let r = camera.get_ray(u, v);
                col = col + color(r, scene);
            }
            col = col / samples as f32;
            col = Vec3(
                // sqrt to apply gamma correction
                col.x().sqrt(),
                col.y().sqrt(),
                col.z().sqrt(),
            );
            pixels.push(col.to_rgb());
        }
    }
    pixels
}
