use eframe::epaint::{Color32, ColorImage};

use std::cmp::max;

use crate::{camera::Camera, ray::Ray, scene::Scene};

pub struct Renderer {}

impl Default for Renderer {
    fn default() -> Self {
        Renderer {}
    }
}

impl Renderer {
    pub fn render(&self, scene: &Scene, camera: &Camera) -> ColorImage {
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

                pixels.push(self.trace_ray(&scene, &ray))
            }
        }

        ColorImage {
            size: [width, height],
            pixels,
        }
    }
    pub fn trace_ray(&self, scene: &Scene, ray: &Ray) -> Color32 {
        let mut closest_sphere = None;
        let mut closest_t = f32::INFINITY;
        let color: glm::Vec4;

        for sphere in &scene.spheres {
            // (bx^2 + by^2 + bz^2)t^2 + (axbx + ayby + azbz)2t + (ax^2 + ay^2 + az^2 - r^2) = 0
            // where
            // a = ray origin
            // b = ray direction
            // r = sphere radius
            // t = hit distance

            let origin = ray.origin - sphere.position;

            let a = glm::dot(&ray.direction, &ray.direction);
            let b = 2.0 * glm::dot(&origin, &ray.direction);
            let c = glm::dot(&origin, &origin) - sphere.radius * sphere.radius;

            // Quadratic formula discriminant:
            // b^2 - 4ac
            // (-b +- sqrt(discriminant)) / (2.0 * a)
            let discriminant = b * b - 4.0 * a * c;

            // The ray missed the sphere
            if discriminant < 0.0 {
                continue;
            }

            // let _t0 = (-b + discriminant.sqrt()) / (2.0 * a);
            let t = (-b - discriminant.sqrt()) / (2.0 * a);

            // The Sphere is behind the camera
            if t < 0.0 {
                continue;
            }

            if t < closest_t {
                closest_t = t;
                closest_sphere = Some(sphere);
            }
        }

        match closest_sphere {
            Some(sphere) => {
                let hit_position = ray.origin - sphere.position + ray.direction * closest_t;

                let normal = (hit_position).normalize();
                let light_dir = glm::vec3(-1.0, -1.0, -1.0).normalize();

                let light = f32::max(glm::dot(&normal, &(-light_dir)), 0.0);

                // let sphere_color = normal * 0.5 + glm::vec3(0.5, 0.5, 0.5);
                color = sphere.material.albedo * light;
            }
            None => {
                color = glm::Vec4::zeros();
            }
        }

        return Color32::from_rgb(
            (color.x * 255.0) as u8,
            (color.y * 255.0) as u8,
            (color.z * 255.0) as u8,
        );
    }
}
