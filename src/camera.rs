use eframe::egui;

pub struct Camera {
    pub mat_projection: glm::Mat4,
    pub mat_view: glm::Mat4,
    pub mat_inverse_projection: glm::Mat4,
    pub mat_inverse_view: glm::Mat4,

    pub vertical_fov: f32,
    pub near_clip: f32,
    pub far_clip: f32,
    pub viewport_width: usize,
    pub viewport_height: usize,

    pub position: glm::Vec3,
    pub forward_direction: glm::Vec3,

    pub ray_directions: Vec<glm::Vec3>,

    last_mouse_position: glm::Vec2,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            mat_projection: glm::Mat4::identity(),
            mat_view: glm::Mat4::identity(),
            mat_inverse_projection: glm::Mat4::identity(),
            mat_inverse_view: glm::Mat4::identity(),

            vertical_fov: 45.0,
            near_clip: 0.1,
            far_clip: 100.0,
            viewport_width: 100,
            viewport_height: 100,

            position: glm::vec3(0.0, 0.0, 6.0),
            forward_direction: glm::vec3(0.0, 0.0, -1.0),

            ray_directions: Vec::new(),
            last_mouse_position: glm::Vec2::zeros(),
        }
    }
}

impl Camera {
    pub fn on_update(&mut self, ctx: &egui::Context, delta_time: f32) {
        let mouse_position = match ctx.pointer_latest_pos() {
            Some(value) => value,
            None => return,
        };
        let mouse_position = glm::vec2(mouse_position.x, mouse_position.y);

        let mouse_delta = (mouse_position - self.last_mouse_position) * 0.002;
        self.last_mouse_position = mouse_position;

        if !ctx
            .input()
            .pointer
            .button_down(egui::PointerButton::Secondary)
        {
            return;
        }

        ctx.output().cursor_icon = egui::output::CursorIcon::None;

        let mut moved = false;

        let up_direction = glm::vec3(0.0, 1.0, 0.0);
        let right_direction = glm::cross(&self.forward_direction, &up_direction);

        let speed = 5.0;

        // Compute position
        if ctx.input().key_down(egui::Key::W) {
            // Move forward
            self.position += self.forward_direction * speed * delta_time;
            moved = true;
        } else if ctx.input().key_down(egui::Key::S) {
            // Move Backwards
            self.position -= self.forward_direction * speed * delta_time;
            moved = true;
        } else if ctx.input().key_down(egui::Key::A) {
            // Move Left
            self.position -= right_direction * speed * delta_time;
            moved = true;
        } else if ctx.input().key_down(egui::Key::D) {
            // Move Right
            self.position += right_direction * speed * delta_time;
            moved = true;
        } else if ctx.input().key_down(egui::Key::Q) {
            // Move Down
            self.position -= up_direction * speed * delta_time;
            moved = true;
        } else if ctx.input().key_down(egui::Key::E) {
            // Move Up
            self.position += up_direction * speed * delta_time;
            moved = true;
        }

        let rotation_speed = 1.0;

        if mouse_delta != glm::Vec2::zeros() {
            let pitch_delta = mouse_delta.y * rotation_speed;
            let yaw_delta = mouse_delta.x * rotation_speed;

            let pitch_quat = glm::quat_angle_axis(-pitch_delta, &right_direction);
            let yaw_quat = glm::quat_angle_axis(-yaw_delta, &up_direction);
            let rotation_delta = glm::quat_cross(&pitch_quat, &yaw_quat).normalize();

            self.forward_direction =
                glm::quat_rotate_vec3(&rotation_delta, &self.forward_direction);

            moved = true;
        }

        if moved {
            self.recalculate_view();
            self.recalculate_projection();
            self.recalculate_ray_directions();
        }
    }
    pub fn on_resize(&mut self, width: usize, height: usize) {
        if self.viewport_width == width && self.viewport_height == height {
            return;
        }

        self.viewport_width = width;
        self.viewport_height = width;

        self.recalculate_view();
        self.recalculate_projection();
        self.recalculate_ray_directions();
    }

    fn recalculate_projection(&mut self) {
        self.mat_projection = glm::perspective_fov(
            self.vertical_fov * glm::pi::<f32>() / 180.0,
            self.viewport_width as f32,
            self.viewport_height as f32,
            self.near_clip,
            self.far_clip,
        );
        self.mat_inverse_projection = self.mat_projection.try_inverse().unwrap();
    }

    fn recalculate_view(&mut self) {
        self.mat_view = glm::look_at(
            &self.position,
            &(self.position + self.forward_direction),
            &glm::vec3(0.0, 1.0, 0.0),
        );
        self.mat_inverse_view = self.mat_view.try_inverse().unwrap();
    }

    fn recalculate_ray_directions(&mut self) {
        self.ray_directions = vec![];
        for y in 0..self.viewport_height {
            for x in 0..self.viewport_width {
                // Create coordinates within a 0..1 range
                let mut coordinates = glm::vec2(
                    x as f32 / self.viewport_width as f32,
                    y as f32 / self.viewport_height as f32,
                );

                // remap coordinate to -1..1 range
                coordinates.x = coordinates.x * 2.0 - 1.0;
                coordinates.y = coordinates.y * 2.0 - 1.0;

                let target =
                    self.mat_inverse_projection * glm::vec4(coordinates.x, coordinates.y, 1.0, 1.0);

                let asdf = self.mat_inverse_view
                    * glm::vec3_to_vec4(&(glm::vec4_to_vec3(&target) / target.w).normalize());

                let ray_direction = glm::vec4_to_vec3(&asdf);

                self.ray_directions.push(ray_direction);
            }
        }
    }
}
