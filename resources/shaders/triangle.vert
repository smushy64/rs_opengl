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

void main()
{
    v2f.position = vec4( Position, 1.0 );
    v2f.color = vec3( Color );
    v2f.uv = UV;
    v2f.normal = normalize( Normal );
    gl_Position = v2f.position;
}