#version 430 core

in layout(location=1) vec4 colour_out;
in layout(location=3) vec3 normal_out;
out vec4 color;

void main()
{
    vec3 lightDirection = normalize(vec3(0.8, -0.5, 0.6));
    color = colour_out*max(0, dot(normal_out, -lightDirection));
}