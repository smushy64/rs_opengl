#vertex -------------------------------------------------------------

#version 330 core

layout ( location = 0 ) in vec3 Position;
layout ( location = 1 ) in vec3 Color;
layout ( location = 2 ) in vec3 Normal;
layout ( location = 3 ) in vec2 UV;

out struct {

    vec3 local_position;
    vec4 world_position;

} v2f;

uniform mat4 transform;
uniform mat4 camera_transform;
uniform mat4 projection;

void main()
{
    v2f.local_position = Position;
    v2f.world_position = transform * vec4( Position, 1.0 );

    gl_Position = projection * camera_transform * v2f.world_position;
}

#fragment -----------------------------------------------------------

#version 330 core

in struct {

    vec3 local_position;
    vec4 world_position;

} v2f;

uniform vec3 color;

out vec4 FRAG_COLOR;
void main()
{
    FRAG_COLOR = vec4( color, 1.0 );
}
