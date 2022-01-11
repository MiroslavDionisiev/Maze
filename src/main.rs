use ggez::{
    conf::{Conf, WindowMode, WindowSetup},
    event, ContextBuilder, GameResult, filesystem
};
use std::path;
use std::env;
mod game;

fn main() -> GameResult
{
    let conf = Conf::new().
        window_mode(WindowMode {
            width: 1000.0,
            height: 1000.0,
            ..Default::default()
        });

    let (mut ctx, event_loop) = ContextBuilder::new("Maze", "Miroslav").
        default_conf(conf.clone()).
        build().
        unwrap();

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") 
    {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        filesystem::mount(&mut ctx, &path, true);
    }
        
    let game = game::MazeGame::new(&mut ctx)?;

    event::run(ctx, event_loop, game)
}
