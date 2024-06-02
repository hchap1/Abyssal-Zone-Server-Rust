#version 330 core
layout (location = 0) in vec2 aPos;
layout (location = 1) in vec2 aTexCoord;
layout (location = 2) in float crouching;
layout (location = 3) in float frame;
layout (location = 4) in float direction;

out vec2 TexCoord;

out float redBrightness;
out float greenBrightness;
out float blueBrightness;

uniform float lightScale;
uniform vec4 lightSources[64];
uniform float lightCount;
uniform float blockX;
uniform float blockY;
uniform float zoom;
uniform float xOffset;
uniform float yOffset;
uniform float screenX;
uniform float screenY;
uniform float lightConstant;
uniform float texOffset;
uniform float torchLight;

void main(){
	vec4 shiftedPosition = vec4((aPos.x + xOffset) * zoom, (aPos.y + yOffset) * zoom, 1.0, 1.0);
	float x = shiftedPosition.x / zoom;
	float y = shiftedPosition.y / zoom;
	vec3 RGB = vec3(0.0,0.0,0.0);
	for (int i = 0; i < lightCount && i < 64; i++) {
		vec4 data = lightSources[i];
		float lx = ((data.x + 0.5) * blockX + xOffset);
		float ly = (data.y * blockY + yOffset);
		float dlx = 1-clamp(abs((lx-x) * screenX) / 1000.0, 0.0, 1.0);
		float dly = 1-clamp(abs((ly-y) * screenY) / 1000.0, 0.0, 1.0);
		float b = (pow(sin(dlx)*sin(dly), 2) * 3) * data.w;
		if (data.z == 3.0) {
			RGB = vec3(RGB.x + b, RGB.y + b, RGB.z + b);
		}	
		else if (data.z == 7.0) {
			RGB = vec3(RGB.x + b * 1.3 * torchLight, RGB.y + b * 0.6 * torchLight, RGB.z + b * 0.2 * torchLight);
		}
	} 
	gl_Position = shiftedPosition;
	vec2 texCoord;
	if (direction == 1.0) { 
		texCoord = vec2(aTexCoord.x + frame * 0.1, aTexCoord.y + crouching * 0.5); 
	}
	else if (direction ==-1.0) { 
		float xP = 0.0;
		if (aTexCoord.x == 0.0) { xP = 0.1; }
		else { xP = 0.0; }
		texCoord = vec2(xP + frame * 0.1, aTexCoord.y + crouching * 0.5); }
	else {
		texCoord = vec2(aTexCoord.x, aTexCoord.y + crouching * 0.5);
	}
	TexCoord = texCoord;
	float dx = 1-clamp(abs((x * screenX) / 1000.0), 0.0, 1.0);
	float dy = 1-clamp(abs((y * screenY) / 1000.0), 0.0, 1.0);
	float centerBrightness = (pow(sin(dx)*sin(dy), 2) * 1.6);
	redBrightness = (RGB.x + centerBrightness) * lightScale;
	greenBrightness = (RGB.y + centerBrightness) * lightScale;
	blueBrightness = (RGB.z + centerBrightness) * lightScale;
}