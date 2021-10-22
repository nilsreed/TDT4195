#version 430 core

in layout(location=1) vec4 colour_out;
out vec4 color;

void main()
{
    color = colour_out;
}