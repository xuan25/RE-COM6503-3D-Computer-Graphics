#version 330 core

out vec4 fragColor;

struct Material {
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    float shininess;
}; 
  
uniform Material material;

void main() {
    fragColor = vec4(material.diffuse, 1.0);
}