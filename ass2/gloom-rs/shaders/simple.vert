#version 430 core

in layout(location=0) vec3 position;
in layout(location=1) vec4 colour_in;

out layout(location=1) vec4 colour_out;


void main()
{
    colour_out = colour_in;
    gl_Position = vec4(position, 1.0f);
}