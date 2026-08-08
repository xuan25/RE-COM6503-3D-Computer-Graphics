use gl;
use glfw::Context;

fn main() {
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw.create_window(
            300, 
            300, 
            "Hello this is window", 
            glfw::WindowMode::Windowed
        )
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);
    
    // Enable V-Sync
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
    
    unsafe {
        // Initialize OpenGL
        gl::load_with(|f_name| {
            glfw.get_proc_address_raw(f_name)
                .map_or(std::ptr::null(), |proc| proc as *const () as *const std::ffi::c_void)
        });

        // Set the clear color for the window
        gl::ClearColor(0.3, 0.3, 0.3, 1.0);

        // Create a Vertex Array Object (VAO)
        let mut vao = 0;
        gl::GenVertexArrays(1, &mut vao);
        assert_ne!(vao, 0);

        // Create a Vertex Buffer Object (VBO)
        let mut vbo = 0;
        gl::GenBuffers(1, &mut vbo);
        assert_ne!(vbo, 0);

        // Bind the VBO to the GL_ARRAY_BUFFER target
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        // Define the vertex data for a triangle
        type Vertex = [f32; 3];
        const VERTICES: [Vertex; 3] =
            [[-0.5, -0.5, 0.0], [0.5, -0.5, 0.0], [0.0, 0.5, 0.0]];
        
        // Upload the vertex data to the GPU
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_val(&VERTICES) as isize,
            VERTICES.as_ptr().cast(),
            gl::STATIC_DRAW,
        );
        
        // Bind the VAO to store the vertex attribute configuration
        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            size_of::<Vertex>().try_into().unwrap(),
            0 as *const _,
        );
        gl::EnableVertexAttribArray(0);

        // Create a Vertex Shader
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        assert_ne!(vertex_shader, 0);
        
        // Define the source code for the vertex shader
        const VERT_SHADER: &str = r#"
            #version 330 core
            layout (location = 0) in vec3 pos;
            void main() {
                gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
            }
        "#;

        // Set the source code for the vertex shader
        gl::ShaderSource(
            vertex_shader,
            1,
            &(VERT_SHADER.as_bytes().as_ptr().cast()),
            &(VERT_SHADER.len().try_into().unwrap()),
        );

        // Compile the vertex shader
        gl::CompileShader(vertex_shader);

        // Check for compilation errors
        let mut success = 0;
        gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);

        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetShaderInfoLog(
            vertex_shader,
            1024,
            &mut log_len,
            v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
        }

        // Create a Fragment Shader
        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        assert_ne!(fragment_shader, 0);
        
        // Define the source code for the fragment shader
        const FRAG_SHADER: &str = r#"
            #version 330 core
            out vec4 final_color;

            void main() {
                final_color = vec4(1.0, 0.5, 0.2, 1.0);
            }
        "#;

        // Set the source code for the fragment shader
        gl::ShaderSource(
            fragment_shader,
            1,
            &(FRAG_SHADER.as_bytes().as_ptr().cast()),
            &(FRAG_SHADER.len().try_into().unwrap()),
        );

        // Compile the fragment shader
        gl::CompileShader(fragment_shader);

        // Check for compilation errors
        let mut success = 0;
        gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetShaderInfoLog(
            fragment_shader,
            1024,
            &mut log_len,
            v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
        }

        // Create a Shader Program and link the vertex and fragment shaders
        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);

        // Check for linking errors
        let mut success = 0;
        gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success == 0 {
            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetProgramInfoLog(
            shader_program,
            1024,
            &mut log_len,
            v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
        }

        // Delete the shaders as they are no longer needed after linking
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }

    // Loop until the user closes the window
    while !window.should_close() {
        // Present the newly cleared back buffer.
        window.swap_buffers();

        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    window.set_should_close(true)
                },
                _ => {},
            }
        }
        
        unsafe {
            // Clear the back buffer before presenting it.
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Use the shader program for rendering
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}
