use gl;
use gl::types::{GLuint, GLenum, GLsizei, GLfloat, GLvoid, GLint};

pub mod pixel;

pub enum TextureTarget {
    Texture1D,
    Texture2D,
    Texture3D,
    Texture1DArray,
    Texture2DArray,
    TextureRectangle,
    /// Note: cube maps aren't entirely supported quite yet. Can't use
    /// TexImage with them.
    TextureCubeMap,
    TextureBuffer,
}

impl TextureTarget {
    pub fn to_glenum(&self) -> GLenum {
        match *self {
            Texture1D => gl::TEXTURE_1D,
            Texture2D => gl::TEXTURE_2D,
            Texture3D => gl::TEXTURE_3D,
            Texture1DArray => gl::TEXTURE_1D_ARRAY,
            Texture2DArray => gl::TEXTURE_2D_ARRAY,
            TextureRectangle => gl::TEXTURE_RECTANGLE,
            TextureCubeMap => gl::TEXTURE_CUBE_MAP,
            TextureBuffer => gl::TEXTURE_BUFFER,
        }
    }
}

pub enum WrapMode {
    ClampToEdge,
    ClampToBorder,
    Repeat,
    MirroredRepeat
}

impl WrapMode {
    pub fn to_glenum(&self) -> GLenum {
        match *self {
            ClampToEdge => gl::CLAMP_TO_EDGE,
            ClampToBorder => gl::CLAMP_TO_BORDER,
            Repeat => gl::REPEAT,
            MirroredRepeat => gl::MIRRORED_REPEAT
        }
    }
}

pub enum FilterMethod {
    Nearest,
    Linear,
    NearestMipmapNearest,
    LinearMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapLinear
}

impl FilterMethod {
    pub fn to_glenum(&self) -> GLenum {
        match *self {
            Nearest => gl::NEAREST,
            Linear => gl::LINEAR,
            NearestMipmapNearest => gl::NEAREST_MIPMAP_NEAREST,
            LinearMipmapNearest => gl::LINEAR_MIPMAP_NEAREST,
            NearestMipmapLinear => gl::NEAREST_MIPMAP_LINEAR,
            LinearMipmapLinear => gl::LINEAR_MIPMAP_LINEAR,
        }
    }
}

/// ImageInfo represents the non-data parameters to glTexImage*, with the goal
/// of making the numerous arguments more readable.
pub struct ImageInfo {
    pub level: GLint,
    pub internal_format: GLint,
    pub width: Option<GLsizei>,
    pub height: Option<GLsizei>,
    pub depth: Option<GLsizei>,
    pub format: pixel::PixelFormat,
    pub ptype: pixel::PixelType
}

impl ImageInfo {
    /// Create a new ImageInfo where everything is set to a "good default",
    /// that is, RGBA with the data type being bytes.
    ///
    /// Note that this struct uses the builder pattern. Its intended usage is:
    ///
    ///     let ii = ImageInfo::new().width(4).level(3).pixel_type(pixel::BYTE);
    ///
    /// This usually reads nicer. Note that since internal formats are so
    /// numerous and complex, they do not have a typesafe wrapper. The
    /// dimensions of the image will be inferred based on whether width etc
    /// are called.
    pub fn new() -> ImageInfo {
        ImageInfo {
            level: 0,
            internal_format: gl::RGBA as GLint,
            width: None,
            height: None,
            depth: None,
            format: pixel::RGBA,
            ptype: pixel::FLOAT,
        }
    }

    /// Set the mipmap level
    pub fn level(self, level: GLint) -> ImageInfo {
        ImageInfo { level: level, ..self }
    }

    /// Set the width. 1D textures only have width.
    pub fn width(self, s: GLint) -> ImageInfo {
        ImageInfo { width: Some(s), ..self }
    }

    /// Set the height. 2D and 3D textures require this.
    pub fn height(self, s: GLint) -> ImageInfo {
        ImageInfo { height: Some(s), ..self }
    }

    /// Set the depth. Only 3D textures have depth.
    pub fn depth(self, s: GLint) -> ImageInfo {
        ImageInfo { depth: Some(s), ..self }
    }

    /// Set the pixel data format
    pub fn pixel_format(self, format: pixel::PixelFormat) -> ImageInfo {
        ImageInfo { format: format, ..self }
    }

    /// Set the pixel data type
    pub fn pixel_type(self, ptype: pixel::PixelType) -> ImageInfo {
        ImageInfo { ptype: ptype, ..self }
    }

