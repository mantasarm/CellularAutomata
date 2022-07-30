#version 140
in vec2 pos;
in vec4 color;

out vec4 fColor;

uniform mat4 uProjection;
uniform mat4 uView;

void main() {
    gl_Position = uProjection * uView * vec4(pos, 0.0, 1.0);

    fColor = color;
}