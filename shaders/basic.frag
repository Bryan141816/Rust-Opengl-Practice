#version 330 core

in vec3 FragPos;
in vec3 Normal;
in vec2 TexCoords;

out vec4 FragColor;

uniform vec3 lightColor;
uniform vec3 lightDir;
uniform float lightIntensity;

uniform vec3 viewPos;

// Material properties
uniform vec3 materialAmbient;
uniform vec3 materialSpecular;
uniform float materialShininess;

uniform sampler2D texture1;
uniform bool hasTexture;

void main() {
    vec4 texColor;

    if (hasTexture) {
        texColor = texture(texture1, TexCoords);
    } else {
        texColor = vec4(0.5, 0.5, 0.5, 1.0);
    }

    vec3 norm = normalize(Normal);
    vec3 lightDirection = normalize(-lightDir);
    vec3 viewDirection = normalize(viewPos - FragPos);

    // Ambient lighting
    vec3 ambient = materialAmbient * lightColor * 0.15;

    // Diffuse lighting
    float diff = max(dot(norm, lightDirection), 0.0);
    vec3 diffuse = diff * lightColor;

    // Specular lighting
    vec3 reflectDirection = reflect(-lightDirection, norm);

    float spec = pow(
        max(dot(viewDirection, reflectDirection), 0.0),
        materialShininess
    );

    vec3 specular = materialSpecular * spec * lightColor;

    // Combine lighting
    vec3 result = (
        ambient + (diffuse + specular) * lightIntensity
    ) * texColor.rgb;

    FragColor = vec4(result, texColor.a);
}
