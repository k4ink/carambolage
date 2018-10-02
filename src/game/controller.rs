// This file is part of Carambolage.

// Carambolage is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Carambolage is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Carambolage.  If not, see <http://www.gnu.org/licenses/>.
use super::glfw::{Action, Key, Window};

use nalgebra::{clamp, zero, Vector2};
use time::Duration;

// Prototype controller emulation for keyboard users.
#[derive(Copy, Clone, Debug)]
pub struct Controller {
    // Internal settings and flags.
    is_smooth: bool,
    is_w: bool,
    is_a: bool,
    is_s: bool,
    is_d: bool,
    axis_goal: Vector2<f32>,

    // Buttons and input axis that can be used in the game.
    axis: Vector2<f32>,
}

// DO NOT CHANGE WASD to other keys please. Setting your controls to e.g.
// arrow keys will come later. Thanks in advance, K4ugummi.
impl Controller {
    pub fn new(smooth: bool) -> Controller {
        Controller {
            is_smooth: smooth,
            is_w: false,
            is_a: false,
            is_s: false,
            is_d: false,
            axis_goal: zero(),
            axis: zero(),
        }
    }

    pub fn process_input(&mut self, window: &Window) {
        if window.get_key(Key::W) == Action::Press && !self.is_w {
            self.set_y_axis(1.);
            self.is_w = true;
        }
        if window.get_key(Key::W) == Action::Release && self.is_w {
            self.set_y_axis(0.);
            self.is_w = false;
        }
        if window.get_key(Key::S) == Action::Press && !self.is_s {
            self.set_y_axis(-1.);
            self.is_s = true;
        }
        if window.get_key(Key::S) == Action::Release && self.is_s {
            self.set_y_axis(0.);
            self.is_s = false;
        }
        if window.get_key(Key::A) == Action::Press && !self.is_a {
            self.set_x_axis(-1.);
            self.is_a = true;
        }
        if window.get_key(Key::A) == Action::Release && self.is_a {
            self.set_x_axis(0.);
            self.is_a = false;
        }
        if window.get_key(Key::D) == Action::Press && !self.is_d {
            self.set_x_axis(1.);
            self.is_d = true;
        }
        if window.get_key(Key::D) == Action::Release && self.is_d {
            self.set_x_axis(0.);
            self.is_d = false;
        }
    }

    pub fn run(&mut self, delta_time: Duration) {
        if self.is_smooth {
            let dt = delta_time.num_milliseconds() as f32 / 1_000.;
            self.axis =
                Vector2::lerp(&self.axis, &self.axis_goal, 0.5 * dt * 10.);
            self.axis[0] = (self.axis[0] * 10_000.).trunc() / 10_000.;
            self.axis[1] = (self.axis[1] * 10_000.).trunc() / 10_000.;
        } else {
            self.axis = self.axis_goal;
        }
    }

    pub fn get_x_axis(&self) -> f32 {
        self.axis[0]
    }

    pub fn get_y_axis(&self) -> f32 {
        self.axis[1]
    }

    fn set_x_axis(&mut self, value: f32) {
        self.axis_goal[0] = value;
    }

    fn set_y_axis(&mut self, value: f32) {
        self.axis_goal[1] = value;
    }
}

trait Lerp {
    fn lerp(a: &Self, b: &Self, factor: f32) -> Self;
}

impl Lerp for Vector2<f32> {
    fn lerp(a: &Self, b: &Self, factor: f32) -> Self {
        let f = clamp(factor, 0., 1.);
        a + (b - a) * f
    }
}

impl Lerp for f32 {
    fn lerp(a: &Self, b: &Self, factor: f32) -> Self {
        let f = clamp(factor, 0., 1.);
        a + (b - a) * f
    }
}