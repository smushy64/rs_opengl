#vertex -------------------------------------------------------------

#version 330 core

layout ( location = 0 ) in vec3 position;

uniform mat4 transform;
uniform mat4 projection_view;

void main() {
    gl_Position = projection_view * transform * position;
}

#fragment -----------------------------------------------------------

#version 330 core

out vec4 FRAG_COLOR;

void main() {
    FRAG_COLOR = vec4(1.0, 0.0, 1.0, 1.0);
}