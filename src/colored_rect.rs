extern crate graphics;
use graphics::*;

pub const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
pub const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
pub const BLUE:  [f32; 4] = [0.0, 0.0, 1.0, 1.0];
pub const BLACK:  [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const GREY: [f32; 4] = [0.5, 0.5, 0.5, 1.0];

pub struct ColoredRect {
    pub color: [f32; 4],
    pub rectangle: graphics::types::Rectangle,
}

impl ColoredRect {
    pub fn new() -> Self {
        ColoredRect {
            color: GREEN,
            rectangle: [0.0, 0.0, 0.0, 0.0]
        }
    }

    pub fn width_height(self, w: f64, h: f64) -> Self {
        let mut cr = self;
        cr.rectangle[2] = w;
        cr.rectangle[3] = h;
        cr
    }

    pub fn color(self, color: [f32; 4]) -> Self {
        let mut cr = self;
        cr.color = color;
        cr
    }

    pub fn position(self, x: f64, y: f64) -> Self {
        let mut cr = self;
        cr.rectangle[0] = x;
        cr.rectangle[1] = y;
        cr
    }

    pub fn next_color(&mut self) {
        self.color = match self.color {
            RED   => GREEN,
            GREEN => BLUE,
            BLUE  => RED,
            _     => [0.0, 0.0, 0.0, 0.0]
        }
    }
}
