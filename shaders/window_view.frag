#version 330 core

// Ins & Outs

in vec3 aFragPos;
in vec3 aNormal;
in vec2 aTexCoordMesh;
in vec2 aTexCoordFinal;

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

// Main

void main() {
    // Properties

    vec3 result = vec3(0);
    result += vec3(texture(texture0, aTexCoordMesh));
    result += vec3(texture(texture1, aTexCoordFinal));
    result *= material.diffuse;

    fragColor = vec4(result, 1.0);
}