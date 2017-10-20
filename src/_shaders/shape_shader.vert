#version 400

// Vertex attributes
in vec2 position;

// Uniforms
uniform vec2 button_offset;

void main() {
    gl_Position = vec4(position + button_offset, 0.0, 1.0);
}
