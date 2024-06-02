#version 330 core

in vec2 TexCoord;

in float redBrightness;
in float greenBrightness;
in float blueBrightness;
out vec4 FragColor;

uniform sampler2D blockTexture;

void main(){
	vec4 pC = texture(blockTexture, TexCoord);
	FragColor = vec4(pC.x * redBrightness, pC.y * greenBrightness, pC.z * blueBrightness, pC.w);
}