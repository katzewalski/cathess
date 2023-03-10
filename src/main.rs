mod gfx;

fn main() {
    let mut window = gfx::window::Window::new(800, 600, "Hello, World");
    window.init_gl();

    while !window.should_close() {
        window.update();
        window.process_key_events();
    }
}
