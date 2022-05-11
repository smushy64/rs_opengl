#vertex -------------------------------------------------------------

#version 330 core

layout ( location = 0 ) in vec3 Position;
layout ( location = 1 ) in vec3 Color;
layout ( location = 2 ) in vec3 Normal;
layout ( location = 3 ) in vec2 UV;

out struct {

    vec4 position;
    vec3 color;
    vec3 normal;
    vec2 uv;

} v2f;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    v2f.position = vec4( Position, 1.0 );
    v2f.color = Color;
    v2f.uv = UV;
    v2f.normal = normalize( Normal );
    gl_Position = projection * view * model * v2f.position;
}

#fragment -----------------------------------------------------------

#version 330 core

in struct {

    vec4 position;
    vec3 color;
    vec3 normal;
    vec2 uv;

} v2f;

uniform vec3 lightColor;

out vec4 FRAG_COLOR;

void main()
{
    FRAG_COLOR = vec4( lightColor, 1.0 );
}