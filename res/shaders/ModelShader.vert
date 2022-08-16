#version 130

in vec3 position;
in vec2 texCoord;
in vec3 normal;

out vec2 texCoord0;
out vec3 toLightVector;
out vec3 transformedNormal;

uniform vec3 lightPos;

uniform mat4 projectionMatrix;
uniform mat4 viewMatrix;
uniform mat4 modelMatrix;

void main() {
    texCoord0 = texCoord;

    transformedNormal = normalize((modelMatrix * vec4(normal, 0)).xyz);
    vec4 worldPosition = modelMatrix * vec4(position, 1);

    gl_Position = projectionMatrix * viewMatrix * worldPosition;
    toLightVector = normalize(lightPos - worldPosition.xyz);
}