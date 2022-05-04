#version 330 core

in struct {

    vec4 position;
    vec3 color;
    vec3 normal;
    vec2 uv;

} v2f;

out vec4 FRAG_COLOR;

void main()
{
    FRAG_COLOR = vec4( v2f.color, 1.0 );
}