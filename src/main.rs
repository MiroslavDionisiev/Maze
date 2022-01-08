use ggez::{
    conf::{WindowMode, WindowSetup},
    event, ContextBuilder, GameResult,
};
use std::path;
mod game;

fn main() -> GameResult
{
    let win_mode = WindowMode::default().dimensions(1000.0, 800.0);

    let win_setup = WindowSetup::default().title("Maze");

    //let mut asset_path = path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    //asset_path.push("resources");

    let (mut ctx, event_loop) = ContextBuilder::new("Maze", "Miroslav")
        .window_setup(win_setup)
        .window_mode(win_mode)
        /* .add_resource_path(asset_path.clone())*/
        .build()
        .unwrap();
        
    let game = game::MazeGame::new(&mut ctx)?;

    event::run(ctx, event_loop, game)
}
