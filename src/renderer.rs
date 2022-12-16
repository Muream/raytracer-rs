use eframe::epaint::{Color32, ColorImage};

use std::cmp::max;

use crate::{camera::Camera, ray::Ray};

pub struct Renderer {}

impl Default for Renderer {
    fn default() -> Self {
        Renderer {}
    }
}

impl Renderer {
    pub fn render(&self, camera: &Camera) -> ColorImage {
        let mut pixels = Vec::new();

        // Creating a texture of 0x0px panics
        let width = max(camera.viewport_width, 1);
        let height = max(camera.viewport_height, 1);

        let ray_origin = camera.position;

        // Iterate
        for y in (0..height).rev() {
            for x in 0..width {
                let ray_direction = camera.ray_directions[x + y * camera.viewport_width];

                let ray = Ray {
                    origin: ray_origin,
                    direction: ray_direction,
                };

                pixels.push(self.trace_ray(&ray))
            }
        }

        ColorImage {
            size: [width, height],
            pixels,
        }
    }
    pub fn trace_ray(&self, ray: &Ray) -> Color32 {
        // (bx^2 + by^2 + bz^2)t^2 + (axbx + ayby + azbz)2t + (ax^2 + ay^2 + az^2 - r^2) = 0
        // where
        // a = ray origin
        // b = ray direction
        // r = sphere radius
        // t = hit distance

        let radius = 0.5;

        let a = glm::dot(&ray.direction, &ray.direction);
        let b = 2.0 * glm::dot(&ray.origin, &ray.direction);
        let c = glm::dot(&ray.origin, &ray.origin) - radius * radius;

        // Quadratic formula discriminant:
        // b^2 - 4ac
        // (-b +- sqrt(discriminant)) / (2.0 * a)
        let discriminant = b * b - 4.0 * a * c;

        // The ray missed the sphere, we return a clear color
        if discriminant < 0.0 {
            return Color32::BLACK;
        }

        // let _t0 = (-b + discriminant.sqrt()) / (2.0 * a);
        let closest_t = (-b - discriminant.sqrt()) / (2.0 * a);

        if closest_t < 0.0 {
            return Color32::BLACK;
        }

        let hit_position = ray.origin + ray.direction * closest_t;

        let normal = (hit_position).normalize();
        let light_dir = glm::vec3(-1.0, -1.0, -1.0).normalize();

        let light = f32::max(glm::dot(&normal, &(-light_dir)), 0.0);

        // let sphere_color = normal * 0.5 + glm::vec3(0.5, 0.5, 0.5);
        let color = glm::vec3(light, light, light);

        return Color32::from_rgb(
            (color.x * 255.0) as u8,
            (color.y * 255.0) as u8,
            (color.z * 255.0) as u8,
        );
    }
}
