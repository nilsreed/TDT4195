#version 430 core

in layout(location=1) vec4 colour_out;
in layout(location=2) vec3 normal_out;
out vec4 color;

void main()
{
    color = vec4(normal_out.x, normal_out.y, normal_out.z, colour_out.a);
}