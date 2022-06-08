#vertex -------------------------------------------------------------

#version 460 core

layout ( location = 0 ) in vec3 Position;
layout ( location = 1 ) in vec3 Normal;
layout ( location = 2 ) in vec4 Tangent;
layout ( location = 3 ) in vec2 UV;
layout ( location = 4 ) in vec3 Color;

out struct {

    vec3 local_position;
    vec3 world_position;

    vec3 normal;
    vec3 tangent;
    vec3 bitangent;

    vec2 uv;

    vec3 color;

} v2f;

// NOTE: Matrices Block | 128 bytes
layout (std140) uniform Matrices {

    mat4 view;       // 64
    mat4 projection; // 64

};

uniform mat4 model;
uniform mat3 normal_mat;

void main()
{

    v2f.local_position  = Position;

    vec4 worldPosition = model * vec4( Position, 1.0 );
    v2f.world_position = worldPosition.xyz;

    v2f.normal    = normal_mat * Normal; 
    v2f.tangent   = normal_mat * Tangent.xyz;
    v2f.bitangent = cross( v2f.tangent, v2f.normal );

    v2f.uv    = UV;
    v2f.color = Color;

    gl_Position = projection * view * worldPosition;
}

#fragment -----------------------------------------------------------

#version 460 core

in struct {

    vec3 local_position;
    vec3 world_position;

    vec3 normal;
    vec3 tangent;
    vec3 bitangent;

    vec2 uv;

    vec3 color;

} v2f;

// NOTE: Directional struct | 48 bytes
struct Directional {
    vec4 direction; // 16 | offset: 0
    vec4 diffuse;   // 16 | offset: 16
    vec4 specular;  // 16 | offset: 32
};

#define MAX_POINT_LIGHTS 4 // NOTE: Point lights | 256 bytes
// NOTE: Point struct | 64 bytes
struct Point {
    vec4 position;   // 16 | offset: 0
    vec4 diffuse;    // 16 | offset: 16
    vec4 specular;   // 16 | offset: 32

    float constant;  // 4  | offset: 48
    float linear;    // 4  | offset: 52
    float quadratic; // 4  | offset: 56

    bool  is_active; // 4  | offset: 60
};

#define MAX_SPOT_LIGHTS 2 // NOTE: Spot lights | 192 bytes
// NOTE: Spot struct | 96 bytes
struct Spot {
    vec4 position;      // 16 | offset: 0
    vec4 direction;     // 16 | offset: 16
    vec4 diffuse;       // 16 | offset: 32
    vec4 specular;      // 16 | offset: 48

    float inner_cutoff; // 4  | offset: 64
    float outer_cutoff; // 4  | offset: 68
    float constant;     // 4  | offset: 72
    float linear;       // 4  | offset: 76
    float quadratic;    // 4  | offset: 80

    bool  is_active;    // 4  | offset: 84
    // 88, padded to 96
};

// NOTE: Lights Block | 496 bytes
layout (std140) uniform Lights {
    Directional directional_light;                // 48  | offset: 0
    Point       point_lights[ MAX_POINT_LIGHTS ]; // 256 | offset: 48 
    Spot        spot_lights[ MAX_SPOT_LIGHTS ];   // 192 | offset: 304
    // 496
};

// NOTE: Data Block | 40 bytes
layout (std140) uniform Data {
    vec4  camera_position; // 16 | offset: 0
    vec4  fog_color;       // 16 | offset: 16
    float near_clip;       // 4  | offset: 32
    float far_clip;        // 4  | offset: 36
    // 40, padded to 48
};

uniform sampler2D albedo_sampler;
uniform vec2 albedo_sampler_scaler;
uniform sampler2D specular_sampler;
uniform vec2 specular_sampler_scaler;
uniform bool      use_vertex_color;
uniform float     glossiness;

vec3 DirectionalLight(
    vec3 dir, vec3 norm, vec3 cam_dir,
    vec3 albedo_texture, vec3 spec_texture,
    vec3 diffuse, vec3 specular
);

vec3 PointLight(
    vec3 pos, vec3 world_pos,
    vec3 norm, vec3 cam_dir,
    vec3 albedo_texture, vec3 spec_texture,
    vec3 diffuse, vec3 specular,
    float constant, float lin, float quad
);

vec3 SpotLight(
    vec3 pos, vec3 dir, vec3 world_pos,
    vec3 norm, vec3 cam_dir,
    vec3 albedo_texture, vec3 spec_texture,
    vec3 diffuse, vec3 specular,
    float constant, float lin, float quad,
    float inner, float outer
);

float Spec( vec3 dir, vec3 norm, vec3 cam_dir );

