extern crate cgmath;
extern crate glium;
extern crate num;

use self::num::complex::Complex;
use cgmath::{Matrix3, SquareMatrix, Vector2, Vector3};
use glium::glutin::{ElementState, MouseButton};

#[derive(Debug, Clone, Copy)]
enum CameraState {
    Rest,
    Pan,
    Tumble,
}

pub struct Camera {
    state: CameraState,
    center: Vector2<f32>,
    old_mouse_location: Vector2<f32>,
    pan_delta: Vector2<f32>,
    scale: f32,
    angle: Complex<f32>,
    angle_delta: Complex<f32>,
}

fn vec2_to_complex(v: Vector2<f32>) -> Complex<f32> {
    Complex::new(v.x, v.y)
}

fn complex_to_mat3(c: Complex<f32>) -> Matrix3<f32> {
    Matrix3::new(c.re, c.im, 0.0, -c.im, c.re, 0.0, 0.0, 0.0, 1.0)
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            state: CameraState::Rest,
            center: Vector2::new(0.0, 0.0),
            old_mouse_location: Vector2::new(0.0, 0.0),
            pan_delta: Vector2::new(0.0, 0.0),
            scale: 1.0,
            angle: Complex::new(1.0, 0.0),
            angle_delta: Complex::new(1.0, 0.0),
        }
    }

    pub fn handle_mouse_click(
        &mut self,
        button_state: ElementState,
        button: MouseButton,
    ) {

        self.state = match (self.state, button_state, button) {
            // Here we enter either Pan, or Tumble mode
            (CameraState::Rest, ElementState::Pressed, MouseButton::Left) => CameraState::Pan,
            (CameraState::Rest, ElementState::Pressed, MouseButton::Right) => CameraState::Tumble,

            // Here we leave either Pan, or Tumble mode
            (CameraState::Pan, ElementState::Released, MouseButton::Left) => {
                // When leaving Pan mode, add the delta to current and zero it
                self.center += self.pan_delta;
                self.pan_delta *= 0.0;
                CameraState::Rest
            },
            (CameraState::Tumble, ElementState::Released, MouseButton::Right) => {
                // When leaving tumble mode, compose the delta to current and zero it
                self.angle *= self.angle_delta;
                self.angle_delta = Complex::new(1.0, 0.0);
                CameraState::Rest
            },

            // This is a catch all for unexpected transitions
            (state, button_state, button) => {
                println!(
                    "ERROR: camera encountered unexpected transition: {:?} -> ({:?}, {:?})",
                    state,
                    button_state,
                    button
                );
                self.state
            },
        };
    }

    pub fn handle_mouse_move(
        &mut self,
        x: f64,
        y: f64,
        w: u32,
        h: u32,
    ) {
        let new_mouse_location = Vector2 {
            x: x as f32,
            y: y as f32,
        };
        let mouse_delta = self.old_mouse_location - new_mouse_location;
        match self.state {
            CameraState::Pan => {
                let camera_scale_inverse = Matrix3::from_diagonal(Vector3::new(
                    -(2.0 * self.scale) / w as f32,
                    2.0 * self.scale / h as f32,
                    1.0,
                ));

                let camera_rotate_inverse = {
                    let mut inverse = (self.angle * self.angle_delta).inv();
                    inverse = inverse.unscale(inverse.norm());
                    complex_to_mat3(inverse)
                };

                self.pan_delta = (camera_scale_inverse * camera_rotate_inverse *
                                      mouse_delta.extend(1.0))
                    .truncate();
            },
            CameraState::Tumble => {
                let origin = Vector2::new(w as f32, h as f32) / 2.0;
                let mut old = vec2_to_complex(self.old_mouse_location - origin);
                let mut new = vec2_to_complex(new_mouse_location - origin);
                old = old.unscale(old.norm());
                new = new.unscale(new.norm());

                self.angle_delta = new * old.inv();
            },
            CameraState::Rest => {
                self.old_mouse_location = Vector2 {
                    x: x as f32,
                    y: y as f32,
                };
            },
        }
    }

    pub fn handle_mouse_scroll(
        &mut self,
        delta: f32,
    ) {
        self.scale *= 1.0 + 0.001 * delta;
    }

    pub fn get_camera_matrix(
        &self,
        target_dimensions: (u32, u32),
    ) -> [[f32; 3]; 3] {
        let translate = -1.0 * (self.center + self.pan_delta);
        let camera_translation =
            Matrix3::new(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, translate.x, translate.y, 0.0);

        let camera_rotation = {
            let mut rot = self.angle * self.angle_delta;
            rot = rot.unscale(rot.norm());
            complex_to_mat3(rot)
        };

        let camera_scale = Matrix3::new(self.scale, 0.0, 0.0, 0.0, self.scale, 0.0, 0.0, 0.0, 1.0);

        let camera_perspective = {
            let aspect_ratio = target_dimensions.0 as f32 / target_dimensions.1 as f32;
            Matrix3::new(aspect_ratio, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0)
        };

        let matrix: [[f32; 3]; 3] =
            (camera_translation * camera_rotation * camera_scale * camera_perspective).into();
        matrix
    }
}
