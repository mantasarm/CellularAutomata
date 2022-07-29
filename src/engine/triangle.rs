#![allow(dead_code)]

use std::vec;

use glam::{Vec2, Vec3};
use miniquad::*;

use super::shader;

struct Vertex {
    pos: Vec2,
    color: Vec3
}

pub fn create_triangle(ctx: &mut Context) -> (Pipeline, Bindings){
    #[rustfmt::skip]
    let vertices: [Vertex; 3] = [
        Vertex { pos: Vec2 { x: -100f32, y: -100f32 }, color: Vec3 { x: 1.0, y: 0.0, z: 0.0 }},
        Vertex { pos: Vec2 { x: 100f32, y: -100f32 }, color: Vec3 { x: 0.0, y: 1.0, z: 0.0 }},
        Vertex { pos: Vec2 { x: 100f32, y: 100f32 }, color: Vec3 { x: 0.0, y: 0.0, z: 1.0 }},
    ];
    let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

    let indices: [u16; 3] = [0, 1, 2];
    let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

    let bindings = Bindings {
        vertex_buffers: vec![vertex_buffer],
        index_buffer,
        images: vec![],
    };

    let shader = Shader::new(ctx, &shader::vertex_shader(), &shader::fragment_shader(), shader::meta()).unwrap();

    let pipeline = Pipeline::new(
        ctx, 
        &[BufferLayout::default()], 
        &[VertexAttribute::new("pos", VertexFormat::Float2),
        VertexAttribute::new("color", VertexFormat::Float3)], 
        shader
    );

    (pipeline, bindings)
}