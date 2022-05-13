#vertex -------------------------------------------------------------

#version 330 core

layout ( location = 0 ) in vec3 Position;
layout ( location = 1 ) in vec3 Color;
layout ( location = 2 ) in vec3 Normal;
layout ( location = 3 ) in vec2 UV;

out struct {

    vec3 model_space_position;
    vec4 world_space_position;

    vec3 vertex_color;

    vec3 normal;
    vec3 world_space_normal;

    vec2 uv;

} v2f;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    v2f.model_space_position = Position;
    v2f.world_space_position = model * vec4( Position, 1.0 );

    v2f.vertex_color = Color;

    v2f.normal = Normal;
    // band-aid solution, we really need to send in a normal matrix
    v2f.world_space_normal = mat3( transpose( inverse( model ) ) ) * Normal;

    v2f.uv = UV;

    gl_Position = projection * view * v2f.world_space_position;
}

#fragment -----------------------------------------------------------

#version 330 core

in struct {

    vec3 model_space_position;
    vec4 world_space_position;

    vec3 vertex_color;

    vec3 normal;
    vec3 world_space_normal;

    vec2 uv;

} v2f;

struct Material {

    sampler2D diffuse;
    sampler2D specular;
    float shininess;

};

struct Light {

    vec3 position;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;

};

uniform Material material;
uniform Light light;

uniform vec3 view_position;

out vec4 FRAG_COLOR;

void main()
{

    vec3 diffuse_texture = vec3( texture( material.diffuse, v2f.uv ) );
    vec3 specular_texture = vec3( texture( material.specular, v2f.uv ) );

    vec3 ambient = light.ambient * diffuse_texture;

    // normalized here because of interpolation
    vec3 normal = normalize( v2f.world_space_normal );

    vec3 lightDirection = normalize( light.position - v2f.world_space_position.xyz );
    float diff = max( dot( lightDirection, normal ), 0.0 );
    vec3 diffuse = light.diffuse * ( diff * diffuse_texture );

    vec3 viewDirection = normalize( view_position - v2f.world_space_position.xyz );
    vec3 reflectDirection = reflect( -lightDirection, normal );
    float spec = pow( max( dot( viewDirection, reflectDirection ), 0.0 ), material.shininess );
    vec3 specular = light.specular * ( spec * specular_texture );

    vec3 result = ambient + diffuse + specular;
    FRAG_COLOR = vec4( result, 1.0 );

}