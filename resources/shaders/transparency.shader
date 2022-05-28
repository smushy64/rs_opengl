#vertex -------------------------------------------------------------

#version 420 core

layout ( location = 0 ) in vec3 Position;
layout ( location = 2 ) in vec2 UV;

uniform mat4 transform;
uniform mat4 view;
uniform mat4 projection;

out struct {

    vec2 uv;

} v2f;

void main()
{
    v2f.uv = UV;

    gl_Position = projection * view * transform * vec4( Position, 1.0 );
}

#fragment -----------------------------------------------------------

#version 330 core

in struct {

    vec2 uv;

} v2f;

uniform sampler2D diffuse;

out vec4 FRAG_COLOR;
void main()
{
    FRAG_COLOR = texture( diffuse, v2f.uv );
}
