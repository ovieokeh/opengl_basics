extern crate glfw;

use glfw::{Action, Context, Key};
use glow::HasContext;

const VERTEX_SHADER_SRC: &str = r#"
    #version 410 core
    layout (location = 0) in vec3 aPos;
    layout (location = 1) in vec2 aTexCoord;

    out vec2 TexCoord;

    void main() {
        gl_Position = vec4(aPos, 1.0);
        TexCoord = aTexCoord;
    }
"#;

const FRAGMENT_SHADER_SRC: &str = r#"
    #version 410 core
    in vec2 TexCoord;
    out vec4 Fragcolour;

    uniform sampler2D texture1;

    void main() {
        Fragcolour = texture(texture1, TexCoord);
    }
"#;

unsafe fn compile_shader(
    gl: &glow::Context,
    shader_type: u32,
    source: &str,
) -> Result<glow::Shader, String> {
    let shader = gl.create_shader(shader_type).unwrap();
    gl.shader_source(shader, source);
    gl.compile_shader(shader);

    if gl.get_shader_compile_status(shader) {
        Ok(shader)
    } else {
        Err(gl.get_shader_info_log(shader))
    }
}

use std::path::Path;

unsafe fn create_texture(gl: &glow::Context, path: &Path) -> glow::Texture {
    let img = image::open(path)
        .expect("Failed to load texture")
        .flipv()
        .into_rgba8();
    let (width, height) = img.dimensions();
    let data = img.into_raw();

    let texture = gl.create_texture().unwrap();
    gl.bind_texture(glow::TEXTURE_2D, Some(texture));
    gl.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_WRAP_S,
        glow::CLAMP_TO_EDGE as i32,
    );
    gl.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_WRAP_T,
        glow::CLAMP_TO_EDGE as i32,
    );
    gl.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_MIN_FILTER,
        glow::LINEAR as i32,
    );
    gl.tex_parameter_i32(
        glow::TEXTURE_2D,
        glow::TEXTURE_MAG_FILTER,
        glow::LINEAR as i32,
    );
    gl.tex_image_2d(
        glow::TEXTURE_2D,
        0,
        glow::RGBA as i32,
        width as i32,
        height as i32,
        0,
        glow::RGBA,
        glow::UNSIGNED_BYTE,
        Some(&data),
    );

    texture
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Set GLFW window hints
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // Create a window with default settings
    let (mut window, events) = glfw
        .create_window(
            800,
            600,
            "OpenGL Basics in Rust",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();

    // Load OpenGL functions
    let gl =
        unsafe { glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _) };

    unsafe {
        gl.clear_color(0.1, 0.2, 0.3, 1.0);
    }

    let vertex_shader = unsafe {
        compile_shader(&gl, glow::VERTEX_SHADER, VERTEX_SHADER_SRC)
            .expect("Failed to compile vertex shader")
    };
    let fragment_shader = unsafe {
        compile_shader(&gl, glow::FRAGMENT_SHADER, FRAGMENT_SHADER_SRC)
            .expect("Failed to compile fragment shader")
    };

    let vertices: [f32; 15] = [
        -0.5, -0.5, 0.0, 0.0, 0.0, 0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 0.5, 0.0, 0.5, 1.0,
    ];

    let program = unsafe {
        let program = gl
            .create_program()
            .expect("Failed to create shader program");
        gl.attach_shader(program, vertex_shader);
        gl.attach_shader(program, fragment_shader);
        gl.link_program(program);

        if gl.get_program_link_status(program) {
            Ok(program)
        } else {
            Err(gl.get_program_info_log(program))
        }
    }
    .expect("Failed to link shader program");

    unsafe {
        gl.delete_shader(vertex_shader);
        gl.delete_shader(fragment_shader);
    }

    // Create and bind VAO
    let vao = unsafe { gl.create_vertex_array().unwrap() };
    unsafe { gl.bind_vertex_array(Some(vao)) };

    // Create and bind VBO
    let vbo = unsafe { gl.create_buffer().unwrap() };
    unsafe {
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(
            glow::ARRAY_BUFFER,
            &vertices.align_to::<u8>().1,
            glow::STATIC_DRAW,
        );
    }

    // Configure vertex attribute pointers
    unsafe {
        let stride = 5 * std::mem::size_of::<f32>() as i32;
        gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, stride, 0);
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(
            1,
            2,
            glow::FLOAT,
            false,
            stride,
            (3 * std::mem::size_of::<f32>()) as i32,
        );
        gl.enable_vertex_attrib_array(1);
    }

    let texture = unsafe { create_texture(&gl, Path::new("texture.png")) };

    // Set up event handling
    window.set_key_polling(true);

    // Main loop
    while !window.should_close() {
        // Poll and handle events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_event(&mut window, event);
        }

        unsafe {
            // Clear the colour buffer
            gl.clear(glow::COLOR_BUFFER_BIT);

            // Use the shader program
            gl.use_program(Some(program));

            // Bind the VAO and draw the triangle

            // Bind the texture
            gl.active_texture(glow::TEXTURE0);
            gl.bind_texture(glow::TEXTURE_2D, Some(texture));
            gl.bind_vertex_array(Some(vao));
            gl.draw_arrays(glow::TRIANGLES, 0, 3);

            // Unbind VAO and VBO
            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            gl.bind_vertex_array(None);
            gl.use_program(None);
        }
        // Swap buffers and poll events
        window.swap_buffers();
    }
}

fn handle_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        _ => {}
    }
}
