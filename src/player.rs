use ggez::{
    graphics::{self, DrawParam},
    event::KeyCode,
    Context,
    GameResult,
    mint::{Point2}
};

const PLAYER_UP_IMG: &str = "\\player_w.png";
const PLAYER_DOWN_IMG: &str = "\\player_s.png";
const PLAYER_LEFT_IMG: &str = "\\player_a.png";
const PLAYER_RIGHT_IMG: &str = "\\player_d.png";
const FLOOR_IMG: &str = "\\floor.png";

pub struct Player
{
    pub x: usize,
    pub y: usize,
    pub direction: KeyCode
}

impl Player
{
    pub fn new(x: usize, y: usize) -> Self
    {
        Player{x: x, y: y, direction: KeyCode::S}
    }

    pub fn update(&mut self, x: usize, y: usize)
    {
        self.x = x;
        self.y = y;
    }

    pub fn update_direction(&mut self, keycode: KeyCode)
    {
        self.direction = keycode;
    }

    pub fn draw(&self, ctx: &mut Context, x_sq: i32, y_sq: i32) -> GameResult
    {
        let draw_param = DrawParam::new().dest(Point2{x:x_sq as f32, y:y_sq as f32});
        let floor = graphics::Image::new(ctx, FLOOR_IMG)?;
        graphics::draw(ctx, &floor, draw_param)?;

        match self.direction 
        {
            KeyCode::D => 
            {
                let draw_param = DrawParam::new().dest(Point2{x:x_sq as f32, y:y_sq as f32});
                let img = graphics::Image::new(ctx, PLAYER_RIGHT_IMG)?;
                graphics::draw(ctx, &img, draw_param)?;
            },
            KeyCode::A =>
            {
                let draw_param = DrawParam::new().dest(Point2{x:x_sq as f32, y:y_sq as f32});
                let img = graphics::Image::new(ctx, PLAYER_LEFT_IMG)?;
                graphics::draw(ctx, &img, draw_param)?;
            },
            KeyCode::W => 
            {
                let draw_param = DrawParam::new().dest(Point2{x:x_sq as f32, y:y_sq as f32});
                let img = graphics::Image::new(ctx, PLAYER_UP_IMG)?;
                graphics::draw(ctx, &img, draw_param)?;
            },
            KeyCode::S => 
            {
                let draw_param = DrawParam::new().dest(Point2{x:x_sq as f32, y:y_sq as f32});
                let img = graphics::Image::new(ctx, PLAYER_DOWN_IMG)?;
                graphics::draw(ctx, &img, draw_param)?;
            },
            _ => ()
        }
        Ok(())
    }
}