    /// Set the internal format
    pub fn internal_format(self, ifmt: GLint) -> ImageInfo {
        ImageInfo { internal_format: ifmt, ..self }
    }
}

/// SubImageInfo represents the non-data parameters to glTexSubImage*, with
/// the goal of making the numerous arguments more readable.
pub struct SubImageInfo {
    level: GLint,
    width: Option<GLsizei>,
    height: Option<GLsizei>,
    depth: Option<GLsizei>,
    xoffset: Option<GLsizei>,
    yoffset: Option<GLsizei>,
    zoffset: Option<GLsizei>,
    format: pixel::PixelFormat,
    ptype: pixel::PixelType
}

impl SubImageInfo {
    /// Create a new SubImageInfo where everything is set to a "good default",
    /// that is, RGBA with the data type being bytes.
    ///
    /// Note that this struct uses the builder pattern. Its intended usage is:
    ///
    ///     let ii = SubImageInfo::new().width(4).xoffset(5).level(3).pixel_type(pixel::BYTE);
    ///
    /// This usually reads nicer. Note that since internal formats are so
    /// numerous and complex, they do not have a typesafe wrapper. The
    /// dimensions of the image will be inferred based on whether width etc
    /// are called.
    pub fn new() -> SubImageInfo {
        SubImageInfo {
            level: 0,
            width: None,
            height: None,
            depth: None,
            xoffset: None,
            yoffset: None,
            zoffset: None,
            format: pixel::RGBA,
            ptype: pixel::FLOAT,
        }
    }

    /// Set the mipmap level
    pub fn level(self, level: GLint) -> SubImageInfo {
        SubImageInfo { level: level, ..self }
    }

    /// Set the width. 1D textures only have width.
    pub fn width(self, s: GLint) -> SubImageInfo {
        SubImageInfo { width: Some(s), ..self }
    }

    /// Set the height. 2D and 3D textures require this.
    pub fn height(self, s: GLint) -> SubImageInfo {
        SubImageInfo { height: Some(s), ..self }
    }

    /// Set the depth. Only 3D textures have depth.
    pub fn depth(self, s: GLint) -> SubImageInfo {
        SubImageInfo { depth: Some(s), ..self }
    }

    /// Set the X offset into the texture. 1D textures only have an X offset.
    pub fn xoffset(self, s: GLint) -> SubImageInfo {
        SubImageInfo { xoffset: Some(s), ..self }
    }

    /// Set the Y offset into the texture. 2D and 3D textures require this.
    pub fn yoffset(self, s: GLint) -> SubImageInfo {
        SubImageInfo { yoffset: Some(s), ..self }
    }

    /// Set the Z offset into the texture. Only 3D textures have a Z offset.
    pub fn zoffset(self, s: GLint) -> SubImageInfo {
        SubImageInfo { zoffset: Some(s), ..self }
    }

    /// Set the pixel data format
    pub fn pixel_format(self, format: pixel::PixelFormat) -> SubImageInfo {
        SubImageInfo { format: format, ..self }
    }

    /// Set the pixel data type
    pub fn pixel_type(self, ptype: pixel::PixelType) -> SubImageInfo {
        SubImageInfo { ptype: ptype, ..self }
    }
}

/// A texture object.
pub struct Texture {
    pub name: GLuint,
    pub target: GLenum,
}

impl Texture {
    /// Create a new texture and load an image into it.  Note that even if
    /// your data isn't GL_BYTE, you can pass a *u8 anyway since the GL
    /// doesn't care about the type.
    pub fn new(target: TextureTarget, info: ImageInfo, data: *u8) -> Texture {
        let mut tex: GLuint = 0;
        unsafe { gl::GenTextures(1, &mut tex as *mut GLuint); }
        let t = Texture { name: tex, target: target.to_glenum() };
        t.bind();
        t.load_image(info, data);
        t.gen_mipmaps();
        t
    }

    /// Create a texture without binding it.
    pub fn new_raw(target: TextureTarget) -> Texture {
        let mut tex: GLuint = 0;
        unsafe { gl::GenTextures(1, &mut tex as *mut GLuint); }
        Texture { name: tex, target: target.to_glenum() }
    }

    pub fn bind(&self) {
        gl::BindTexture(self.target, self.name);
    }

    pub fn wrap(&self, w: WrapMode) {
        self.wrap_s(w);
        self.wrap_t(w);
        self.wrap_r(w);
    }

