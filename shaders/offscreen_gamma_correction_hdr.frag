#version 330 core

out vec4 fragColor;
  
in vec2 aTexCoords;

uniform sampler2D screenTexture;

uniform float exposure;

void main()
{ 
    vec3 hdrColor = texture(screenTexture, aTexCoords).rgb;
    
    // exposure tone mapping
    vec3 mapped = vec3(1.0) - exp(-hdrColor * exposure);

    // gamma correction
    float gamma = 2.2; 
    mapped = pow(mapped, vec3(1.0 / gamma));
  
    fragColor = vec4(mapped, 1.0);
}