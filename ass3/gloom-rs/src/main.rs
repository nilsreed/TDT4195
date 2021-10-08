extern crate nalgebra_glm as glm;
use std::{ mem, ptr, os::raw::c_void };
use std::thread;
use std::sync::{Mutex, Arc, RwLock};

mod shader;
mod util;
mod mesh;
mod scene_graph;
use scene_graph::SceneNode;
mod toolbox;

use glutin::event::{Event, WindowEvent, DeviceEvent, KeyboardInput, ElementState::{Pressed, Released}, VirtualKeyCode::{self, *}};
use glutin::event_loop::ControlFlow;

const SCREEN_W: u32 = 800;
const SCREEN_H: u32 = 600;

// == // Helper functions to make interacting with OpenGL a little bit prettier. You *WILL* need these! // == //
// The names should be pretty self explanatory
fn byte_size_of_array<T>(val: &[T]) -> isize {
    std::mem::size_of_val(&val[..]) as isize
}

// Get the OpenGL-compatible pointer to an arbitrary array of numbers
fn pointer_to_array<T>(val: &[T]) -> *const c_void {
    &val[0] as *const T as *const c_void
}

// Get the size of the given type in bytes
fn size_of<T>() -> i32 {
    mem::size_of::<T>() as i32
}

// Get an offset in bytes for n units of type T
fn offset<T>(n: u32) -> *const c_void {
    (n * mem::size_of::<T>() as u32) as *const T as *const c_void
}

// Get a null pointer (equivalent to an offset of 0)
// ptr::null()



// == // Modify and complete the function below for the first task
unsafe fn set_up_VAO(vertices: &Vec<f32>, colour: &Vec<f32>, indices: &Vec<u32>, normals: &Vec<f32>) -> u32 {
    let mut array_ID: u32 = 0;
    let number_of_VAOs = 1;

    /* Vertex Array Object */
    gl::GenVertexArrays(number_of_VAOs, &mut array_ID as *mut u32);
    gl::BindVertexArray(array_ID);

    /* Vertex Buffer Object for vertices */
    let mut vertex_buffer_ID: u32 = 0;
    let number_of_VBOs = 1;

    gl::GenBuffers(number_of_VBOs, &mut vertex_buffer_ID as *mut u32);
    gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_ID);

    gl::BufferData(gl::ARRAY_BUFFER, byte_size_of_array(vertices), pointer_to_array(vertices), gl::STATIC_DRAW);

    let vertex_attrib_ptr_idx = 0;
    let components_per_vertex = 3;
    let stride: i32 = 0;         //Set to 0 because input only contains vertices, and thus OpenGL can figure it out by itself
    let first_data = ptr::null();
    gl::VertexAttribPointer(vertex_attrib_ptr_idx, components_per_vertex, 
                            gl::FLOAT, gl::FALSE, stride, first_data);

    gl::EnableVertexAttribArray(vertex_attrib_ptr_idx);

    /* Vertex Buffer Object for colours */

    let mut colour_buffer_ID: u32 = 0;
    let number_of_CVBOs = 1;

    gl::GenBuffers(number_of_CVBOs, &mut colour_buffer_ID as *mut u32);
    gl::BindBuffer(gl::ARRAY_BUFFER, colour_buffer_ID);

    gl::BufferData(gl::ARRAY_BUFFER, byte_size_of_array(colour), pointer_to_array(colour), gl::STATIC_DRAW);

    let colour_attrib_ptr_idx = 1;
    let components_per_colour = 4;

    gl::VertexAttribPointer(colour_attrib_ptr_idx, components_per_colour,
                            gl::FLOAT, gl::FALSE, stride, first_data);
    
    gl::EnableVertexAttribArray(colour_attrib_ptr_idx);

    /* Vertex Buffer Object for normals */

    let mut normal_buffer_ID: u32 = 0;
    let number_of_NVBOs = 1;

    gl::GenBuffers(number_of_NVBOs, &mut normal_buffer_ID as *mut u32);
    gl::BindBuffer(gl::ARRAY_BUFFER, normal_buffer_ID);

    gl::BufferData(gl::ARRAY_BUFFER, byte_size_of_array(normals), pointer_to_array(normals), gl::STATIC_DRAW);

    let normal_attrib_ptr_idx = 3;
    let components_per_normal = 3;

    gl::VertexAttribPointer(normal_attrib_ptr_idx, components_per_normal,
                            gl::FLOAT, gl::FALSE, stride, first_data);
    
    gl::EnableVertexAttribArray(normal_attrib_ptr_idx);

    /* Vertex Buffer Object for indices */
    let mut idx_buffer_ID: u32 = 0;
    let number_of_IVBOs = 1;

    gl::GenBuffers(number_of_IVBOs, &mut idx_buffer_ID as *mut u32);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, idx_buffer_ID);

    gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, byte_size_of_array(indices), pointer_to_array(indices), gl::STATIC_DRAW);

    return array_ID;
}

