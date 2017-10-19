use cgmath::{InnerSpace, Matrix2, Vector2};
use glium;
use glium::{DrawParameters, IndexBuffer, Program, Surface, VertexBuffer};
use glium::backend::Facade;
use glium::glutin::{ElementState, MouseButton};
use std::f32;

#[derive(Debug, Copy, Clone)]
struct ShapeVertex {
    position: [f32; 2],
}

implement_vertex!(ShapeVertex, position);

#[derive(Clone, Copy, Debug, PartialEq)]
enum PlaneState {
    Rest,
    Drag,
}

pub struct Plane {
    state: PlaneState,
    button_pos: Vector2<f32>,
    mouse_pos: Vector2<f32>,
    mouse_offset: Vector2<f32>,
    button_rad: f32,
    selector_v_buffer: VertexBuffer<ShapeVertex>,
    selector_indices: IndexBuffer<u32>,
    program: Program,
}

impl Plane {
    pub fn new<F: Facade>(
        rad: f32,
        w: u32,
        h: u32,
        display: &F,
    ) -> Plane {
        let segments = 50;
        let vert_rad = 2.0 * rad / w as f32;

        // Build Selector Vertex Buffer
        let mut selector_vertices = Vec::new();
        for i in 0..segments {
            let theta = i as f32 * 2.0 * f32::consts::PI / segments as f32;
            selector_vertices.push(ShapeVertex {
                position: [vert_rad * theta.sin(), vert_rad * theta.cos()],
            });
        }
        selector_vertices.push(ShapeVertex { position: [0.0, 0.0] });
        let selector_v_buffer = VertexBuffer::new(display, &selector_vertices).unwrap();

        // Build selector indices
        let mut selector_index_vec = Vec::new();
        for i in 0..segments {
            selector_index_vec.push(i);
            selector_index_vec.push((i + 1) % segments);
            selector_index_vec.push(segments);
        }
        let selector_indices = IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &selector_index_vec,
        ).unwrap();

        // Build the shader program
        let vertex_shader_src = include_str!("shape_shader.vert");
        let fragment_shader_src = include_str!("shape_shader.frag");
        let shader_program =
            Program::from_source(display, vertex_shader_src, fragment_shader_src, None).unwrap();

        Plane {
            state: PlaneState::Rest,
            button_pos: Vector2::new(w as f32 / 2.0, h as f32 / 2.0),
            mouse_pos: Vector2::new(0.0, 0.0),
            mouse_offset: Vector2::new(0.0, 0.0),
            button_rad: rad,
            selector_v_buffer: selector_v_buffer,
            selector_indices: selector_indices,
            program: shader_program,
        }
    }

    pub fn draw<S: Surface>(
        &self,
        target: &mut S,
    ) {
        // Translate the button position
        let (w, h) = target.get_dimensions();
        // let origin = 0.5 * Vector2::new(w as f32, h as f32);
        let button_offset = self.button_pos - Vector2::new(w as f32 / 2.0, h as f32 / 2.0);

        // Scale the button offset:
        //  - Source is width and height
        //  - target is (-1, 1), (1, -1)
        let aspect_ratio = w as f32 / h as f32;
        let scale_x = aspect_ratio * 2.0 / w as f32;
        let scale_y = 2.0 / h as f32;
        let transform = Matrix2::new(scale_x, 0.0, 0.0, scale_y);
        let button_offset_scaled = transform * button_offset;
        let button_offset_raw: [f32; 2] = button_offset_scaled.into();

        // Find the color
        let color = if self.state == PlaneState::Drag {
            [1.0, 0.0, 0.0, 1.0f32]
        } else {
            [0.0, 1.0, 0.0, 1.0f32]
        };

        // Build the uniforms
        let uniforms =
            uniform! {
            button_offset: button_offset_raw,
            in_color: color,
        };

        // Build the draw parameters
        let draw_parameters = DrawParameters { ..Default::default() };

        // Make the draw call
        target
            .draw(
                &self.selector_v_buffer,
                &self.selector_indices,
                &self.program,
                &uniforms,
                &draw_parameters,
            )
            .unwrap();
    }

    pub fn handle_mouse_click(
        &mut self,
        button_state: ElementState,
        button: MouseButton,
    ) {

        self.state = match (self.state, button_state, button) {
            (PlaneState::Rest, ElementState::Pressed, MouseButton::Left) => {
                self.mouse_offset = self.button_pos - self.mouse_pos;
                println!(
                    "mouse_offset: {:?}, button_pos: {:?}, mouse_pos: {:?}",
                    self.mouse_offset,
                    self.button_pos,
                    self.mouse_pos
                );
                if self.mouse_offset.magnitude2() < self.button_rad.powi(2) {
                    PlaneState::Drag
                } else {
                    PlaneState::Rest
                }
            },
            (PlaneState::Drag, ElementState::Released, MouseButton::Left) => PlaneState::Rest,
            (state, button_state, button) => {
                println!(
                    "ERROR: unexpected transiton for plane, {:?} -> ({:?}, {:?})",
                    state,
                    button_state,
                    button
                );
                self.state
            },
        }
    }

    pub fn handle_mouse_scroll(
        &mut self,
        _y: f32,
    ) {


    }

    pub fn handle_mouse_move(
        &mut self,
        x: f64,
        y: f64,
        _w: u32,
        h: u32,
    ) {
        self.mouse_pos = Vector2::new(x as f32, h as f32 - y as f32);

        if self.state == PlaneState::Drag {
            self.button_pos = self.mouse_pos + self.mouse_offset
        }
    }

    pub fn get_pos(&self) -> Vector2<f32> {
        self.button_pos
    }
}
