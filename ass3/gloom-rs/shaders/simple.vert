#version 430 core

in layout(location=0) vec3 position;
in layout(location=1) vec4 colour_in;
in layout(location=3) vec3 normal_in;
uniform layout(location=2) mat4 M_Mod;
uniform layout(location=4) mat4 M_WP;

out layout(location=1) vec4 colour_out;
out layout(location=3) vec3 normal_out;


void main()
{

    vec4 hom_pos = vec4(position.x, position.y, position.z, 1);
    
    colour_out = colour_in;
    normal_out = normalize(mat3(M_Mod)*normal_in);
    gl_Position = M_WP*M_Mod*hom_pos;
}