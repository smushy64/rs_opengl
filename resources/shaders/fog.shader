#vertex -------------------------------------------------------------

#version 330 core

layout ( location = 0 ) in vec3 Position;
layout ( location = 2 ) in vec2 UV;

out struct {

    vec3 local_position;
    vec4 world_position;

    vec2 uv;

} v2f;

uniform mat4 transform;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    v2f.local_position = Position;
    v2f.world_position = transform * vec4( Position, 1.0 );

    v2f.uv = UV;

    gl_Position = projection * view * v2f.world_position;
}

#fragment -----------------------------------------------------------

#version 330 core

in struct {

    vec3 local_position;
    vec4 world_position;

    vec2 uv;

} v2f;

uniform sampler2D diffuse;
uniform vec3 fog_color;

float near = 000.01;
float far  = 100.00;

float LinearDepth( float depth );

out vec4 FRAG_COLOR;
void main()
{
    float depth_mask = LinearDepth( gl_FragCoord.z ) / far;
    vec3  tex_sample = vec3( texture( diffuse, v2f.uv * 100.0 ) ) * ( 1.0 - depth_mask );

    vec3 fog = fog_color * depth_mask;

    vec3 result = tex_sample + fog;

    FRAG_COLOR = vec4( result, 1.0 );
    // FRAG_COLOR = vec4( vec3( depth_mask ), 1.0 );
}

float LinearDepth( float depth ) {
    // normalized device coordinates
    float ndc = depth * 2.0 - 1.0;
    return ( 2.0 * near * far ) / ( far + near - ndc * ( far - near ) );
}
