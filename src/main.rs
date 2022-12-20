#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

extern crate nalgebra_glm as glm;

mod camera;
mod ray;
mod renderer;
mod scene;

use camera::Camera;
use renderer::Renderer;
use scene::{Scene, Sphere};

use std::time::{Duration, Instant};

use eframe::{egui, epaint::Color32};

fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "RayTracer",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    renderer: Renderer,
    camera: Camera,
    scene: Scene,
    texture: Option<egui::TextureHandle>,
    frame_time: Duration,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut scene = Scene::default();

        let mut sphere = Sphere::default();
        sphere.material.albedo = glm::vec4(0.1, 0.3, 1.0, 0.0);
        sphere.radius = 2.0;
        sphere.position = glm::vec3(0.0, 0.0, -5.0);
        scene.spheres.push(sphere);

        let mut sphere = Sphere::default();
        sphere.radius = 0.5;
        sphere.material.albedo = glm::vec4(1.0, 0.0, 1.0, 0.0);
        scene.spheres.push(sphere);

        Self {
            renderer: Renderer::default(),
            camera: Camera::default(),
            scene: scene,
            texture: None,
            frame_time: Duration::ZERO,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let frame_start = Instant::now();

        egui::SidePanel::right("Info")
            .resizable(true)
            .show(ctx, |ui| {
                ui.label(format!(
                    "FPS: {}",
                    (1.0 / self.frame_time.as_secs_f32()) as usize
                ));
                ui.label(format!(
                    "Frame Time: {}ms",
                    self.frame_time.as_secs_f32() * 1000.0
                ));
                ui.label(format!("Pointer Delta: {:?}", ctx.pointer_latest_pos()));
                ui.label(format!("Camera Position: {:?}", self.camera.position))
            });

        let central_panel_frame = egui::containers::Frame {
            outer_margin: egui::style::Margin {
                left: 0.,
                right: 0.,
                top: 0.,
                bottom: 0.,
            },
            inner_margin: egui::style::Margin {
                left: 0.,
                right: 0.,
                top: 0.,
                bottom: 0.,
            },
            rounding: egui::Rounding {
                nw: 0.,
                ne: 0.,
                sw: 0.,
                se: 0.,
            },
            shadow: eframe::epaint::Shadow::default(),
            fill: Color32::BLACK,
            stroke: egui::Stroke::default(),
        };

        egui::CentralPanel::default()
            .frame(central_panel_frame)
            .show(ctx, |ui| {
                let panel_size = ui.available_size();

                self.camera.on_update(ctx, self.frame_time.as_secs_f32());
                self.camera
                    .on_resize(panel_size.x as usize, panel_size.y as usize);

                let img = self.renderer.render(&self.scene, &self.camera);

                let texture = self.texture.insert(ctx.load_texture(
                    "eventFrame",
                    img,
                    egui::TextureFilter::Nearest,
                ));

                ui.image(texture, panel_size);
            });
        ctx.request_repaint();

        self.frame_time = frame_start.elapsed();
    }
}
