#version 100

attribute vec3 a_position;
attribute vec3 a_color;
uniform mat4 u_mvp;
varying vec4 v_color;

void main()
{
    v_color = vec4(a_color, 1.0);
    gl_Position = u_mvp * vec4(a_position, 1.0);
}
