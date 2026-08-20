#version 330 core

// Ins & Outs

in vec3 aTexCoord;

out vec4 fragColor;

// Basic properties
 
uniform samplerCube texture0;

// Material

struct Material {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    float shininess;
}; 
  
uniform Material material;

// Main

void main() {
    fragColor = texture(texture0, aTexCoord) * vec4(material.diffuse, 1.0);
}