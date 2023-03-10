extern crate gl;
extern crate glfw;

use glfw::*;

use std::sync::mpsc::Receiver;

pub struct Window {
    glfw: glfw::Glfw,
    window_handle: glfw::Window,
    events: Receiver<(f64, glfw::WindowEvent)>,
}

impl Window {
    pub fn new(w: u32, h: u32, title: &str) -> Window {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let (mut window, events) = glfw
            .create_window(w, h, title, glfw::WindowMode::Windowed)
            .expect("failed to create a window");

        window.make_current();
        window.set_key_polling(true);

        Window {
            glfw,
            window_handle: window,
            events,
        }
    }

    pub fn init_gl(&mut self) {
        gl::load_with(|s| self.window_handle.get_proc_address(s) as *const _);
    }

    pub fn should_close(&self) -> bool {
        self.window_handle.should_close()
    }

    pub fn update(&mut self) {
        unsafe {
            gl::ClearColor(0.2, 0.1, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        self.glfw.poll_events();
        self.window_handle.swap_buffers();
    }

    pub fn process_key_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    // Make sure the viewport matches the new window dimensions.
                    unsafe { gl::Viewport(0, 0, width, height) }
                }
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window_handle.set_should_close(true)
                }
                _ => {}
            }
        }
    }
}
