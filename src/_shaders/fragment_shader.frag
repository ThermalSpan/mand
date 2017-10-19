#version 400

in vec2 tex;

uniform mat3 camera_transform;

out vec3 color;

void main() {
   int iter_max = 100;
    vec2 c = vec2(camera_transform * vec3(tex, 1.0));

    vec2 f_c = vec2(0.0, 0.0);
    int iteration = 0;
    while (iteration < iter_max) {
        float x = f_c.x * f_c.x - f_c.y * f_c.y;
        float y = 2.0 * f_c.x * f_c.y;
        f_c = vec2(x + c.x, y + c.y);
        if (f_c.x * f_c.x + f_c.y + f_c.y > 4.0) {
            break;
        }
        iteration++;
    }

    float ratio = float(iteration) / float(iter_max);
    color = vec3(ratio, ratio, ratio);
}

