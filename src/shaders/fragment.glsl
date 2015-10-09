#version 330

uniform uint max_iteration;

in vs_output
{
   vec2 pos;
} f_in;

out vec3 o_color;

void main()
{
    float x0 = f_in.pos.x * 3.5 / 2.0 - 0.75;
    float y0 = f_in.pos.y;
    float x = 0;
    float y = 0;
    uint iteration = 0u;
    while (x*x + y*y < 4 && iteration < max_iteration)
    {
        float t = x*x - y*y + x0;
        y = 2*x*y + y0;
        x = t;
        iteration++;
    }

    float c = iteration == max_iteration
        ? 0
        : float(iteration) / max_iteration;
    o_color = vec3(c, 0, c);
}
