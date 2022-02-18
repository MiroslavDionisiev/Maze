use ggez::{
    graphics::{self, DrawParam},
    event::KeyCode,
    Context,
    GameResult,
    mint::{Point2}
};
use crate::assets::Assets;

#[derive(Clone, Copy)]
pub struct Player
{
    pub x: usize,
    pub y: usize,
    pub direction: KeyCode,
    pub has_key: bool,
    pub is_on_exit: bool
}

impl Player
{
    pub fn new(x: usize, y: usize) -> Self
    {
        Player{x: x, y: y, direction: KeyCode::S, has_key: false, is_on_exit: false}
    }

    pub fn update(&mut self, x: usize, y: usize, is_on_exit: bool)
    {
        self.x = x;
        self.y = y;
        self.is_on_exit = is_on_exit;
    }

    pub fn update_direction(&mut self, keycode: KeyCode)
    {
        self.direction = keycode;
    }

    pub fn draw(&self, ctx: &mut Context, assets: &Assets, x_sq: i32, y_sq: i32) -> GameResult
    {
        let draw_param = DrawParam::new().dest(Point2{x:x_sq as f32, y:y_sq as f32});
        graphics::draw(ctx, &assets.floor, draw_param)?;
        if self.has_key == false && self.is_on_exit == true
        {
            graphics::draw(ctx, &assets.door, draw_param)?;
        }

        match self.direction 
        {
            KeyCode::D => 
            {
                graphics::draw(ctx, &assets.player_right, draw_param)?;
            },
            KeyCode::A =>
            {
                graphics::draw(ctx, &assets.player_left, draw_param)?;
            },
            KeyCode::W => 
            {
                graphics::draw(ctx, &assets.player_up, draw_param)?;
            },
            KeyCode::S => 
            {
                graphics::draw(ctx, &assets.player_down, draw_param)?;
            },
            _ => ()
        }
        Ok(())
    }
}