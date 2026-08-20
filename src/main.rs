mod gmaths;
mod graphics;
mod museum;
mod museum_control_panel;
mod museum_gl;
mod scene;

fn main() {
    if let Err(error) = museum::run() {
        eprintln!("Museum failed to start: {error}");
    }
}
