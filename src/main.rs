#[macro_use]
extern crate glium;
extern crate cgmath;

use glium::{IndexBuffer, Program, Surface, VertexBuffer};
use glium::backend::glutin::Display;
use glium::glutin::{ContextBuilder, EventsLoop, ElementState, WindowEvent, KeyboardInput, DeviceEvent, Event, VirtualKeyCode};
use cgmath::Vector2;

mod camera;
mod plane;

#[derive(PartialEq)]
enum MainState {
    Camera,
    Plane,
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

fn main() {
    let starting_width = 2048;
    let starting_height = 2048;

    // Make the events loop
    let events_loop = EventsLoop::new();

    // Make the context Builder
    let context_builder = ContextBuilder::new();

    // Make the window builder
    let window_builder = glium::glutin::WindowBuilder::new()
        .with_dimensions(starting_width / 2, starting_height/ 2)
        .with_title(format!("mand"));

    // Now we put them together to make the display
    let display = Display::new(
        window_builder,
        context_builder,
        &events_loop
    ).unwrap();

    // We statically include the shader sources, and build the shader program
    let vertex_shader_src = include_str!("vertex_shader.vert");
    let fragment_shader_src = include_str!("fragment_shader.frag");
    let shader_program = Program::from_source(&display, 
                         &vertex_shader_src, &fragment_shader_src,
                         None).unwrap();

    // We need a quad (two triangled) to cover the enitire screen
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

    // Initiliaze the plane
    let mut plane = plane::Plane::new(100.0f32, starting_width, starting_height, &display);

    // The main state
    let mut main_state = MainState::Camera;

    'render_loop: loop {
        let mut target = display.draw();

        // Handle all the window events / user input
        let (w, h) = target.get_dimensions();
        for event in events_loop.poll_events() {

            if let Event::WindowEvent{window_id: id, event: event} = event {
                match  event {
                    WindowEvent::Closed => std::process::exit(0),
                    WindowEvent::MouseWheel{ device_id: _, delta: glium::glutin::MouseScrollDelta::PixelDelta(_, y), phase: _} => {
                        match main_state {
                            MainState::Camera => cam.handle_mouse_scroll(y),
                            MainState::Plane  => plane.handle_mouse_scroll(y),
                        };
                    },
                    WindowEvent::MouseInput{ device_id: _, state: button_state, button: button} => {
                        match main_state {
                            MainState::Camera => cam.handle_mouse_click(button_state, button),
                            MainState::Plane  => plane.handle_mouse_click(button_state, button),
                        };
                    },
                    WindowEvent::MouseMoved{device_id: _, position: (x, y)} => {
                        match main_state {
                            MainState::Camera => cam.handle_mouse_move(x, y, w, h),
                            MainState::Plane => plane.handle_mouse_move(x, y, w, h),
                        };
                    },
                    WindowEvent::KeyboardInput{ device_id: _, input: keyboard_input} => {
                        let KeyboardInput{scancode: _, state: state, virtual_keycode: code, modifiers: _} = keyboard_input;
                        match (state, code) {
                            (ElementState::Pressed, Some(VirtualKeyCode::F)) => {
                                main_state = MainState::Plane; 
                            }
                            (ElementState::Released, Some(VirtualKeyCode::F)) => {
                                main_state = MainState::Camera;
                            }
                            (ElementState::Pressed, Some(VirtualKeyCode::Escape)) => {
                                // TODO break the loop, don't exit
                                std::process::exit(0);
                            }
                        }
                    },
                    _ => (),
                }
            }
        }
        
        let uniforms = uniform! {
            // Note that window resize events don't work, so we have to poll the dimensions. 
            // https://github.com/tomaka/winit/issues/39
            camera_transform: cam.get_camera_matrix(target.get_dimensions()),
            c: {
                let pos = plane.get_pos() / 1024.0 - Vector2::new(1.0, 1.0);
                let pos_raw: [f32; 2] = pos.into();
                pos_raw
            },
        };

        // Clear the screen, draw, and swap the buffers
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(&vertex_buffer, &indices, &shader_program, &uniforms, &params).unwrap();

        if main_state == MainState::Plane {
            plane.draw(&mut target);
        }
        
        target.finish().unwrap();
    }  
}
