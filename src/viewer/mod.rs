use camera;

use cgmath::Vector2;
use glium;
use glium::{IndexBuffer, Program, Surface, VertexBuffer};
use glium::backend::glutin::Display;
use glium::glutin::{ElementState, Event, EventsLoop, KeyboardInput, MouseScrollDelta,
                    VirtualKeyCode, WindowEvent};
use plane;
use std::process;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);


#[derive(PartialEq)]
enum MainState {
    Camera,
    Plane,
}

pub struct Viewer<'a> {
    state: MainState,
    cam: camera::Camera,
    plane: plane::Plane,
    display: &'a Display,
}

impl<'a> Viewer<'a> {
    pub fn new(
        width: u32,
        height: u32,
        display: &'a Display,
    ) -> Viewer<'a> {
        // Initialize the camera
        let cam = camera::Camera::new();

        // Initiliaze the plane
        let plane = plane::Plane::new(100.0f32, width, height, display);

        // The main state
        let main_state = MainState::Camera;


        Viewer {
            cam: cam,
            plane: plane,
            state: main_state,
            display: display,
        }
    }

    pub fn run(
        mut self,
        events_loop: &mut EventsLoop,
    ) {
        // We statically include the shader sources, and build the shader program
        let vertex_shader_src = include_str!("../_shaders/vertex_shader.vert");
        let fragment_shader_src = include_str!("../_shaders/fragment_shader.frag");
        let shader_program =
            Program::from_source(self.display, vertex_shader_src, fragment_shader_src, None)
                .unwrap();

        // We need a quad (two triangled) to cover the enitire screen
        let v1 = Vertex {
            position: [-1.0, -1.0],
            tex_coords: [-1.0, -1.0],
        };
        let v2 = Vertex {
            position: [-1.0, 1.0],
            tex_coords: [-1.0, 1.0],
        };
        let v3 = Vertex {
            position: [1.0, 1.0],
            tex_coords: [1.0, 1.0],
        };
        let v4 = Vertex {
            position: [1.0, -1.0],
            tex_coords: [1.0, -1.0],
        };
        let vertex_buffer = VertexBuffer::new(self.display, &[v1, v2, v3, v4]).unwrap();
        let indices = IndexBuffer::new(
            self.display,
            glium::index::PrimitiveType::TrianglesList,
            &[0, 1, 3, 1, 2, 3u16],
        ).unwrap();

        // Drawing parameters
        let params = glium::DrawParameters { ..Default::default() };


        'render_loop: loop {
            let mut target = self.display.draw();

            // Handle all the window events / user input
            let (w, h) = target.get_dimensions();
            events_loop.poll_events(|e| self.handle_event(w, h, e));

            let uniforms =
                uniform! {
                // Note that window resize events don't work, so we have to poll the dimensions.
                // https://github.com/tomaka/winit/issues/39
                // TODO: is this still the case?
                camera_transform: self.cam.get_camera_matrix(target.get_dimensions()),
                c: {
                    let pos = self.plane.get_pos() / 1024.0 - Vector2::new(1.0, 1.0);
                    let pos_raw: [f32; 2] = pos.into();
                    pos_raw
                }
            };

            // Clear the screen, draw, and swap the buffers
            target.clear_color(0.0, 0.0, 0.0, 0.0);
            target
                .draw(
                    &vertex_buffer,
                    &indices,
                    &shader_program,
                    &uniforms,
                    &params,
                )
                .unwrap();

            if self.state == MainState::Plane {
                self.plane.draw(&mut target);
            }

            target.finish().unwrap();
        }
    }

    fn handle_event(
        &mut self,
        target_width: u32,
        target_height: u32,
        event: Event,
    ) {
        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::Closed => process::exit(0),
                WindowEvent::MouseWheel { delta: MouseScrollDelta::PixelDelta(_, y), .. } => {
                    match self.state {
                        MainState::Camera => self.cam.handle_mouse_scroll(y),
                        MainState::Plane => self.plane.handle_mouse_scroll(y),
                    };
                },
                WindowEvent::MouseInput {
                    state: button_state,
                    button,
                    ..
                } => {
                    match self.state {
                        MainState::Camera => self.cam.handle_mouse_click(button_state, button),
                        MainState::Plane => self.plane.handle_mouse_click(button_state, button),
                    };
                },
                WindowEvent::MouseMoved { position: (x, y), .. } => {
                    match self.state {
                        MainState::Camera => {
                            self.cam.handle_mouse_move(
                                x,
                                y,
                                target_width,
                                target_height,
                            )
                        },
                        MainState::Plane => {
                            self.plane.handle_mouse_move(
                                x,
                                y,
                                target_width,
                                target_height,
                            )
                        },
                    };
                },
                WindowEvent::KeyboardInput { input: keyboard_input, .. } => {
                    let KeyboardInput {
                        state,
                        virtual_keycode: code,
                        ..
                    } = keyboard_input;
                    match (state, code) {
                        (ElementState::Pressed, Some(VirtualKeyCode::F)) => {
                            self.state = MainState::Plane;
                        },
                        (ElementState::Released, Some(VirtualKeyCode::F)) => {
                            self.state = MainState::Camera;
                        },
                        (ElementState::Pressed, Some(VirtualKeyCode::Escape)) => {
                            process::exit(0);
                        },
                        _ => (),
                    }
                },
                _ => (),
            }
        };
    }
}
