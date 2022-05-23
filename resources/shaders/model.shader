#vertex -------------------------------------------------------------

#version 330 core

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

uniform mat4 transform;
uniform mat4 view;
uniform mat4 projection;

uniform mat3 normal_mat;

void main()
{

    v2f.local_position = Position;
    v2f.world_position = transform * vec4( Position, 1.0 );

    v2f.vertex_color = Color;

    v2f.normal = Normal;
    // v2f.world_normal = mat3( transpose( inverse( transform ) ) ) * Normal;
    v2f.world_normal = normal_mat * Normal;

    v2f.uv = UV;

    gl_Position = projection * view * v2f.world_position;

}

#fragment -----------------------------------------------------------

#version 330 core

in struct {

    vec3 local_position;
    vec4 world_position;

    vec3 vertex_color;

    vec3 normal;
    vec3 world_normal;

    vec2 uv;

} v2f;

uniform vec3 camera_position;

struct DirectionalLight {

    vec3 direction;

    vec3 color;
    vec3 ambient_color;

};
uniform DirectionalLight directional_light;

uniform vec3  diffuse_color;
uniform float specular_strength;
uniform float glossiness;

float PosDot( vec3 normal, vec3 direction ) {
    return max( dot( normal, direction ), 0.0 );
}

out vec4 FRAG_COLOR;
void main()
{

    vec3 fragmentPosition  = v2f.world_position.xyz;
    vec3 directionToCamera = normalize( camera_position - fragmentPosition );
    vec3 lightDirection    = normalize( -directional_light.direction );
    vec3 normal            = normalize( v2f.world_normal );

    vec3 result = vec3( 0.0 );

    vec3 ambient = directional_light.ambient_color * diffuse_color;

    float diff   = PosDot( normal, lightDirection );
    vec3 diffuse = ( directional_light.color * diffuse_color  ) * diff;

    vec3 reflectDirection = reflect( -lightDirection, normal );
    float spec = pow( PosDot( directionToCamera, reflectDirection ), glossiness );
    vec3 specular = directional_light.color * spec * specular_strength;

    result = ambient + diffuse + specular;

    FRAG_COLOR = vec4( result, 1.0 );

}
