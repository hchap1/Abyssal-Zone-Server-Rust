#version 330 core
layout (location = 0) in vec2 aPos;
layout (location = 1) in vec2 aTexCoord;
layout (location = 2) in float isHover;

out vec2 TexCoord;

uniform float mouseX;
uniform float mouseY;
uniform float offset;

void main(){
	vec4 shiftedPosition = vec4(aPos.x, aPos.y, 1.0, 1.0);
	gl_Position = shiftedPosition;
	vec2 texCoord = aTexCoord;
	if (isHover == 1.0) {
		texCoord = vec2(aTexCoord.x, aTexCoord.y + offset);
	}
	else {
		texCoord = vec2(aTexCoord.x, aTexCoord.y);
	}
	TexCoord = texCoord;
}