#version 430 core

in vec3 position;

void main()
{
    mat3x3 flipper_matrix = mat3(-1.0, 0, 0,
                                 0, -1.0, 0,
                                 0, 0, 1.0);

    vec3 new_pos = flipper_matrix*position;
    gl_Position = vec4(new_pos, 1.0f);
}