#version 330

in vs_output
{
   vec3 color;
} f_in;

out vec3 o_color;

void main()
{
   o_color = f_in.color;
}
