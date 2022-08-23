#version 130
out vec4 color;

in vec2 texCoord;

uniform sampler2D guiTexture;

void main() {
    color = texture(guiTexture, texCoord);
}