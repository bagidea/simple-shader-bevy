#version 450

layout(location = 0) in vec2 position;
layout(location = 1) in vec2 uv;
layout(location = 2) in vec4 color;

layout(location = 0) out vec2 frag_uv;
layout(location = 1) out vec4 frag_color;

layout(set = 0, binding = 0) uniform mat4 model_view_projection;

void main() {
    gl_Position = model_view_projection * vec4(position, 0.0, 1.0);
    
    frag_uv = uv;
    frag_color = color;
}