    pub fn wrap_s(&self, w: WrapMode) {
        self.bind();
        gl::TexParameteri(self.target, gl::TEXTURE_WRAP_S, w.to_glenum() as GLint);
    }

    pub fn wrap_t(&self, w: WrapMode) {
        self.bind();
        gl::TexParameteri(self.target, gl::TEXTURE_WRAP_T, w.to_glenum() as GLint);
    }

    pub fn wrap_r(&self, w: WrapMode) {
        self.bind();
        gl::TexParameteri(self.target, gl::TEXTURE_WRAP_R, w.to_glenum() as GLint);
    }

    pub fn border_color(&self, color: &[GLfloat]) {
        self.bind();
        unsafe {
            gl::TexParameterfv(self.target, gl::TEXTURE_BORDER_COLOR, color.as_ptr());
        }
    }

    pub fn filter(&self, fm: FilterMethod) {
        self.min_filter(fm);
        self.mag_filter(fm);
    }

    pub fn min_filter(&self, fm: FilterMethod) {
        self.bind();
        gl::TexParameteri(self.target, gl::TEXTURE_MIN_FILTER, fm.to_glenum() as GLint);
    }

    pub fn mag_filter(&self, fm: FilterMethod) {
        self.bind();
        gl::TexParameteri(self.target, gl::TEXTURE_MAG_FILTER, fm.to_glenum() as GLint);
    }

    /// SAFETY NOTE: You *must* call `load_data` before calling this method.
    /// Bad Things will happen otherwise.
    pub fn gen_mipmaps(&self) {
        gl::GenerateMipmap(self.target);
    }

    /// Load an image into this texture.
    pub fn load_image(&self, info: ImageInfo, data: *u8) {
        self.bind();

        let ImageInfo { level, internal_format, width, height, depth, format, ptype } = info;
        let format = format.to_glenum();
        let ptype = ptype.to_glenum();
        if depth.is_none() {
            if height.is_none() { unsafe {
                // 1D
                gl::TexImage1D(self.target, level, internal_format,
                               width.expect("1D texture needs a width!"),
                               0, format, ptype, data as *GLvoid);
            } } else { unsafe {
                // 2D
                gl::TexImage2D(self.target, level, internal_format,
                               width.expect("2D texture needs a width!"),
                               height.expect("2D texture needs a height!"),
                               0, format, ptype, data as *GLvoid);
            } }
        } else { unsafe {
            // 3D
            gl::TexImage3D(self.target, level, internal_format,
                           width.expect("3D texture needs a width!"),
                           height.expect("3D texture needs a height!"),
                           depth.expect("3D texture needs a depth!"),
                           0, format, ptype, data as *GLvoid);
        } }
    }

    /// Load an image into part of this texture.
    pub fn load_subimage(&self, info: SubImageInfo, data: *u8) {
        self.bind();

        let SubImageInfo { level, width, height, depth, xoffset, yoffset, zoffset, format, ptype } = info;
        let format = format.to_glenum();
        let ptype = ptype.to_glenum();

        if depth.is_none() && zoffset.is_none() {
            if height.is_none() && yoffset.is_none() { unsafe {
                // 1D
                gl::TexSubImage1D(self.target, level,
                                  xoffset.expect("1D texture needs an xoffset!"),
                                  width.expect("1D texture needs a width!"),
                                  format, ptype, data as *GLvoid);
            } } else { unsafe {
                // 2D
                gl::TexSubImage2D(self.target, level,
                                  xoffset.expect("2D texture needs an xoffset!"),
                                  yoffset.expect("2D texture needs a yoffset!"),
                                  width.expect("2D texture needs a width!"),
                                  height.expect("2D texture needs a height!"),
                                  format, ptype, data as *GLvoid);
            } }
        } else { unsafe {
            // 3D
            gl::TexSubImage3D(self.target, level,
                              xoffset.expect("3D texture needs an xoffset!"),
                              yoffset.expect("3D texture needs a yoffset!"),
                              zoffset.expect("3D texture needs a zoffset!"),
                              width.expect("3D texture needs a width!"),
                              height.expect("3D texture needs a height!"),
                              depth.expect("3D texture needs a height!"),
                              format, ptype, data as *GLvoid);
        } }
    }

    /// Bind this texture to texture unit `num` (GL_TEXTURE0 + num)
    pub fn activate(&self, num: GLuint) {
        gl::ActiveTexture(gl::TEXTURE0 + num);
        self.bind();
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.name); }
    }
}

