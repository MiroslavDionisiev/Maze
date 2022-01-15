use ggez::{
    graphics::{self, DrawParam},
    Context,
    GameResult,
    mint::{Point2}
};
use rand::{self, Rng};
use crate::assets::Assets;

const UP: char = 'W';
const DOWN: char = 'S';
const LEFT: char = 'A';
const RIGHT: char = 'D';

pub struct Bot
{
    pub x: usize,
    pub y: usize,
    pub time_until_next_step: f32,
    pub time_for_step: f32,
    pub direction: char,
    pub is_on_exit: bool,
    pub is_on_key: bool
}

impl Bot
{
    pub fn new(x: usize, y: usize) -> Self
    {
        let directions = ['W', 'D', 'A', 'S'];
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(0..4);
        Bot{x: x, y: y, time_until_next_step: 0.1, time_for_step: 0.1, direction: directions[random_number], is_on_exit: false, is_on_key: false}
    }

    pub fn update_position(&mut self, x: usize, y: usize, is_on_exit: bool, is_on_key: bool)
    {
        self.x = x;
        self.y = y;
        self.is_on_exit = is_on_exit;
        self.is_on_key = is_on_key;
    }

    pub fn update_direction(&mut self, is_on_cross_road: bool)
    {
        let mut directions = vec!['W', 'D', 'A', 'S'];
        if is_on_cross_road
        {
            let opposite_direction = get_oposite_direction(self.direction);
            let index = directions.iter().position(|x| *x == opposite_direction).unwrap();
            directions.remove(index);
        }
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(0..directions.len());
        self.direction = directions[random_number];
    }

    pub fn restart_timer(&mut self)
    {
        self.time_until_next_step = self.time_for_step;
    }

    pub fn look_for_player(&mut self, maze: Vec<Vec<char>>)
    {
        let mut tmp_x = self.x;
        let mut tmp_y = self.y;

        while maze[tmp_y][tmp_x] != 'W'
        {
            if maze[tmp_y][tmp_x] == 'P'
            {
                self.direction = 'W';
                return;
            }
            tmp_y -= 1;
        }
        while maze[tmp_y][tmp_x] != 'W'
        {
            if maze[tmp_y][tmp_x] == 'P'
            {
                self.direction = 'A';
                return;
            }
            tmp_x -= 1;
        }

        let mut tmp_x = self.x;
        let mut tmp_y = self.y;

        while maze[tmp_y][tmp_x] != 'W'
        {
            if maze[tmp_y][tmp_x] == 'P'
            {
                self.direction = 'S';
                return;
            }
            tmp_y += 1;
        }
        while maze[tmp_y][tmp_x] != 'W'
        {
            if maze[tmp_y][tmp_x] == 'P'
            {
                self.direction = 'D';
                return;
            }
            tmp_x += 1;
        }
    }

    pub fn speed_up(&mut self)
    {
        self.time_for_step -= 0.02;
    }
    
    pub fn draw(&self, ctx: &mut Context, assets: &Assets, x_sq: i32, y_sq: i32) -> GameResult
    {
        let draw_param = DrawParam::new().dest(Point2{x:x_sq as f32, y:y_sq as f32});
        graphics::draw(ctx, &assets.floor, draw_param)?;
        if self.is_on_exit
        {
            graphics::draw(ctx, &assets.door, draw_param)?;
        }

        match self.direction
        {
            UP =>
            {
                graphics::draw(ctx, &assets.bot_up, draw_param)?;
            }
            DOWN =>
            {
                graphics::draw(ctx, &assets.bot_down, draw_param)?;
            }
            LEFT =>
            {
                graphics::draw(ctx, &assets.bot_left, draw_param)?;
            }
            RIGHT =>
            {
                graphics::draw(ctx, &assets.bot_right, draw_param)?;
            }
            _ => ()
        }
        Ok(())
    }
}

fn get_oposite_direction(direction: char) -> char
    {
        match direction
        {
            UP => DOWN,
            DOWN => UP,
            LEFT => RIGHT,
            RIGHT => LEFT,
            _ => 'q'
        }
    }