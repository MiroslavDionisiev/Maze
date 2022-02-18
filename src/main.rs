use ggez::{
    conf::{Conf, WindowMode},
    event, ContextBuilder, GameResult, filesystem
};
use std::path;
use std::env;
use Maze::game;

fn main() -> GameResult
{
    let mut conf = Conf::new().
        window_mode(WindowMode {
            width: 945.0,
            height: 945.0,
            ..Default::default()
        });
    conf.window_setup.title = String::from("Maze");

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
        
    let game = game::MazeGame::new(&mut ctx, conf)?;

    event::run(ctx, event_loop, game)
}
