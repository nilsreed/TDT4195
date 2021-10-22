#version 430 core

in layout(location=1) vec4 colour_out;
in layout(location=3) vec3 normal_out;
out vec4 color;

void main()
{
    vec3 lightDirection = normalize(vec3(0.8, -0.5, 0.6));
    vec3 colour_rgb = vec3(colour_out[0], colour_out[1], colour_out[2])*dot(normal_out, -lightDirection);
    color = vec4(colour_rgb[0], colour_rgb[1], colour_rgb[2], colour_out[3]);
}