unsafe fn draw_scene(node: &scene_graph::SceneNode, view_projection_matrix: &glm::Mat4){
    if node.index_count >= 0 {
        gl::UniformMatrix4fv(2, 1, 0, node.current_transformation_matrix.as_ptr());
        gl::UniformMatrix4fv(4, 1, 0, view_projection_matrix.as_ptr());
        gl::BindVertexArray(node.vao_id);
        gl::DrawElements(gl::TRIANGLES, node.index_count, gl::UNSIGNED_INT, ptr::null());
    }
    
    for &child in &node.children{
        draw_scene(&*child, view_projection_matrix);
    }
}

unsafe fn update_node_transformations(node: &mut scene_graph::SceneNode, transformation_so_far: &glm::Mat4){
    //Move to reference point
    let mut trans = glm::translation(&glm::vec3(-node.reference_point.x, -node.reference_point.y, -node.reference_point.z));
    //Apply rotations
    trans = glm::rotation(node.rotation.z, &glm::vec3(0.0, 0.0, 1.0))*trans;
    trans = glm::rotation(node.rotation.y, &glm::vec3(0.0, 1.0, 0.0))*trans;
    trans = glm::rotation(node.rotation.x, &glm::vec3(1.0, 0.0, 0.0))*trans;
    //Move back from reference point
    trans = glm::translation(&glm::vec3(node.reference_point.x, node.reference_point.y, node.reference_point.z))*trans;
    //Apply parent transformation
    trans = (*transformation_so_far)*trans;
    //Apply translation
    trans = glm::translation(&glm::vec3(node.position.x, node.position.y, node.position.z))*trans;

    //Update the current transformation 
    node.current_transformation_matrix = trans;

    for &child in &node.children{
        update_node_transformations(&mut *child, &node.current_transformation_matrix);
    }
}

