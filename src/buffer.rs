use gl;
use std;
use std::libc::c_void;
use gl::types::{GLuint, GLenum, GLsizeiptr};

/// A vertex buffer object
pub struct Vbo {
    name: GLuint
}

impl Drop for Vbo {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.name); }
    }
}

/// Frequency with which the vbo is expected to be updated
pub enum VboUsage {
    /// Updated once, drawn many times
    StaticDraw,
    /// Updated many times, drawn many times
    DynamicDraw,
    /// Updated once, drawn once
    StreamDraw
    // TODO: add Read and Copy variants
}

impl VboUsage {
    pub fn to_glenum(&self) -> GLenum {
        match *self {
            StaticDraw  => gl::STATIC_DRAW,
            DynamicDraw => gl::DYNAMIC_DRAW,
            StreamDraw  => gl::STREAM_DRAW,
        }
    }
}

impl Vbo {
    /// Generate a new VBO, without binding it.
    pub fn new() -> Vbo {
        let mut vbo: GLuint = 0;
        unsafe { gl::GenBuffers(1, &mut vbo as *mut GLuint); }
        Vbo { name: vbo }
    }

    /// Generate a new VBO and upload `data` to it.
    pub fn from_data<T>(data: &[T], usage: VboUsage) -> Vbo {
        let vbo = Vbo::new();
        vbo.bind();
        vbo.load_data(data, usage);
        vbo
    }

    /// Load data into this vbo.
    pub fn load_data<T>(&self, data: &[T], usage: VboUsage) {
        self.bind();
        unsafe {
            gl::BufferData(gl::ARRAY_BUFFER,
                           (data.len() * std::mem::size_of::<T>()) as GLsizeiptr,
                           data.as_ptr() as *c_void, usage.to_glenum());
        }
    }

    pub fn bind(&self) {
        gl::BindBuffer(gl::ARRAY_BUFFER, self.name);
    }
}

/// An Element Buffer Object, aka GL_ELEMENT_ARRAY_BUFFER.
pub struct Ebo {
    name: GLuint
}

impl Ebo {
    /// Create an EBO from a slice of indices
    pub fn from_indices(indices: &[GLuint]) -> Ebo {
        let mut ebo = 0;
        unsafe { gl::GenBuffers(1, &mut ebo); }
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        unsafe { gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                                (indices.len() * std::mem::size_of::<GLuint>()) as GLsizeiptr,
                                indices.as_ptr() as *c_void, gl::STATIC_DRAW);
        }
        Ebo { name: ebo }
    }

    pub fn bind(&self) {
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.name);
    }
}
