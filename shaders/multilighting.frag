#version 330 core

// Ins & Outs

in vec3 aFragPos;
in vec3 aNormal;
in vec2 aTexCoord;

out vec4 fragColor;

// Basic properties
 
uniform vec3 viewPos;
uniform sampler2D texture0;
uniform sampler2D texture1;

// Material

struct Material {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    float shininess;
}; 
  
uniform Material material;

// DirectionalLight

struct DirLight {
    vec3 direction;
  
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};  
#define NR_DIR_LIGHTS 3
uniform DirLight dirLights[NR_DIR_LIGHTS];
uniform int numDirLights;

// PointLight

struct PointLight {    
    vec3 position;
    
    float constant;
    float linear;
    float quadratic;  

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};  
#define NR_POINT_LIGHTS 5
uniform PointLight pointLights[NR_POINT_LIGHTS];
uniform int numPointLights;

// SpotLight

struct SpotLight {    
    vec3  position;
    vec3  direction;
    float cutOff;
    float outerCutOff;
    
    float constant;
    float linear;
    float quadratic;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};  
#define NR_SPOT_LIGHTS 5
uniform SpotLight spotLights[NR_SPOT_LIGHTS];
uniform int numSpotLights;

// Calc DirectionalLight

vec3 CalcDirLight(DirLight light, vec3 normal, vec3 viewDir, vec3 diffuseColor, vec3 specularColor)
{
    // Blinn-Phong
    vec3 lightDir = normalize(-light.direction);
    // diffuse shading
    float diff = max(dot(normal, lightDir), 0.0);
    // specular shading
    vec3 halfwayDir = normalize(lightDir + viewDir);  
    float spec      = pow(max(dot(normal, halfwayDir), 0.0), material.shininess);
    // combine results
    vec3 ambient  = light.ambient  *        diffuseColor * material.diffuse;
    vec3 diffuse  = light.diffuse  * diff * diffuseColor * material.diffuse;
    vec3 specular = light.specular * spec * specularColor * material.specular;
    return (ambient + diffuse + specular);
}

// Calc PointLight

vec3 CalcPointLight(PointLight light, vec3 normal, vec3 fragPos, vec3 viewDir, vec3 diffuseColor, vec3 specularColor)
{
    // Blinn-Phong
    vec3 lightDir = normalize(light.position - fragPos);
    // diffuse shading
    float diff = max(dot(normal, lightDir), 0.0);
    // specular shading
    vec3 halfwayDir = normalize(lightDir + viewDir);  
    float spec      = pow(max(dot(normal, halfwayDir), 0.0), material.shininess);
    // attenuation
    float distance    = length(light.position - fragPos);
    float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance * distance));
    // combine results
    vec3 ambient  = light.ambient  *        diffuseColor * material.diffuse;
    vec3 diffuse  = light.diffuse  * diff * diffuseColor * material.diffuse;
    vec3 specular = light.specular * spec * specularColor * material.specular;
    ambient  *= attenuation;    // world ranged global illumination will be provided by directional light, not here
    diffuse  *= attenuation;
    specular *= attenuation;
    return (ambient + diffuse + specular);
}

// Calc SpotLight

vec3 CalcSpotLight(SpotLight light, vec3 normal, vec3 fragPos, vec3 viewDir, vec3 diffuseColor, vec3 specularColor)
{
    // Blinn-Phong
    vec3 lightDir = normalize(light.position - fragPos);
    // diffuse shading
    float diff = max(dot(normal, lightDir), 0.0);
    // specular shading
    vec3 halfwayDir = normalize(lightDir + viewDir);  
    float spec      = pow(max(dot(normal, halfwayDir), 0.0), material.shininess);
    // attenuation
    float distance    = length(light.position - fragPos);
    float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance * distance));    
    // combine results
    vec3 ambient  = light.ambient  *        diffuseColor * material.diffuse;
    vec3 diffuse  = light.diffuse  * diff * diffuseColor * material.diffuse;
    vec3 specular = light.specular * spec * specularColor * material.specular;
    ambient  *= attenuation;    // world ranged global illumination will be provided by directional light, not here
    diffuse  *= attenuation;
    specular *= attenuation;

    // spot range
    float theta     = dot(lightDir, normalize(-light.direction));
    float epsilon   = light.cutOff - light.outerCutOff;
    float intensity = clamp((theta - light.outerCutOff) / epsilon, 0.0, 1.0);

    diffuse  *= intensity;
    specular *= intensity;

    return (ambient + diffuse + specular);
}

// Main

void main() {
    // Properties
    vec3 norm = normalize(aNormal);
    vec3 viewDir = normalize(viewPos - aFragPos);

    // Sample from texture
    vec3 color0 = vec3(texture(texture0, aTexCoord));
    vec3 color1 = vec3(texture(texture1, aTexCoord));

    vec3 result = vec3(0);
    // Directional lighting
    for(int i = 0; i < numDirLights; i++)
        result += CalcDirLight(dirLights[i], norm, viewDir, color0, color1);
    // Point lights
    for(int i = 0; i < numPointLights; i++)
        result += CalcPointLight(pointLights[i], norm, aFragPos, viewDir, color0, color1);
    // Spot light
    for(int i = 0; i < numSpotLights; i++)
        result += CalcSpotLight(spotLights[i], norm, aFragPos, viewDir, color0, color1);

    fragColor = vec4(result, 1.0);
}