fn main() {
    // Set up the necessary objects to deal with windows and event handling
    let el = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Gloom-rs")
        .with_resizable(false)
        .with_inner_size(glutin::dpi::LogicalSize::new(SCREEN_W, SCREEN_H));
    let cb = glutin::ContextBuilder::new()
        .with_vsync(true);
    let windowed_context = cb.build_windowed(wb, &el).unwrap();
    // Uncomment these if you want to use the mouse for controls, but want it to be confined to the screen and/or invisible.
    // windowed_context.window().set_cursor_grab(true).expect("failed to grab cursor");
    // windowed_context.window().set_cursor_visible(false);

    // Set up a shared vector for keeping track of currently pressed keys
    let arc_pressed_keys = Arc::new(Mutex::new(Vec::<VirtualKeyCode>::with_capacity(10)));
    // Make a reference of this vector to send to the render thread
    let pressed_keys = Arc::clone(&arc_pressed_keys);

    // Set up shared tuple for tracking mouse movement between frames
    let arc_mouse_delta = Arc::new(Mutex::new((0f32, 0f32)));
    // Make a reference of this tuple to send to the render thread
    let mouse_delta = Arc::clone(&arc_mouse_delta);

    // Spawn a separate thread for rendering, so event handling doesn't block rendering
    let render_thread = thread::spawn(move || {
        // Acquire the OpenGL Context and load the function pointers. This has to be done inside of the rendering thread, because
        // an active OpenGL context cannot safely traverse a thread boundary
        let context = unsafe {
            let c = windowed_context.make_current().unwrap();
            gl::load_with(|symbol| c.get_proc_address(symbol) as *const _);
            c
        };

        // Set up openGL
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
            gl::Enable(gl::CULL_FACE);
            gl::Disable(gl::MULTISAMPLE);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(Some(util::debug_callback), ptr::null());

            // Print some diagnostics
            println!("{}: {}", util::get_gl_string(gl::VENDOR), util::get_gl_string(gl::RENDERER));
            println!("OpenGL\t: {}", util::get_gl_string(gl::VERSION));
            println!("GLSL\t: {}", util::get_gl_string(gl::SHADING_LANGUAGE_VERSION));
        }

        // == // Set up your VAO here
        let (mut terrain_vao, mut body_vao, mut door_vao, mut main_rotor_vao, mut tail_rotor_vao);
        
        let terrain_path = "resources/lunarsurface.obj";
        let surface = mesh::Terrain::load(&terrain_path);

        let helicopter_path = "resources/helicopter.obj";
        let helicopter = mesh::Helicopter::load(&helicopter_path);

        unsafe {terrain_vao = set_up_VAO(&surface.vertices, &surface.colors, &surface.indices, &surface.normals);}

        let mut root = SceneNode::new();
        let mut terrain_node = SceneNode::from_vao(terrain_vao, surface.index_count);

        root.add_child(&terrain_node);

        for i in 0..=4{
            unsafe {
                body_vao = set_up_VAO(&helicopter.body.vertices, &helicopter.body.colors, &helicopter.body.indices, &helicopter.body.normals);
                door_vao = set_up_VAO(&helicopter.door.vertices, &helicopter.door.colors, &helicopter.door.indices, &helicopter.door.normals);
                main_rotor_vao = set_up_VAO(&helicopter.main_rotor.vertices, &helicopter.main_rotor.colors, &helicopter.main_rotor.indices, &helicopter.main_rotor.normals);
                tail_rotor_vao = set_up_VAO(&helicopter.tail_rotor.vertices, &helicopter.tail_rotor.colors, &helicopter.tail_rotor.indices, &helicopter.tail_rotor.normals);
            
                (*root.children[0]).add_child(&SceneNode::from_vao(body_vao, helicopter.body.index_count));
                (*(*root.children[0]).children[i]).add_child(&SceneNode::from_vao(door_vao, helicopter.door.index_count));
                (*(*root.children[0]).children[i]).add_child(&SceneNode::from_vao(main_rotor_vao, helicopter.main_rotor.index_count));
                (*(*root.children[0]).children[i]).add_child(&SceneNode::from_vao(tail_rotor_vao, helicopter.tail_rotor.index_count));
                (*(*(*root.children[0]).children[i]).children[1]).reference_point = glm::vec3(0.0, 0.0, 0.0);
                (*(*(*root.children[0]).children[i]).children[2]).reference_point = glm::vec3(0.35, 2.3, 10.4);
            }
        }
        

        /* Create scene nodes and combine them into a scene graph */
        /*let mut root = SceneNode::new();
        let mut terrain_node = SceneNode::from_vao(terrain_vao, surface.index_count);
        let mut body_node = SceneNode::from_vao(body_vao, helicopter.body.index_count);
        let mut door_node = SceneNode::from_vao(door_vao, helicopter.door.index_count);
        let mut main_rotor_node = SceneNode::from_vao(main_rotor_vao, helicopter.main_rotor.index_count);
        let mut tail_rotor_node = SceneNode::from_vao(tail_rotor_vao, helicopter.tail_rotor.index_count);

        tail_rotor_node.reference_point = glm::vec3(0.35, 2.3, 10.4);
        main_rotor_node.reference_point = glm::vec3(0.0, 0.0, 0.0);
        /* TEST */
        //body_node.position = glm::vec3(0.0, 0.0, -20.0);
        //terrain_node.position = glm::vec3(0.0, -3.0, 0.0);
        //body_node.rotation.y = -3.14*3.0/4.0;

        root.add_child(&terrain_node);
        terrain_node.add_child(&body_node);
        body_node.add_child(&door_node);
        body_node.add_child(&main_rotor_node);
        body_node.add_child(&tail_rotor_node);*/


        // Basic usage of shader helper:
        // The example code below returns a shader object, which contains the field `.program_id`.
        // The snippet is not enough to do the assignment, and will need to be modified (outside of
        // just using the correct path), but it only needs to be called once
        //
        //     shader::ShaderBuilder::new()
        //        .attach_file("./path/to/shader.file")
        //        .link();
        let program;
        unsafe {
            program = shader::ShaderBuilder::new()
            .attach_file("shaders/simple.vert")
            .attach_file("shaders/simple.frag")
            .link();
            program.activate();
        }

        let mut angles: Vec<f32> = vec![0.0, 0.0];
        let mut position: Vec<f32> = vec![0.0, 0.0, 0.0];
        

        // Used to demonstrate keyboard handling -- feel free to remove
        let mut _arbitrary_number = 0.0;

        let first_frame_time = std::time::Instant::now();
        let mut last_frame_time = first_frame_time;
        // The main rendering loop
        loop {
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(first_frame_time).as_secs_f32();
            let delta_time = now.duration_since(last_frame_time).as_secs_f32();
            last_frame_time = now;

            // Handle keyboard input
            if let Ok(keys) = pressed_keys.lock() {
                for key in keys.iter() {
                    match key {
                        VirtualKeyCode::Space => {
                            position[1] += delta_time*40.0;
                        },
                        VirtualKeyCode::A => {
                            position[0] -= delta_time*40.0;
                        },
                        VirtualKeyCode::LShift => {
                            position[1] -= delta_time*40.0;
                        },
                        VirtualKeyCode::D => {
                            position[0] += delta_time*40.0;
                        },
                        VirtualKeyCode::S => {
                            position[2] += delta_time*40.0;
                        },
                        VirtualKeyCode::W => {
                            position[2] -= delta_time*40.0;
                        },
                        VirtualKeyCode::Up => {
                            angles[0] += delta_time*0.5;
                        },
                        VirtualKeyCode::Down => {
                            angles[0] -= delta_time*0.5;
                        },
                        VirtualKeyCode::Left => {
                            angles[1] += delta_time*0.5;
                        },
                        VirtualKeyCode::Right => {
                            angles[1] -= delta_time*0.5;
                        },
                        


                        _ => { }
                    }
                }
            }
            // Handle mouse movement. delta contains the x and y movement of the mouse since last frame in pixels
            if let Ok(mut delta) = mouse_delta.lock() {



                *delta = (0.0, 0.0);
            }

            unsafe {
                gl::ClearColor(0.76862745, 0.71372549, 0.94901961, 1.0); // moon raker, full opacity
                //gl::ClearColor(0.0, 0.0, 0.0, 1.0); // moon raker, full opacity
                //Temp:
                //gl::ClearColor(1.0,1.0,1.0,1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                let offset = 0.2;

                for i in 0..=4{
                    // Issue the necessary commands to draw your scene here
                    //Make rotors rotate
                    (*(*(*root.children[0]).children[i]).children[1]).rotation.y = 1.6*elapsed;
                    (*(*(*root.children[0]).children[i]).children[2]).rotation.x = 1.6*elapsed;
                    //main_rotor_node.rotation.y = 1.6*elapsed;
                    //tail_rotor_node.rotation.x = 1.6*elapsed;

                    //Get animation for helicopter:
                    let heading: toolbox::Heading = toolbox::simple_heading_animation(elapsed + i*offset);

                    body_node.position.x = heading.x;
                    body_node.position.z = heading.z;

                    body_node.rotation.y = heading.yaw;
                    body_node.rotation.x = heading.pitch;
                    body_node.rotation.z = heading.roll;
                }

                //Update transformations
                update_node_transformations(&mut root as &mut scene_graph::SceneNode, &glm::identity());

                let perspective_mat: glm::Mat4 = glm::perspective(1.333, 1.0, 1.0, 1000.0);

                draw_scene(&root, &perspective_mat);
            }

            context.swap_buffers().unwrap();
        }
    });

    // Keep track of the health of the rendering thread
    let render_thread_healthy = Arc::new(RwLock::new(true));
    let render_thread_watchdog = Arc::clone(&render_thread_healthy);
    thread::spawn(move || {
        if !render_thread.join().is_ok() {
            if let Ok(mut health) = render_thread_watchdog.write() {
                println!("Render thread panicked!");
                *health = false;
            }
        }
    });

    // Start the event loop -- This is where window events get handled
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        // Terminate program if render thread panics
        if let Ok(health) = render_thread_healthy.read() {
            if *health == false {
                *control_flow = ControlFlow::Exit;
            }
        }

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
            },
            // Keep track of currently pressed keys to send to the rendering thread
            Event::WindowEvent { event: WindowEvent::KeyboardInput {
                input: KeyboardInput { state: key_state, virtual_keycode: Some(keycode), .. }, .. }, .. } => {

                if let Ok(mut keys) = arc_pressed_keys.lock() {
                    match key_state {
                        Released => {
                            if keys.contains(&keycode) {
                                let i = keys.iter().position(|&k| k == keycode).unwrap();
                                keys.remove(i);
                            }
                        },
                        Pressed => {
                            if !keys.contains(&keycode) {
                                keys.push(keycode);
                            }
                        }
                    }
                }

                // Handle escape separately
                match keycode {
                    Escape => {
                        *control_flow = ControlFlow::Exit;
                    },
                    Q => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => { }
                }
            },
            Event::DeviceEvent { event: DeviceEvent::MouseMotion { delta }, .. } => {
                // Accumulate mouse movement
                if let Ok(mut position) = arc_mouse_delta.lock() {
                    *position = (position.0 + delta.0 as f32, position.1 + delta.1 as f32);
                }
            },
            _ => { }
        }
    });
}