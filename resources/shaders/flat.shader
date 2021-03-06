#vertex -------------------------------------------------------------

#version 420 core

layout ( location = 0 ) in vec3 Position;

uniform mat4 transform;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    gl_Position = projection * view * transform * vec4( Position, 1.0 );
}

#fragment -----------------------------------------------------------

#version 330 core

uniform vec3 color;

out vec4 FRAG_COLOR;
void main()
{
    FRAG_COLOR = vec4( color, 1.0 );
}
