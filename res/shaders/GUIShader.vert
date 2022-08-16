#version 130
in vec2 position;

uniform vec2 pos;
uniform vec2 scale;

out vec2 texCoord;

void main() {
    gl_Position = vec4(position.x * scale.x, position.y * scale.y, 0, 1);

    texCoord = position * .5 + .5;
	texCoord.y = -texCoord.y;
}