out vec4 FRAG_COLOR;
void main()
{
    vec3 albedoTexture   = texture2D( albedo_sampler,   v2f.uv * albedo_sampler_scaler ).rgb;
    vec3 specularTexture = texture2D( specular_sampler, v2f.uv * specular_sampler_scaler ).rgb;

    vec3 cameraDirectionRaw = camera_position.xyz - v2f.world_position;

    float distance_to_camera = length( cameraDirectionRaw );
    float fog_mask = smoothstep( near_clip, far_clip, distance_to_camera );

    vec3 fog = fog_color.rgb * fog_mask;

    if( use_vertex_color ) {
        albedoTexture = v2f.color;
    }

    vec3 cameraDirection = normalize( cameraDirectionRaw );
    vec3 normal    = normalize( v2f.normal );
    vec3 tangent   = normalize( v2f.tangent );
    vec3 bitangent = normalize( v2f.bitangent );

    vec3 color = vec3(0.0);

    color += DirectionalLight(
        normalize( directional_light.direction.xyz ), normal, cameraDirection,
        albedoTexture, specularTexture,
        directional_light.diffuse.rgb, directional_light.specular.rgb
    );

    for( int i = 0; i < MAX_POINT_LIGHTS; ++i ) {
        if( !point_lights[i].is_active ) { continue; }
        color += PointLight(
            point_lights[i].position.xyz, v2f.world_position,
            normal, cameraDirection,
            albedoTexture, specularTexture,
            point_lights[i].diffuse.rgb, point_lights[i].specular.rgb,
            point_lights[i].constant, point_lights[i].linear, point_lights[i].quadratic
        );
    }

    for( int i = 0; i < MAX_SPOT_LIGHTS; ++i ) {
        if( !spot_lights[i].is_active ) { continue; }
        color += SpotLight(
            spot_lights[i].position.xyz, spot_lights[i].direction.xyz, v2f.world_position,
            normal, cameraDirection,
            albedoTexture, specularTexture,
            spot_lights[i].diffuse.rgb, spot_lights[i].specular.rgb,
            spot_lights[i].constant, spot_lights[i].linear, spot_lights[i].quadratic,
            spot_lights[i].inner_cutoff, spot_lights[i].outer_cutoff
        );
    }

    color = ( color * ( 1.0 - fog_mask ) ) + fog;

    FRAG_COLOR = vec4( color, 1.0 );

}

vec3 SpotLight(
    vec3 pos, vec3 dir, vec3 world_pos,
    vec3 norm, vec3 cam_dir,
    vec3 albedo_texture, vec3 spec_texture,
    vec3 diffuse, vec3 specular,
    float constant, float lin, float quad,
    float inner, float outer
)
{
    vec3 lightColor = PointLight(
        pos, world_pos, norm, cam_dir, albedo_texture, spec_texture,
        diffuse, specular, constant, lin, quad
    );
    vec3  dir2    = normalize( pos - world_pos );
    float theta   = dot( dir2, normalize( -dir ) );
    float epsilon = inner - outer;
    float intensity = clamp( ( theta - outer ) / epsilon, 0.0, 1.0 );

    return lightColor * intensity;
}

vec3 PointLight(
    vec3 pos, vec3 world_pos,
    vec3 norm, vec3 cam_dir,
    vec3 albedo_texture, vec3 spec_texture,
    vec3 diffuse, vec3 specular,
    float constant, float lin, float quad
)
{
    vec3 lightDir = pos - world_pos;
    vec3 lightColor = DirectionalLight(
        normalize( lightDir ), norm, cam_dir,
        albedo_texture, spec_texture,
        diffuse, specular
    );
    float dist = length( lightDir );
    float atten = 1.0 / ( constant + lin * dist + quad * ( dist * dist ) );

    return lightColor * atten;
}

vec3 DirectionalLight(
    vec3 dir, vec3 norm, vec3 cam_dir,
    vec3 albedo_texture, vec3 spec_texture,
    vec3 diffuse, vec3 specular
)
{
    float cutoff = max( dot( norm, dir ), 0.0 );
    vec3 amb  = ( albedo_texture * diffuse ) * 0.3;
    vec3 diff = ( albedo_texture * diffuse  ) * cutoff;
    vec3 spec = (spec_texture * specular) * Spec( dir, norm, cam_dir );

    return diff + amb + ( spec * cutoff );
}

float Spec( vec3 dir, vec3 norm, vec3 cam_dir ) {
    vec3 reflectDirection = reflect( -dir, norm );
    return pow(
        max( dot( cam_dir, reflectDirection ), 0.0 ),
        max( glossiness, 1.0 )
    );
}
