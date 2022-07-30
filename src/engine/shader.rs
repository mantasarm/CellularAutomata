#![allow(dead_code)]

use miniquad::*;

pub fn vertex_shader() -> String {
    let vertex_shader_src = std::fs::read_to_string("src/engine/vertex.glsl").unwrap();

    vertex_shader_src
}

pub fn fragment_shader() -> String {
    let fragment_shader_src = std::fs::read_to_string("src/engine/fragment.glsl").unwrap();

    fragment_shader_src
}

pub fn meta() -> ShaderMeta {
    ShaderMeta {
        images: vec![],
        uniforms: UniformBlockLayout {
            uniforms: vec![UniformDesc::new("uProjection", UniformType::Mat4),
                            UniformDesc::new("uView", UniformType::Mat4)]
        },
    }
}

#[repr(C)]
pub struct Uniforms {
    pub uProjection: glam::Mat4,
    pub uView: glam::Mat4,
}