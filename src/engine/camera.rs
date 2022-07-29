#![allow(dead_code)]

use std::ops::Add;

use glam::{Mat4, Vec2, Vec3};

pub struct Camera {
    proj_matrix: Mat4,
    view_matrix: Mat4,

    inverse_proj: Mat4,
    inverse_view: Mat4,

    view_width: f32,
    view_height: f32,

    pub position: Vec2
}

impl Camera {
    pub fn new(x: f32, y: f32, view_width: f32, view_height: f32) -> Self {
        let proj_matrix = Mat4::orthographic_rh(0.0, view_width, 0.0, view_height, 0.0, 100.0);
        //proj_matrix.inverse();

        Self {
            proj_matrix,
            view_matrix: Mat4:: ZERO,

            inverse_proj: Mat4::ZERO,
            inverse_view: Mat4::ZERO,

            view_width,
            view_height,

            position: Vec2::new(x, y)
        }
    }

    pub fn adjust_projection(&mut self, view_width: f32, view_height: f32) {
        self.view_width = view_width;
        self.view_height = view_height;
        self.proj_matrix = Mat4::orthographic_rh(0.0, view_width, 0.0, view_height, 0.0, 100.0);
    }

    pub fn get_view_matrix(&mut self) -> Mat4 {
        let cam_front = Vec3::new(0.0, 0.0, -1.0);
        let cam_up = Vec3::new(0.0, 1.0, 0.0);

        self.view_matrix = Mat4::look_at_rh(
            Vec3::new(self.position.x, self.position.y, 20f32),
            cam_front.add(Vec3::new(self.position.x, self.position.y, 0f32)),
            cam_up
        );

        self.view_matrix.inverse();

        self.view_matrix
    }

    pub fn get_proj_matrix(&self) -> Mat4 {
        self.proj_matrix
    }

    pub fn get_view_width(&self) -> f32 {
        self.view_width
    }

    pub fn get_view_height(&self) -> f32 {
        self.view_height
    }
}