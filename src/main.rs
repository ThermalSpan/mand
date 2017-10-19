#[macro_use]
extern crate glium;
extern crate cgmath;
extern crate clap;

use glium::backend::glutin::Display;
use glium::glutin::{ContextBuilder, EventsLoop}; 

mod args_and_usage;
mod camera;
mod plane;
mod viewer;

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
    let args = args_and_usage::parse_args();

    let starting_width = args.win_width;
    let starting_height = args.win_height;

    // Make the events loop
    let mut events_loop = EventsLoop::new();

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
    
    // Construct the viewer
    let viewer = viewer::Viewer::new(
        starting_width,
        starting_height,
        &display,
    );
    
    // let it run
    viewer.run(&mut events_loop);
}
