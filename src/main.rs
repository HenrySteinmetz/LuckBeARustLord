use pix_engine::prelude::{Engine, PixResult};
mod game;
use game::GameState;

fn main() -> PixResult<()> {
    let mut engine = Engine::builder()
        .dimensions(1920, 1080)
        .title("Luck be a Rustlord")
        .position(10, 10)
        .show_frame_rate()
        .target_frame_rate(60)
        .build()?;
    let mut app = GameState::new();
    engine.run(&mut app)
}
