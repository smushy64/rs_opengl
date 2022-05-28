#vertex -------------------------------------------------------------

#version 420 core

layout ( location = 0 ) in vec3 Position;
layout ( location = 1 ) in vec3 Normal;
layout ( location = 2 ) in vec2 UV;
layout ( location = 3 ) in vec3 Color;

out struct {

    vec3 local_position;
    vec4 world_position;

    vec3 vertex_color;

    vec3 normal;
    vec3 world_normal;

    vec2 uv;

} v2f;

uniform mat4 view;
uniform mat4 projection;
uniform mat4 transform;
uniform mat3 normal_mat;

void main()
{

    v2f.local_position = Position;
    v2f.world_position = transform * vec4( Position, 1.0 );

    v2f.vertex_color = Color;

    v2f.normal       = Normal;
    v2f.world_normal = normal_mat * Normal;

    v2f.uv = UV;

    gl_Position = projection * view * v2f.world_position;

}

#fragment -----------------------------------------------------------

#version 420 core

in struct {

    vec3 local_position;
    vec4 world_position;

    vec3 vertex_color;

    vec3 normal;
    vec3 world_normal;

    vec2 uv;

} v2f;

struct DirectionalLight {

    vec3 direction;

    vec3 color;
    vec3 ambient_color;

};

float PosDot( vec3 normal, vec3 direction ) {
    return max( dot( normal, direction ), 0.0 );
}

uniform DirectionalLight directional_light;
uniform vec3 camera_position;
uniform float glossiness = 1.0;
uniform sampler2D diffuse_texture;
uniform sampler2D specular_texture;

out vec4 FRAG_COLOR;
void main()
{

    vec3 fragmentPosition  = v2f.world_position.xyz;
    vec3 directionToCamera = normalize( camera_position - fragmentPosition );
    vec3 normal            = normalize( v2f.world_normal );

    vec3 diffuse_sample  = vec3( texture(  diffuse_texture, v2f.uv ) );
    vec3 specular_sample = vec3( texture( specular_texture, v2f.uv ) );

    vec3 result = vec3( 0.0 );

    vec3 ambient = directional_light.ambient_color * diffuse_sample;

    float diff   = PosDot( normal, directional_light.direction );
    vec3 diffuse = ( directional_light.color * diffuse_sample  ) * diff;

    vec3 reflectDirection = reflect( -directional_light.direction, normal );
    float spec = pow( PosDot( directionToCamera, reflectDirection ), glossiness );
    vec3 specular = directional_light.color * spec * specular_sample;

    result = ambient + diffuse + specular;

    FRAG_COLOR = vec4( result, 1.0 );

}
