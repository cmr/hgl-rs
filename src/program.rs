//! Dealing with Programs and Shaders

use gl;
use std::io::{File, IoResult};
use gl::types::{GLint, GLuint, GLenum, GLsizei, GLchar};

/// Shader types
pub enum ShaderType {
    VertexShader,
    FragmentShader,
}

impl ShaderType {
    /// Convert a ShaderType into its corresponding GL value
    pub fn to_glenum(&self) -> GLenum {
        match *self {
            VertexShader => gl::VERTEX_SHADER,
            FragmentShader => gl::FRAGMENT_SHADER,
        }
    }
}

pub struct Shader {
    pub name: GLuint,
    pub type_: ShaderType
}

fn get_info_log(shader: GLuint, get: unsafe fn(GLuint, GLenum, *mut GLint),
                info: unsafe fn(GLuint, GLsizei, *mut GLint, *mut GLchar),
                status: GLenum) -> Option<Vec<u8>> {
    let mut ret = gl::FALSE as GLint;
    unsafe {
        get(shader, status, &mut ret);
    }

    if ret == gl::TRUE as GLint {
        return None
    }

    let mut len = 0;
    unsafe {
        get(shader, gl::INFO_LOG_LENGTH, &mut len as *mut GLint);
    }
    if len == 0 {
        return Some(Vec::new());
    }

    // len including trailing null
    let mut s = Vec::with_capacity(len as uint - 1);

    unsafe {
        info(shader, len, &mut len as *mut GLsizei, s.as_mut_slice().as_mut_ptr() as *mut GLchar);
        s.set_len(len as uint - 1);
    }
    Some(s)
}

impl Shader {
    pub fn from_name(name: GLuint, type_: ShaderType) -> Shader {
        if cfg!(not(ndebug)) {
            if gl::IsShader(name) == gl::FALSE {
                fail!("name is not a shader!");
            }
        }
        Shader::new_raw(name, type_)
    }

    fn new_raw(id: GLuint, type_: ShaderType) -> Shader {
        Shader { name: id, type_: type_ }
    }

    /// Returns the name (id) of the shader.
    pub fn name(&self) -> GLuint {
        self.name
    }

    /// Compile a shader.
    ///
    /// Takes the shader contents as a string. On success the Shader is returned.
    /// On failure, the complete log from glGetShaderInfoLog is returned.
    pub fn compile(source: &str, type_: ShaderType) -> Result<Shader, String> {
        let gltype = type_.to_glenum();
        let shader = gl::CreateShader(gltype);

        unsafe {
            gl::ShaderSource(shader, 1 as GLsizei, &(source.as_ptr() as *const GLchar) as *const *const GLchar,
                             &(source.len() as GLint) as *const GLint);
        }
        gl::CompileShader(shader);

        match get_info_log(shader, gl::GetShaderiv, gl::GetShaderInfoLog, gl::COMPILE_STATUS) {
            Some(s) => Err(String::from_utf8(s).ok().expect("non-utf8 infolog!")),
            None    => Ok(Shader::new_raw(shader, type_))
        }
    }

    pub fn from_file(p: &str, type_: ShaderType) -> IoResult<Result<Shader, String>> {
        match File::open(&Path::new(p)).read_to_str() {
            Err(e) => Err(e),
            Ok(s) => Ok(Shader::compile(s.as_slice(), type_))
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        gl::DeleteShader(self.name);
    }
}

/// A program, which consists of multiple compiled shaders "linked" together
pub struct Program {
    pub name: GLuint
}

impl Program {
    /// Link shaders into a program
    pub fn link(shaders: &[Result<Shader, String>]) -> Result<Program, String> {
        let program = gl::CreateProgram();
        for shader in shaders.iter() {
            match shader {
                &Ok(ref shader) => {
                    // there are no relevant errors to handle here.
                    gl::AttachShader(program, shader.name);
                },
                &Err(ref e) => return Err(e.clone())
            }
        }
        gl::LinkProgram(program);

        match get_info_log(program, gl::GetProgramiv, gl::GetProgramInfoLog, gl::LINK_STATUS) {
            Some(s) => Err(String::from_utf8(s).ok().expect("non-utf8 infolog!")),
            None    => Ok(Program { name: program })
        }
    }

    pub fn bind(&self) {
        gl::UseProgram(self.name);
    }

    pub fn bind_frag(&self, color_number: GLuint, name: &str) {
        name.with_c_str(|cstr| unsafe {
            gl::BindFragDataLocation(self.name, color_number, cstr)
        });
    }

    pub fn uniform(&self, name: &str) -> GLint {
        name.with_c_str(|cstr| unsafe {
            gl::GetUniformLocation(self.name, cstr)
        })
    }

    pub fn get_name(&self) -> GLuint { self.name }
}

impl Drop for Program {
    fn drop(&mut self) {
        gl::DeleteProgram(self.name);
    }
}

