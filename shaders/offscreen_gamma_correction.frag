#version 330 core

out vec4 fragColor;
  
in vec2 aTexCoords;

uniform sampler2D screenTexture;

void main()
{ 
    vec3 color = texture(screenTexture, aTexCoords).rgb;
    
    float gamma = 2.2;
    vec3 mapped = pow(color, vec3(1.0 / gamma))
    fragColor = vec4(mapped, 1.0);
}