use ggez::graphics;
use ggez::{Context, GameResult};

const BOT_UP_IMG: &str = "\\bot_w.png";
const BOT_DOWN_IMG: &str = "\\bot_s.png";
const BOT_LEFT_IMG: &str = "\\bot_a.png";
const BOT_RIGHT_IMG: &str = "\\bot_d.png";
const PLAYER_UP_IMG: &str = "\\player_w.png";
const PLAYER_DOWN_IMG: &str = "\\player_s.png";
const PLAYER_LEFT_IMG: &str = "\\player_a.png";
const PLAYER_RIGHT_IMG: &str = "\\player_d.png";
const FLOOR_IMG: &str = "\\floor.png";
const WALL_IMG: &str = "\\wall.png";
const DOOR_IMG: &str = "\\door.png";
const KEY_IMG: &str = "\\key.png";

pub struct Assets {
    pub player_up:   graphics::Image,
    pub player_down:   graphics::Image,
    pub player_left:   graphics::Image,
    pub player_right:   graphics::Image,
    pub bot_up:   graphics::Image,
    pub bot_down:   graphics::Image,
    pub bot_left:   graphics::Image,
    pub bot_right:   graphics::Image,
    pub floor:   graphics::Image,
    pub wall:   graphics::Image,
    pub door:   graphics::Image,
    pub key:   graphics::Image
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let player_up = graphics::Image::new(ctx, PLAYER_UP_IMG)?;
        let player_down = graphics::Image::new(ctx, PLAYER_DOWN_IMG)?;
        let player_left = graphics::Image::new(ctx, PLAYER_LEFT_IMG)?;
        let player_right = graphics::Image::new(ctx, PLAYER_RIGHT_IMG)?;
        let bot_up = graphics::Image::new(ctx, BOT_UP_IMG)?;
        let bot_down = graphics::Image::new(ctx, BOT_DOWN_IMG)?;
        let bot_left = graphics::Image::new(ctx, BOT_LEFT_IMG)?;
        let bot_right = graphics::Image::new(ctx, BOT_RIGHT_IMG)?;
        let floor = graphics::Image::new(ctx, FLOOR_IMG)?;
        let wall = graphics::Image::new(ctx, WALL_IMG)?;
        let door = graphics::Image::new(ctx, DOOR_IMG)?;
        let key = graphics::Image::new(ctx, KEY_IMG)?;

        Ok(Assets {
            player_up, player_down, player_left, player_right, bot_up, bot_down, bot_left, bot_right, floor, wall, door, key
        })
    }
}