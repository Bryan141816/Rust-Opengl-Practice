#version 330 core

struct DirectionalLight {
    vec3 direction;
    vec3 color;
    float intensity;
};
struct PointLight {
    vec3 position;
    vec3 color;
    float intensity;

    float constant;
    float linear;
    float quadratic;
};
struct SpotLight {
    vec3 position;
    vec3 direction;
    vec3 color;
    float intensity;

    float cutOff;
    float outerCutOff;
};

in vec3 FragPos;
in vec3 Normal;
in vec2 TexCoords;

out vec4 FragColor;

uniform vec3 viewPos;

// Material properties
uniform vec3 materialAmbient;
uniform vec3 materialSpecular;
uniform vec3 materialDiffuse; 
uniform float materialShininess;

uniform sampler2D texture1;
uniform bool hasTexture;

#define MAX_DIRECTIONAL_LIGHTS 4
#define MAX_POINT_LIGHTS 16
#define MAX_SPOT_LIGHTS 4

uniform DirectionalLight directionalLights[MAX_DIRECTIONAL_LIGHTS];
uniform PointLight pointLights[MAX_POINT_LIGHTS];
uniform SpotLight spotLights[MAX_SPOT_LIGHTS];

uniform int directionalLightCount;
uniform int pointLightCount;
uniform int spotLightCount;

vec3 calculatePhong(
    vec3 normal,
    vec3 lightDirection,
    vec3 viewDirection,
    vec3 lightColor,
    float intensity
) {
    normal = normalize(normal);
    lightDirection = normalize(lightDirection);
    viewDirection = normalize(viewDirection);

    // Ambient
    vec3 ambient = materialAmbient * lightColor * 0.05;

    // Diffuse
    float diff = max(dot(normal, lightDirection), 0.0);
    vec3 diffuse = materialDiffuse * diff * lightColor;

    // Specular
    vec3 reflectDirection = reflect(-lightDirection, normal);
    float spec = pow(
        max(dot(viewDirection, reflectDirection), 0.0),
        materialShininess
    );

    vec3 specular = materialSpecular * spec * lightColor;

    return ambient + (diffuse + specular) * intensity;
}

vec3 calculateDirectionalLight(
    DirectionalLight light,
    vec3 normal,
    vec3 viewDirection
){
    vec3 lightDirection = normalize(-light.direction);

    return calculatePhong(
        normal,
        lightDirection,
        viewDirection,
        light.color,
        light.intensity
    );
}
vec3 calculatePointLight(
    PointLight light,
    vec3 normal,
    vec3 fragPos,
    vec3 viewDirection
) {
    vec3 lightDirection = normalize(light.position - fragPos);

    float distance = length(light.position - fragPos);

    float attenution = 1.0 / (
        light.constant +
        light.linear * distance +
        light.quadratic * distance * distance
    );

    vec3 result = calculatePhong(
        normal,
        lightDirection,
        viewDirection,
        light.color,
        light.intensity
    );
    return result * attenution;
}
vec3 calculateSpotLight(
    SpotLight light,
    vec3 normal,
    vec3 fragPos,
    vec3 viewDirection
) {
    vec3 lightDirection = normalize(light.position - fragPos);

    vec3 lighToFragment = normalize(fragPos - light.position);

    float theta = dot(
        lighToFragment,
        normalize(light.direction)
    );

    float epsilon = light.cutOff - light.outerCutOff;

    float intensity = clamp(
        (theta - light.outerCutOff) / epsilon,
        0.0,
        1.0
    );

    vec3 result = calculatePhong(
        normal,
        lightDirection,
        viewDirection,
        light.color,
        light.intensity
    );
    return result * intensity;
}
void main() {
    vec4 texColor;

    if (hasTexture) {
        texColor = texture(texture1, TexCoords);
    } else {
        texColor = vec4(0.5, 0.5, 0.5, 1.0);
    }

    vec3 norm = normalize(Normal);
    vec3 viewDirection = normalize(viewPos - FragPos);

    vec3 result = vec3(0.0);

    for (int i = 0; i < directionalLightCount; i++) {
        result += calculateDirectionalLight(
            directionalLights[i],
            norm,
            viewDirection
        );
    }
    for (int i = 0; i < pointLightCount; i++) {
        result += calculatePointLight(
            pointLights[i],
            norm,
            FragPos,
            viewDirection
        );
    }
    for (int i = 0; i < spotLightCount; i++) {
        result += calculateSpotLight(
            spotLights[i],
            norm,
            FragPos,
            viewDirection
        );
    }
    result *= texColor.rgb;
    FragColor = vec4(result, texColor.a);
}

