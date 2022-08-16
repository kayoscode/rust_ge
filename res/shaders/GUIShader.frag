#version 130
out vec4 color;

in vec2 texCoord;

uniform sampler2D guiTexture;

void main() {
    //color = texture(guiTexture, texCoord);
    //color = vec4(texCoord.x, texCoord.y, 0, 1);
	color = vec4(1, 1, 1, 1);
}