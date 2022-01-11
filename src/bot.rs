use ggez::{
    graphics::{self, DrawParam},
    Context,
    GameResult,
    mint::{Point2}
};
use rand::{self, Rng};

const BOT_UP_IMG: &str = "\\bot_w.png";
const BOT_DOWN_IMG: &str = "\\bot_s.png";
const BOT_LEFT_IMG: &str = "\\bot_a.png";
const BOT_RIGHT_IMG: &str = "\\bot_d.png";
const FLOOR_IMG: &str = "\\floor.png";
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
    pub is_on_exit: bool
}

impl Bot
{
    pub fn new(x: usize, y: usize) -> Self
    {
        let directions = ['W', 'D', 'A', 'S'];
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(0..4);
        Bot{x: x, y: y, time_until_next_step: 0.1, time_for_step: 0.1, direction: directions[random_number], is_on_exit: false}
    }

    pub fn update_position(&mut self, x: usize, y: usize, is_on_exit: bool)
    {
        self.x = x;
        self.y = y;
        self.is_on_exit = is_on_exit;
    }

    pub fn update_direction(&mut self)
    {
        let directions = ['W', 'D', 'A', 'S'];
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(0..4);
        self.direction = directions[random_number];
    }

    pub fn restart_timer(&mut self)
    {
        self.time_until_next_step = self.time_for_step;
    }

    pub fn look_for_player(&mut self, maze: Vec<Vec<char>>)
    {
        let mut tmp_x = self.x - 1;
        let mut tmp_y = self.y - 1;

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

        let mut tmp_x = self.x + 1;
        let mut tmp_y = self.y + 1;

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
    
    pub fn draw(&self, ctx: &mut Context, x_sq: i32, y_sq: i32) -> GameResult
    {
        let draw_param = DrawParam::new().dest(Point2{x:x_sq as f32, y:y_sq as f32});
        let floor = graphics::Image::new(ctx, FLOOR_IMG)?;
        graphics::draw(ctx, &floor, draw_param)?;

        match self.direction
        {
            UP =>
            {
                let draw_param = DrawParam::new().dest(Point2{x:x_sq as f32, y:y_sq as f32});
                let img = graphics::Image::new(ctx, BOT_UP_IMG)?;
                graphics::draw(ctx, &img, draw_param)?;
            }
            DOWN =>
            {
                let draw_param = DrawParam::new().dest(Point2{x:x_sq as f32, y:y_sq as f32});
                let img = graphics::Image::new(ctx, BOT_DOWN_IMG)?;
                graphics::draw(ctx, &img, draw_param)?;
            }
            LEFT =>
            {
                let draw_param = DrawParam::new().dest(Point2{x:x_sq as f32, y:y_sq as f32});
                let img = graphics::Image::new(ctx, BOT_LEFT_IMG)?;
                graphics::draw(ctx, &img, draw_param)?;
            }
            RIGHT =>
            {
                let draw_param = DrawParam::new().dest(Point2{x:x_sq as f32, y:y_sq as f32});
                let img = graphics::Image::new(ctx, BOT_RIGHT_IMG)?;
                graphics::draw(ctx, &img, draw_param)?;
            }
            _ => ()
        }
        Ok(())
    }
}