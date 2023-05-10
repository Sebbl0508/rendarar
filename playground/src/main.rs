use renderer::Game;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

fn main() {
    // If log level not set in env variable, set it to debug
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug");
    }

    env_logger::init();

    let mut game = setup();

    game.run();
}

fn setup() -> Game {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("WGPU Playground")
        .with_fullscreen(None)
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    Game::new(event_loop, window).unwrap()
}
