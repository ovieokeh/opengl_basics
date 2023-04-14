extern crate glfw;

use glfw::{Action, Context, Key};
use glow::HasContext;

const VERTEX_SHADER_SRC: &str = r#"
    #version 410 core
    layout (location = 0) in vec3 aPos;

    void main() {
        gl_Position = vec4(aPos, 1.0);
    }
"#;

const FRAGMENT_SHADER_SRC: &str = r#"
    #version 410 core
    out vec4 Fragcolour;

    void main() {
        Fragcolour = vec4(1.0, 0.5, 0.2, 1.0);
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

    // Compile shaders
    let vertex_shader = unsafe {
        compile_shader(&gl, glow::VERTEX_SHADER, VERTEX_SHADER_SRC)
            .expect("Failed to compile vertex shader")
    };

    let fragment_shader = unsafe {
        compile_shader(&gl, glow::FRAGMENT_SHADER, FRAGMENT_SHADER_SRC)
            .expect("Failed to compile fragment shader")
    };

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

    let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

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

    unsafe {
        // Configure vertex attribute pointers
        let stride = 3 * std::mem::size_of::<f32>() as i32;
        gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, stride, 0);
        gl.enable_vertex_attrib_array(0);
    }

    // Set up event handling
    window.set_key_polling(true);

    // Main loop
    while !window.should_close() {
        // Poll and handle events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_event(&mut window, event);
        }

        // Render the triangle
        unsafe {
            // Clear the colour buffer
            gl.clear(glow::COLOR_BUFFER_BIT);

            // Use the shader program
            gl.use_program(Some(program));

            // Bind the VAO and draw the triangle
            gl.bind_vertex_array(Some(vao));
            gl.draw_arrays(glow::TRIANGLES, 0, 3);

            // Unbind the VAO and shader program
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
