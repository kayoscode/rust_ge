#version 130

in vec2 texCoord0;
in vec3 transformedNormal;
in vec3 toLightVector;

out vec4 color;

uniform sampler2D diffuseTexture;

void main() {
    float brightness = dot(transformedNormal, toLightVector);
    brightness = max(brightness, .2);

    color = brightness * texture(diffuseTexture, texCoord0);
}