#version 330 core
layout (location = 0) in vec2 aPos;
layout (location = 1) in vec2 aTexCoord;

out vec2 TexCoord;

void main(){
	vec4 shiftedPosition = vec4(aPos.x, aPos.y, 1.0, 1.0);
	gl_Position = shiftedPosition;
	vec2 texCoord = aTexCoord;
	TexCoord = texCoord;
}