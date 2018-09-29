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
// along with Foobar.  If not, see <http://www.gnu.org/licenses/>.
#![macro_use]

use std::mem::size_of;
use std::ops::Drop;
use std::os::raw::c_void;
use std::ptr;

use super::gl;
use super::shader::Shader;

use nalgebra::Vector3;

macro_rules! offset_of {
    ($ty:ty, $field:ident) => {
        &(*(ptr::null() as *const $ty)).$field as *const _ as usize
    };
}

#[repr(C)]
pub struct Vertex {
    pub position: [f32; 3],
}

pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,

    pub color: Vector3<f32>,

    vao: u32,
    vbo: u32,
    ibo: u32,
}

impl Mesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Mesh {
        let mut mesh: Mesh = Default::default();

        unsafe {
            mesh.init(vertices, indices);
        }

        mesh
    }

    /// render the mesh
    pub unsafe fn draw(&self, shader: &Shader) {
        shader.set_uniform_vec(&"uColor", &self.color);

        gl::BindVertexArray(self.vao);
        gl::DrawElements(
            gl::TRIANGLES,
            self.indices.len() as i32,
            gl::UNSIGNED_INT,
            ptr::null(),
        );
        gl::BindVertexArray(0);
    }

    unsafe fn init(&mut self, vertices: Vec<Vertex>, indices: Vec<u32>) {
        self.vertices = vertices;
        self.indices = indices;

        // VAO
        gl::GenVertexArrays(1, &mut self.vao);
        gl::BindVertexArray(self.vao);

        // VBO
        gl::GenBuffers(1, &mut self.vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        let size = (self.vertices.len() * size_of::<Vertex>()) as isize;
        let data = &self.vertices[0] as *const Vertex as *const c_void;
        gl::BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        // IBO
        gl::GenBuffers(1, &mut self.ibo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo);
        let size = (self.indices.len() * size_of::<u32>()) as isize;
        let data = &self.indices[0] as *const u32 as *const c_void;
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        let size = size_of::<Vertex>() as i32;
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            size,
            offset_of!(Vertex, position) as *const c_void,
        );

        gl::BindVertexArray(0);
    }
}

impl Default for Mesh {
    fn default() -> Mesh {
        Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),

            color: Vector3::new(1., 1., 1.),

            vao: 0,
            vbo: 0,
            ibo: 0,
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.ibo);
            gl::DeleteBuffers(1, &mut self.vbo);
            gl::DeleteVertexArrays(1, &mut self.vao);
        }
    }
}
