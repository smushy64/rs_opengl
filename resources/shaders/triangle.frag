#version 330 core

in struct {

    vec4 position;
    vec3 color;
    vec3 normal;
    vec2 uv;

} v2f;

uniform sampler2D sampler_texture;

out vec4 FRAG_COLOR;

void main()
{
    // FRAG_COLOR = vec4( v2f.color, 1.0 );
    // FRAG_COLOR = texture( sampler_texture, v2f.uv );
    // FRAG_COLOR = vec4( v2f.normal, 1.0 );

    vec3 light_pos = normalize(vec3( 1.0, 1.0, 0.0 ));

    vec4 tex = texture( sampler_texture, v2f.uv );

    float atten = max(dot( v2f.normal, light_pos ), 0.0);

    vec4 ambient = vec4(0.0706, 0.0863, 0.098, 1.0);

    FRAG_COLOR = (tex * atten) + ambient;

}