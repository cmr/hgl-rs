extern crate glfw;
extern crate native;
extern crate hgl;
extern crate gl;
extern crate libc;

use std::mem::size_of;

use glfw::Context;
use hgl::{Shader, Program, Triangles, Vbo, Vao, Texture, ImageInfo, texture};

static VERTEX_SHADER: &'static str = "
#version 140

in vec2 position;
in vec2 texcoord;

out vec2 Texcoord;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    Texcoord = texcoord;
}";

static FRAGMENT_SHADER: &'static str = "
#version 140
out vec4 out_color;
in vec2 Texcoord;
uniform sampler2D checker;

void main() {
    out_color = texture(checker, Texcoord);
}";

#[start]
fn main(argc: int, argv: *const *const u8) -> int {
    native::start(argc, argv, proc() {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(glfw::ContextVersion(3, 1));
        let (window, _events) = glfw.create_window(800, 600, "HGL", glfw::Windowed).unwrap();
        window.make_current();
        gl::load_with(|p| glfw.get_proc_address(p));

        gl::Viewport(0, 0, 800, 600);

        let vao = Vao::new();
        vao.bind();
        let program = Program::link([Shader::compile(VERTEX_SHADER, hgl::VertexShader),
                                     Shader::compile(FRAGMENT_SHADER, hgl::FragmentShader)]).unwrap();
        program.bind_frag(0, "out_color");
        program.bind();

        let vbo = Vbo::from_data([0.0f32,  0.5, 0.0, 0.0,
        0.5,    -0.5, 1.0, 0.0,
        -0.5,    -0.5, 0.5, 1.0],
        hgl::StaticDraw);

        vao.enable_attrib(&program, "position", gl::FLOAT, 2, 4*size_of::<f32>() as i32, 0);
        vao.enable_attrib(&program, "texcoord", gl::FLOAT, 2, 4*size_of::<f32>() as i32, 2*size_of::<f32>());
        vbo.bind();

        gl::Uniform1i(program.uniform("checker"), 0);

        let i = ImageInfo::new().width(2).height(2).pixel_format(texture::pixel::RGB);
        let dat = [0.0f32, 0.0, 0.0, 1.0, 1.0, 1.0,
        1.0,    1.0, 1.0, 0.0, 0.0, 0.0];

        let tex = Texture::new(texture::Texture2D, i, dat.as_slice().as_ptr() as *const u8);
        tex.wrap(texture::Repeat);
        tex.filter(texture::Linear);

        tex.activate(0);

        while !window.should_close() {
            glfw.poll_events();
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            vao.draw_array(Triangles, 0, 3);
            window.swap_buffers();
        }
    });
    0
}
