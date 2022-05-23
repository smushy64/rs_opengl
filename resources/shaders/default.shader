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
uniform mat4 camera_transform;
uniform mat4 projection;

void main()
{
    v2f.local_position = Position;
    v2f.world_position = transform * vec4( Position, 1.0 );

    v2f.vertex_color = Color;

    v2f.normal = Normal;
    // band-aid solution, we really need to send in a normal matrix
    v2f.world_normal = mat3( transpose( inverse( transform ) ) ) * Normal;

    v2f.uv = UV;

    gl_Position = projection * camera_transform * v2f.world_position;
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

struct Material {

    sampler2D diffuse_sampler;
    sampler2D specular_sampler;
    float glossiness;

};
uniform Material material;

struct DirectionalLight {

    vec3 direction;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;

};
uniform DirectionalLight directional_light;

struct PointLight {

    vec3 position;

    float constant;
    float linear;
    float quadratic;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;

};

#define NR_POINT_LIGHTS 4
uniform PointLight point_lights[ NR_POINT_LIGHTS ];

struct SpotLight {

    vec3 position;
    vec3 direction;

    float inner_cutoff;
    float outer_cutoff;

    float constant;
    float linear;
    float quadratic;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;

};

#define NR_SPOT_LIGHTS 1
uniform SpotLight spot_lights[ NR_SPOT_LIGHTS ];

vec3 CalculateLight(
    vec3 light_direction, vec3 normal, vec3 camera_direction,
    vec3 diffuse_texture, vec3 specular_texture,
    vec3 ambient, vec3 diffuse, vec3 specular
);

float CalculateAttenuation(
    vec3 light_position, vec3 world_position,
    float constant, float linear, float quadratic
);

float CalculateSpotLightIntensity( SpotLight spot_light, vec3 direction_to_spot_light );

out vec4 FRAG_COLOR;
void main()
{

    // sample textures
    vec3 diffuseTexture  = vec3( texture( material.diffuse_sampler,  v2f.uv ) );
    vec3 specularTexture = vec3( texture( material.specular_sampler, v2f.uv ) );

    vec3 worldPosition = v2f.world_position.xyz;

    vec3 cameraDirection = normalize( camera_position - worldPosition );
    vec3 normal = normalize( v2f.world_normal );

    vec3 result = vec3( 0.0 );

    // directional light
    result += CalculateLight(
        normalize( -directional_light.direction ), normal, cameraDirection,
        diffuseTexture, specularTexture,
        directional_light.ambient, directional_light.diffuse, directional_light.specular
    );

    // point lights
    for( int i = 0; i < NR_POINT_LIGHTS; ++i ) {
        result += CalculateLight(
            normalize( point_lights[i].position - worldPosition ), normal, cameraDirection,
            diffuseTexture, specularTexture,
            point_lights[i].ambient, point_lights[i].diffuse, point_lights[i].specular
        ) * CalculateAttenuation(
            point_lights[i].position, worldPosition,
            point_lights[i].constant, point_lights[i].linear, point_lights[i].quadratic
        );
    }

    // spot lights
    for( int i = 0; i < NR_SPOT_LIGHTS; ++i ) {

        vec3 directionToSpotLight = normalize(spot_lights[i].position - worldPosition);

        result += CalculateLight(
            directionToSpotLight, normal, cameraDirection,
            diffuseTexture, specularTexture,
            spot_lights[i].ambient, spot_lights[i].diffuse, spot_lights[i].specular
        ) * CalculateAttenuation(
            spot_lights[i].position, worldPosition,
            spot_lights[i].constant, spot_lights[i].linear, spot_lights[i].quadratic
        ) * CalculateSpotLightIntensity(
            spot_lights[i], directionToSpotLight
        );
        
    }

    FRAG_COLOR = vec4( result, 1.0 );

}

float CalculateSpotLightIntensity( SpotLight spot_light, vec3 direction_to_spot_light ) {
    float theta = dot( direction_to_spot_light, normalize( -spot_light.direction ) );
    float epsilon = spot_light.inner_cutoff - spot_light.outer_cutoff;
    return clamp( ( theta - spot_light.outer_cutoff ) / epsilon, 0.0, 1.0 );
}

float ClampedDot( vec3 normal, vec3 light_direction ) {
    return max( dot( normal, light_direction ), 0.0 );
}

float SpecularShading( vec3 light_direction, vec3 normal, vec3 camera_direction ) {
    vec3 reflectDirection = reflect( -light_direction, normal );
    return pow(
        ClampedDot( camera_direction, reflectDirection ),
        material.glossiness
    );
}

vec3 CalculateLight(
    vec3 light_direction, vec3 normal, vec3 camera_direction,
    vec3 diffuse_texture, vec3 specular_texture,
    vec3 ambient, vec3 diffuse, vec3 specular
)
{
    vec3 resultAmbient  =   ambient  * diffuse_texture;
    vec3 resultDiffuse  = ( diffuse  * diffuse_texture  ) * ClampedDot( normal, light_direction );
    vec3 resultSpecular = specular * specular_texture * SpecularShading( light_direction, normal, camera_direction );

    return ( resultAmbient + resultDiffuse + resultSpecular );
}

float CalculateAttenuation(
    vec3 light_position, vec3 world_position,
    float constant, float linear, float quadratic
)
{
    float dist = length( light_position - world_position );
    return 1.0 / ( constant + linear * dist +
        quadratic * ( dist * dist ) );
}
