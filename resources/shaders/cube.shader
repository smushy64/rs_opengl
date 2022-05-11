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

    v2f.normal = normalize( Normal );
    // band-aid solution, we really need to send in a normal matrix
    v2f.world_space_normal = normalize( mat3( transpose( inverse( model ) ) ) * Normal );

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

uniform sampler2D tex0;
uniform sampler2D tex1;

uniform vec3 world_space_light_position;
uniform vec3 lightColor;

uniform vec3 view_position;

out vec4 FRAG_COLOR;

void main()
{
    // ambient light
    vec4 ambient = vec4(0.0706, 0.0588, 0.0588, 1.0);

    // texture color | albedo
    vec4 albedo = mix(
        texture( tex0, v2f.uv ),
        texture( tex1, v2f.uv ),
        0.1
    );

    vec3 lightDirection = normalize( world_space_light_position - v2f.world_space_position.xyz );

    float lightStrength = 1.2;

    // light attenuation
    // dot product between light position and normal
    float attenuation = max(
        dot( lightDirection, v2f.world_space_normal ) * lightStrength,
        0.0
    );

    // diffuse lighting
    vec3 diffuse = lightColor * attenuation;

    // specular lighting
    float specularStrength = 0.5;

    vec3 viewDirection = normalize( view_position - v2f.world_space_position.xyz );
    vec3 reflectDirection = reflect( -lightDirection, v2f.world_space_normal );

    float glossiness = 32.0;

    float specularFalloff = pow(
        max( dot( viewDirection, reflectDirection ), 0.0 ),
        glossiness
    );

    vec3 specular = specularStrength * specularFalloff * lightColor;

    FRAG_COLOR = albedo * vec4( ambient.xyz + diffuse + specular, 1.0 );

}