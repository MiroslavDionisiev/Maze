use ggez::{
    event::{self, EventHandler},
    graphics::{self, DrawMode, DrawParam, Image},
    Context, GameResult,
    input,
    timer,
    mint::{Point2}
};
use std::collections::HashMap;

use Maze::bot::Bot;
use Maze::player::Player;

pub struct MazeGame
{
    elements: HashMap::<char, Image>,
    player: Player,
    ai: Bot,
    game_over: bool,
    result: bool,
    map: [[char; 19]; 19],
    time_until_bot_speed_up: f32
}

impl MazeGame {
    pub fn new(ctx: &mut Context) -> GameResult<Self>
    {
        let player = Player::new(1, 1);
        let ai = Bot::new(10, 1);
        let mut elements = HashMap::<char, Image>::new();
        //let img = Image::new(ctx, "\\wall.png")?;
        //elements.insert('W', img);

        let mut map = [
            ['W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W'],
            ['W', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'W'],
            ['W', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'W'],
            ['W', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'W'],
            ['W', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'W'],
            ['W', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'W'],
            ['W', '.', '.', '.', '.', '.', '.', 'W', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'W'],
            ['W', '.', '.', '.', '.', '.', '.', 'W', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'W'],
            ['W', '.', '.', '.', '.', '.', '.', 'W', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'W'],
            ['W', '.', '.', '.', '.', '.', '.', 'W', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'W'],
            ['W', '.', '.', '.', '.', '.', '.', 'W', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'W'],
            ['W', '.', '.', '.', '.', '.', '.', 'W', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'W'],
            ['W', '.', '.', '.', '.', '.', '.', 'W', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'W'],
            ['W', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'W'],
            ['W', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'W'],
            ['W', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'W'],
            ['W', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'W'],
            ['W', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', 'V', 'W'],
            ['W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W']
        ];
        map[player.y][player.x] = 'P';
        map[ai.y][ai.x] = 'E';
        
        let maze = MazeGame
        {
            elements: elements,
            player: player,
            ai: ai,
            game_over: false,
            result: false,
            map: map,
            time_until_bot_speed_up: 1.0
        };
        Ok(maze)
    }

    pub fn update_player_position(&mut self, newX: usize, newY: usize)
    {
        if self.map[newY][newX] == 'W' 
        {
            return;
        } 
        else if self.map[newY][newX] == 'E' 
        {
            self.game_over = true;
            return;
        } 
        else if self.map[newY][newX] == 'V' 
        {
            self.game_over = true;
            self.result = true;
            return;
        }
        
        self.map[self.player.y][self.player.x] = '.';
        self.player.update(newX, newY);
        self.map[self.player.y][self.player.x] = 'P';
    }

    pub fn update_bot_position(&mut self, newX: usize, newY: usize)
    {
        let mut is_on_exit = false;
        if self.map[newY][newX] == 'W' 
        {
            self.ai.update_direction();
            return;
        } 
        else if self.map[newY][newX] == 'P' 
        {
            self.game_over = true;
            return;
        } 
        else if self.map[newY][newX] == 'V' 
        {
            is_on_exit = true;
        }

        match self.ai.is_on_exit
        {
            true => {self.map[self.ai.y][self.ai.x] = 'V';},
            false => {self.map[self.ai.y][self.ai.x] = '.';}
        }
        self.ai.update_position(newX, newY, is_on_exit);
        self.map[self.ai.y][self.ai.x] = 'E';
    }

    pub fn restart_timer(&mut self)
    {
        self.time_until_bot_speed_up = 1.0;
    }
}

impl EventHandler<ggez::GameError> for MazeGame 
{
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> 
    {
        if self.game_over 
        {
            return Ok(());
        }
        
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) 
        {
            self.ai.look_for_player(self.map);
            let seconds = 1.0 / (DESIRED_FPS as f32);
            self.ai.time_until_next_step -= seconds;
            self.time_until_bot_speed_up -= seconds;
            if self.ai.time_until_next_step <= 0.0
            {
                match self.ai.direction
                {
                    'W' =>
                    {
                        self.update_bot_position(self.ai.x, self.ai.y-1);
                    }
                    'S' =>
                    {
                        self.update_bot_position(self.ai.x, self.ai.y+1);
                    }
                    'A' =>
                    {
                        self.update_bot_position(self.ai.x-1, self.ai.y);
                    }
                    'D' =>
                    {
                        self.update_bot_position(self.ai.x+1, self.ai.y);
                    }
                    _ => ()
                }
                self.ai.restart_timer();
            }

            if self.time_until_bot_speed_up <= 0.0
            {
                self.restart_timer();
            }
        }
        
        Ok(())
    }

    fn key_down_event(&mut self,
        ctx: &mut Context,
        keycode: event::KeyCode,
        _keymod: input::keyboard::KeyMods,
        _repeat: bool) 
    {
        match keycode 
        {
            event::KeyCode::D => self.update_player_position(self.player.x+1, self.player.y),
            event::KeyCode::A => self.update_player_position(self.player.x-1, self.player.y),
            event::KeyCode::W => self.update_player_position(self.player.x, self.player.y-1),
            event::KeyCode::S => self.update_player_position(self.player.x, self.player.y+1),
            _ => ()
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> 
    {
        for (y, row) in self.map.iter().enumerate()
        {
            for (x, cell) in row.iter().enumerate()
            {
                let x_sq = x as i32 * 30;
                let y_sq = y as i32 * 30;

                match *cell
                {
                    'W' =>
                    {
                        let r = graphics::Rect::new_i32(x_sq, y_sq, 30, 30);
                        let mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), r, graphics::Color::MAGENTA)?;
                        graphics::draw(ctx, &mesh, DrawParam::default())?;
                    }
                    '.' =>
                    {
                        let r = graphics::Rect::new_i32(x_sq, y_sq, 30, 30);
                        let mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), r, graphics::Color::WHITE)?;
                        graphics::draw(ctx, &mesh, DrawParam::default())?;
                    }
                    'P' =>
                    {
                        let r = graphics::Rect::new_i32(x_sq, y_sq, 30, 30);
                        let mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), r, graphics::Color::BLUE)?;
                        graphics::draw(ctx, &mesh, DrawParam::default())?;
                    }
                    'V' =>
                    {
                        let r = graphics::Rect::new_i32(x_sq, y_sq, 30, 30);
                        let mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), r, graphics::Color::GREEN)?;
                        graphics::draw(ctx, &mesh, DrawParam::default())?;
                    }
                    'E' =>
                    {
                        let r = graphics::Rect::new_i32(x_sq, y_sq, 30, 30);
                        let mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), r, graphics::Color::RED)?;
                        graphics::draw(ctx, &mesh, DrawParam::default())?;
                    }
                    _ => {continue;}
                }
            }
        }

        if self.game_over 
        {
            graphics::clear(ctx, graphics::Color::BLACK);
            let text = match self.result
            {
                true => graphics::Text::new("You foun the exit!"),
                false => graphics::Text::new("Game over!"),
            };

            let top_left = Point2 {
                x: (1000.0 - text.width(ctx)) / 2.0,
                y: (800.0 - text.height(ctx)) / 2.0,
            };
            graphics::draw(ctx, &text, graphics::DrawParam::default().dest(top_left))?;
            graphics::present(ctx)?;
            return Ok(())
        }

        graphics::present(ctx)?;
        Ok(())
    }
}