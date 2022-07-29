#![allow(dead_code)]

use glam::{Vec2};
use miniquad::{Shader, Bindings, Pipeline, Context, Buffer, BufferLayout, VertexAttribute, VertexFormat, BufferType, BlendState, BlendFactor, Equation, BlendValue};

use super::{shader, camera::Camera};

#[derive(Copy, Clone)]
struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32
}

struct Vertex {
    pos: Vec2,
    color: Color,
}

pub struct ShapeBatch {
    vertices: Vec<Vertex>,
    indices: Vec<i32>,
    color: Color,

    bindings: Bindings,
    pipeline: Pipeline,
    max_batch_size: usize
}

impl ShapeBatch {
    pub fn new(ctx: &mut Context, max_batch_size: usize) -> Self {
        let shader = Shader::new(ctx, &shader::vertex_shader(), &shader::fragment_shader(), shader::meta()).unwrap();

        let vertices: Vec<Vertex> = Vec::new();
        let indices: Vec<i32> = Vec::new();

        let vertex_buffer = Buffer::stream(ctx, BufferType::VertexBuffer, std::mem::size_of::<f32>() * max_batch_size * std::mem::size_of::<Vertex>());

        let index_buffer = Buffer::stream(ctx, BufferType::IndexBuffer, max_batch_size * std::mem::size_of::<f32>());

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![],
        };
    
        let pipeline = Pipeline::new(
            ctx, 
            &[BufferLayout::default()], 
            &[VertexAttribute::with_buffer("pos", VertexFormat::Float2, 0),
            VertexAttribute::with_buffer("color", VertexFormat::Float4, 0)], 
            shader
        );

        Self { 
            vertices, 
            indices,
            color: Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },

            bindings,
            pipeline,
            max_batch_size
        }
    }

    pub fn begin(&mut self) {
        self.vertices.clear();
        self.indices.clear();
    }

    pub fn set_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.color = Color {r, g, b , a};
    }

    pub fn draw_triangle(&mut self, x: f32, y: f32, x1: f32, y1: f32, x2: f32, y2: f32) {
        self.vertices.push(Vertex { pos: Vec2 { x, y}, color: self.color});
        self.vertices.push(Vertex { pos: Vec2 { x: x1, y: y1}, color: self.color});
        self.vertices.push(Vertex { pos: Vec2 { x: x2, y: y2}, color: self.color});

        let i: i32 = self.vertices.len() as i32;
        self.indices.push(i - 3);
        self.indices.push(i - 2);
        self.indices.push(i - 1);
    }

    pub fn draw_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.vertices.push(Vertex { pos: Vec2 { x, y}, color: self.color});
        self.vertices.push(Vertex { pos: Vec2 { x: x + width, y}, color: self.color});
        self.vertices.push(Vertex { pos: Vec2 { x: x + width, y: y + height}, color: self.color});
        self.vertices.push(Vertex { pos: Vec2 { x, y: y + height}, color: self.color});

        let i: i32 = self.vertices.len() as i32;
        self.indices.push(i - 4);
        self.indices.push(i - 3);
        self.indices.push(i - 2);

        self.indices.push(i - 2);
        self.indices.push(i - 4);
        self.indices.push(i - 1);
    }

    pub fn end(&mut self, ctx: &mut Context, camera: &mut Camera) {
        self.bindings.vertex_buffers[0].update(ctx, &self.vertices);

        self.bindings.index_buffer.delete();
        self.bindings.index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &self.indices);

        ctx.apply_pipeline(&self.pipeline);

        // let color_blend = BlendState::new(
        //     Equation::Add,
        //     BlendFactor::Value(BlendValue::SourceColor),
        //     BlendFactor::OneMinusValue(BlendValue::SourceColor),
        // );
        // let alpha_blend = BlendState::new(
        //     Equation::Add,
        //     BlendFactor::Value(BlendValue::SourceAlpha),
        //     BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
        // );
        // ctx.set_blend(Some(color_blend), Some(alpha_blend));
        
        ctx.apply_bindings(&self.bindings);
        ctx.apply_uniforms(&shader::Uniforms { uProjection: camera.get_proj_matrix(), uView: camera.get_view_matrix()});

        ctx.draw(0, self.indices.len() as i32, 1);

    }

    pub fn get_num_indices(&self) -> i32 {
        self.indices.len() as i32
    }

    pub fn get_num_vertices(&self) -> i32 {
        self.vertices.len() as i32
    }
}

struct ShapeRenderer {
    batches: Vec<ShapeBatch>
}

impl ShapeRenderer {
    
}