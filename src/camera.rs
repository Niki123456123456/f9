use egui::{vec2, Pos2, Rect, Vec2};
use glam::{vec3, vec4, Mat4, Vec3};
use std::f32::consts::FRAC_PI_4;

use crate::core::basics::{Plane, Ray, intersert};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Projection {
    Perspective,
    Orthographics,
}

pub struct Camera {
    pub target: Vec3,
    pub y_angle: f32,
    pub x_angle: f32,
    pub position: Vec3,

    pub projection: Projection,
    pub perspective_distance: f32,
    pub perspective_fovy: f32,
    pub orthographics_distance: f32,
    pub orthographics_fovy: f32,

    pub z_near: f32,
    pub z_far: f32,

    pub viewport: Rect,

    pub projection_matrix: Mat4,
    pub view_matrix: Mat4,
    pub projection_view_matrix: Mat4,

    pub plane: Plane,
    pub ray: Ray,
    pub world_mouse_position: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            target: Vec3::ZERO,
            y_angle: -FRAC_PI_4,
            x_angle: -FRAC_PI_4,
            position: Vec3::ZERO,
            projection: Projection::Perspective,
            perspective_distance: 5.,
            perspective_fovy: 45.,
            orthographics_distance: 100.0,
            orthographics_fovy: 45.0,
            z_near: 0.01,
            z_far: 10000.0,
            viewport: Rect::from_two_pos(Pos2::new(0., 0.), Pos2::new(1., 1.)),
            projection_matrix: Mat4::IDENTITY,
            view_matrix: Mat4::IDENTITY,
            projection_view_matrix: Mat4::IDENTITY,

            plane: Plane {
                position: Vec3::ZERO,
                orientation: Vec3::Y,
            },
            ray: Ray {
                origin: Vec3::ZERO,
                direction: Vec3::Y,
            },
            world_mouse_position: Vec3::ZERO,
        }
    }
}

impl Camera {
    pub fn aspect(&self) -> f32 {
        self.viewport.width() / self.viewport.height()
    }

    pub fn distance(&self) -> f32 {
        match self.projection {
            Projection::Perspective => self.perspective_distance,
            Projection::Orthographics => self.orthographics_distance,
        }
    }
    pub fn fovy(&self) -> f32 {
        match self.projection {
            Projection::Perspective => self.perspective_fovy,
            Projection::Orthographics => self.orthographics_fovy,
        }
    }

    pub fn get_projection_matrix(&mut self) -> Mat4 {
        match self.projection {
            Projection::Perspective => {
                Mat4::perspective_rh_gl(self.fovy(), self.aspect(), self.z_near, self.z_far)
            }
            Projection::Orthographics => {
                let top = self.fovy() / 2.0;
                let right = top * self.aspect();

                Mat4::orthographic_rh_gl(-right, right, -top, top, self.z_near, self.z_far)
            }
        }
    }

    pub fn calculate_matrixs(&mut self) {
        self.position = get_position(self.target, self.distance(), self.x_angle, self.y_angle);
        self.view_matrix = Mat4::look_at_rh(self.position, self.target, vec3(0., 1., 0.));
        self.projection_matrix = self.get_projection_matrix();
        self.projection_view_matrix = self.projection_matrix * self.view_matrix;
    }

    pub fn update_input(&mut self, ctx: &egui::Context) {
        let keys = ctx.input(|i| {
            if i.key_down(egui::Key::A) {
                self.y_angle += 0.01;
            } else if i.key_down(egui::Key::D) {
                self.y_angle -= 0.01;
            }

            if i.key_down(egui::Key::W) {
                self.x_angle += 0.01;
            } else if i.key_down(egui::Key::S) {
                self.x_angle -= 0.01;
            }

            let scroll = i.scroll_delta.y;

            match self.projection {
                Projection::Perspective => {
                    if i.key_down(egui::Key::Q) {
                        self.perspective_distance += self.perspective_distance * 0.01;
                    } else if i.key_down(egui::Key::E) {
                        self.perspective_distance -= self.perspective_distance * 0.01;
                    }
                    self.perspective_distance += scroll * self.perspective_distance * 0.001;
                }
                Projection::Orthographics => {
                    if i.key_down(egui::Key::Q) {
                        self.orthographics_fovy += self.orthographics_fovy * 0.01;
                    } else if i.key_down(egui::Key::E) {
                        self.orthographics_fovy -= self.orthographics_fovy * 0.01;
                    }
                    self.orthographics_fovy += scroll * self.orthographics_fovy * 0.001;
                }
            }
        });
    }

    pub fn update_ray(&mut self, mouse: Vec2) {
        match self.projection {
            Projection::Perspective => {
                // https://antongerdelan.net/opengl/raycasting.html
                let mouse = mouse - vec2(self.viewport.left(), self.viewport.top());
                let x = (2.0 * mouse.x) / self.viewport.width() - 1.0;
                let y = 1.0 - (2.0 * mouse.y) / self.viewport.height();
                let z = 1.0;
                let ray_nds = vec3(x, y, z);

                let ray_clip = vec4(ray_nds.x, ray_nds.y, -1.0, 1.0);

                let mut ray_eye = self.projection_matrix.inverse() * ray_clip;
                ray_eye = vec4(ray_eye.x, ray_eye.y, -1.0, 0.0);

                let ray_wor = self.view_matrix.inverse() * ray_eye;
                let ray_world = vec3(ray_wor.x, ray_wor.y, ray_wor.z).normalize();

                self.ray = Ray {
                    origin: self.position,
                    direction: ray_world,
                };
            }
            Projection::Orthographics => {
                let mouse = mouse - vec2(self.viewport.left(), self.viewport.top());

                let x = (2.0 * mouse.x) / self.viewport.width() - 1.0;
                let y = 1.0 - (2.0 * mouse.y) / self.viewport.height();

                let camera_inverse_matrix =
                    self.view_matrix.inverse() * self.projection_matrix.inverse();
                let near = camera_inverse_matrix * Vec3::new(x, y, 0.0).extend(1.0);
                let far = camera_inverse_matrix * Vec3::new(x, y, 1.0).extend(1.0);

                let near = near.truncate() / near.w;
                let far = far.truncate() / far.w;
                let dir: Vec3 = far - near;
                let origin = vec3(near.x, near.y, near.z);

                self.ray = Ray {
                    origin: origin,
                    direction: dir,
                };
            }
        }
    
        self.world_mouse_position = intersert(&self.plane, &self.ray);
    }
}

fn get_position(target: Vec3, distance: f32, x_angle: f32, y_angle: f32) -> Vec3 {
    let position = target
        + vec3(
            y_angle.cos() * x_angle.sin() * distance,
            x_angle.cos() * distance,
            y_angle.sin() * x_angle.sin() * distance,
        );
    return position;
}
