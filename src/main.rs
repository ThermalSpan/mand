#[macro_use]
extern crate glium;
extern crate cgmath;

use cgmath::{Vector2, Matrix3, Rad};
use glium::glutin::{ElementState, MouseButton};

mod camera;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

fn main() {
    // Build out window
    use glium::DisplayBuild;
    let display = glium::glutin::WindowBuilder::new()
        .with_dimensions(1024, 1024)
        .with_title(format!("mand"))
        .build_glium()
        .unwrap();

    // We statically include the shader sources, and build the shader program
    use glium::Program;
    let vertex_shader_src = include_str!("vertex_shader.vert");
    let fragment_shader_src = include_str!("fragment_shader.frag");
    let shader_program = Program::from_source(&display, 
                         &vertex_shader_src, &fragment_shader_src,
                         None).unwrap();

    // We need a quad (two triangled) to cover the enitire screen
    use glium::{ VertexBuffer, IndexBuffer };
    let v1 = Vertex { position: [-1.0, -1.0], tex_coords: [-1.0, -1.0] };
    let v2 = Vertex { position: [-1.0,  1.0], tex_coords: [-1.0, 1.0] };
    let v3 = Vertex { position: [ 1.0,  1.0], tex_coords: [1.0, 1.0] };
    let v4 = Vertex { position: [ 1.0, -1.0], tex_coords: [1.0, -1.0] };
    let vertex_buffer = VertexBuffer::new(&display,
                        &[v1, v2, v3, v4])
                        .unwrap();
    let indices = IndexBuffer::new(&display,
                  glium::index::PrimitiveType::TrianglesList,
                  &[0, 1, 3, 1, 2, 3u16])
                  .unwrap();
    
    // Drawing parameters
    let params = glium::DrawParameters {
        .. Default::default()
    };

    // Initialize the camera
    let mut cam = camera::Camera::new();

    'render_loop: loop {
        use glium::Surface;
        let mut target = display.draw();
 
        // Handle all the window events / user input
        let (w, h) = target.get_dimensions();
        for event in display.poll_events() {
            use glium::glutin::Event;

            match event {
                Event::Closed => std::process::exit(0),
                Event::MouseWheel(glium::glutin::MouseScrollDelta::PixelDelta(_, y), _) => {
                    cam.handle_mouse_scroll(y);
                },
                Event::MouseInput(button_state, button) => {
                    cam.handle_mouse_click(button_state, button);
                },
                Event::MouseMoved(x, y) => {
                    cam.handle_mouse_move(x, y, w, h);
                },
                _ => (),
            }
        }
        
        let uniforms = uniform! {
            // Note that window resize events don't work, so we have to poll the dimensions. 
            // https://github.com/tomaka/winit/issues/39
            camera_transform: cam.get_camera_matrix(target.get_dimensions()),
        };

        // Clear the screen, draw, and swap the buffers
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(&vertex_buffer, &indices, &shader_program, &uniforms, &params).unwrap();
        target.finish().unwrap();
    }  
}
