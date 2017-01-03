#version 400

in vec2 tex;

uniform mat3 camera_transform;

out vec3 color;

void main() {
   int iter_max = 100;
    vec2 c = vec2(camera_transform * vec3(tex, 1.0));

    vec2 z = vec2(0.0, 0.0);
    int iteration = 0;
    while (iteration < iter_max) {
        float x = z.x * z.x - z.y * z.y;
        float y = 2.0 * z.x * z.y;
        z = vec2(x + c.x, y + c.y);
        if (z.x * z.x + z.y + z.y > 4.0) {
            break;
        }
        iteration++;
    }

    float ratio = float(iteration) / float(iter_max);
    color = vec3(ratio, ratio, ratio);
}

