#version 330 core

out vec4 FragColor;

uniform vec4 material;

void main() {
    FragColor = material;
}