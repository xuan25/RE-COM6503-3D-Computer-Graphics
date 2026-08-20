#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 normal;
layout (location = 2) in vec2 texCoord;

out vec3 aTexCoord;

uniform mat4 mvpMatrix;

void main() {
    gl_Position = (mvpMatrix * vec4(position, 1.0)).xyww;
    aTexCoord = position;
}