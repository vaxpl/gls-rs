#version 100

attribute vec3 a_position;
attribute vec2 a_texcoord;
uniform mat4 u_mvp;
varying vec2 v_uv;

void main()
{
    v_uv = a_texcoord;
    gl_Position = u_mvp * vec4(a_position, 1.0);
}
