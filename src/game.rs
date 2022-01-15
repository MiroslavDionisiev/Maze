use ggez::{
    event::{self, EventHandler},
    conf::{Conf, WindowMode},
    graphics::{self, DrawMode, DrawParam},
    Context, GameResult,
    input,
    timer,
    mint::{Point2}
};
use rand::{self, Rng};
use Maze::bot::Bot;
use Maze::player::Player;
use Maze::maze_generator::Graph;
use Maze::assets::Assets;

const CELL_SIZE: i32 = 45;
const WALL: char = 'W';
const FLOOR: char = '.';
const PLAYER: char = 'P';
const BOT: char = 'E';
const EXIT: char = 'V';
const KEY: char = 'K';
const UP: char = 'W';
const DOWN: char = 'S';
const LEFT: char = 'A';
const RIGHT: char = 'D';

pub struct MazeGame
{
    player: Player,
    ai: Bot,
    assets: Assets,
    game_over: bool,
    result: bool,
    map: Vec<Vec<char>>,
    time_until_bot_speed_up: f32,
    conf: Conf
}

impl MazeGame {
    pub fn new(ctx: &mut Context, conf: Conf) -> GameResult<Self>
    {
        let assets = Assets::new(ctx)?;

        let mut graph = Graph::new(10, 10);
        graph.generate_maze(0, 0);
        let mut map =graph.draw_maze();

        let player = Player::new(1, 1);

        let bot_location = generate_location(&map);
        let ai = Bot::new(bot_location.x, bot_location.y);

        map[player.y][player.x] = PLAYER;

        map[ai.y][ai.x] = BOT;
        let exit_y = map.len() - 2;
        let exit_x = map[0].len() - 2;

        map[exit_y][exit_x] = EXIT;

        let key_location = generate_location(&map);
        map[key_location.y][key_location.x] = KEY;
        
        let maze = MazeGame
        {
            player: player,
            ai: ai,
            assets: assets,
            game_over: false,
            result: false,
            map: map,
            time_until_bot_speed_up: 1.0,
            conf: conf
        };
        Ok(maze)
    }

    pub fn update_player_position(&mut self, new_x: usize, new_y: usize, keycode: event::KeyCode)
    {
        let mut is_on_exit = false;
        self.player.update_direction(keycode);
        if self.map[new_y][new_x] == WALL 
        {
            return;
        } 
        else if self.map[new_y][new_x] == BOT 
        {
            self.game_over = true;
            return;
        } 
        else if self.map[new_y][new_x] == EXIT && self.player.hasKey == true
        {
            self.game_over = true;
            self.result = true;
            return;
        }
        else if self.map[new_y][new_x] == EXIT && self.player.hasKey == false
        {
            is_on_exit = true;
        }
        else if self.map[new_y][new_x] == KEY
        {
            self.player.hasKey = true;
        }
        
        match self.player.is_on_exit
        {
            true => {self.map[self.player.y][self.player.x] = EXIT;},
            false => {self.map[self.player.y][self.player.x] = FLOOR;}
        }
        self.player.update(new_x, new_y, is_on_exit);
        self.map[self.player.y][self.player.x] = PLAYER;
    }

    pub fn update_bot_position(&mut self, new_x: usize, new_y: usize)
    {
        let mut is_on_exit = false;
        let mut is_on_key = false;

        if self.map[new_y][new_x] == WALL 
        {
            self.ai.update_direction(false);
            return;
        } 
        else if self.map[new_y][new_x] == PLAYER
        {
            self.game_over = true;
            return;
        } 
        else if self.map[new_y][new_x] == EXIT
        {
            is_on_exit = true;
        }
        else if self.map[new_y][new_x] == KEY
        {
            is_on_key = true;
        }

        match (self.ai.is_on_exit, self.ai.is_on_key)
        {
            (false, true) => {self.map[self.ai.y][self.ai.x] = KEY;},
            (true, false) => {self.map[self.ai.y][self.ai.x] = EXIT;},
            (false, false) => {self.map[self.ai.y][self.ai.x] = FLOOR;},
            _ => ()
        }
        self.ai.update_position(new_x, new_y, is_on_exit, is_on_key);
        self.map[self.ai.y][self.ai.x] = BOT;
    }

    pub fn restart_timer(&mut self)
    {
        self.time_until_bot_speed_up = 1.0;
    }

    fn check_for_cross_road(&self) -> bool
    {
        let mut count = 0;
        if self.map[self.ai.y][self.ai.x+1] == '.'
        {
            count+=1;
        }
        if self.map[self.ai.y][self.ai.x-1] == '.'
        {
            count+=1;
        }
        if self.map[self.ai.y+1][self.ai.x] == '.'
        {
            count+=1;
        }
        if self.map[self.ai.y-1][self.ai.x] == '.'
        {
            count+=1;
        }

        if count > 2
        {
            return true;
        }
        false
    }
}

fn generate_location(map: &Vec<Vec<char>>) -> Point2<usize>
    {
        let mut rng = rand::thread_rng();
        let mut x = rng.gen_range(0..map[0].len());
        let mut y = rng.gen_range(0..map.len());
        while map[y][x] != FLOOR
        {
            x = rng.gen_range(0..map[0].len());
            y = rng.gen_range(0..map.len());
        }
        Point2 { x: x, y: y }
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
            self.ai.look_for_player(self.map.to_owned());
            let seconds = 1.0 / (DESIRED_FPS as f32);
            self.ai.time_until_next_step -= seconds;
            self.time_until_bot_speed_up -= seconds;
            if self.ai.time_until_next_step <= 0.0
            {
                let is_on_cross_road = self.check_for_cross_road();
                if is_on_cross_road
                {
                    self.ai.update_direction(is_on_cross_road);
                }
                match self.ai.direction
                {
                    UP =>
                    {
                        self.update_bot_position(self.ai.x, self.ai.y-1);
                    }
                    DOWN =>
                    {
                        self.update_bot_position(self.ai.x, self.ai.y+1);
                    }
                    LEFT =>
                    {
                        self.update_bot_position(self.ai.x-1, self.ai.y);
                    }
                    RIGHT =>
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
            event::KeyCode::D => self.update_player_position(self.player.x+1, self.player.y, keycode),
            event::KeyCode::A => self.update_player_position(self.player.x-1, self.player.y, keycode),
            event::KeyCode::W => self.update_player_position(self.player.x, self.player.y-1, keycode),
            event::KeyCode::S => self.update_player_position(self.player.x, self.player.y+1, keycode),
            _ => ()
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> 
    {
        for (y, row) in self.map.iter().enumerate()
        {
            for (x, cell) in row.iter().enumerate()
            {
                let x_sq = x as i32 * CELL_SIZE;
                let y_sq = y as i32 * CELL_SIZE;

                match *cell
                {
                    WALL =>
                    {
                        let draw_param = DrawParam::new().dest(Point2{x:x_sq as f32, y:y_sq as f32});
                        graphics::draw(ctx, &self.assets.wall, draw_param)?;
                    }
                    FLOOR =>
                    {
                        let draw_param = DrawParam::new().dest(Point2{x:x_sq as f32, y:y_sq as f32});
                        graphics::draw(ctx, &self.assets.floor, draw_param)?;
                    }
                    PLAYER =>
                    {
                        self.player.draw(ctx, &self.assets, x_sq, y_sq);
                    }
                    EXIT =>
                    {
                        let draw_param = DrawParam::new().dest(Point2{x:x_sq as f32, y:y_sq as f32});
                        graphics::draw(ctx, &self.assets.floor, draw_param)?;
                        graphics::draw(ctx, &self.assets.door, draw_param)?;
                    }
                    BOT =>
                    {
                        self.ai.draw(ctx, &self.assets, x_sq, y_sq);
                    }
                    KEY =>
                    {
                        let draw_param = DrawParam::new().dest(Point2{x:x_sq as f32, y:y_sq as f32});
                        graphics::draw(ctx, &self.assets.floor, draw_param)?;
                        graphics::draw(ctx, &self.assets.key, draw_param)?;
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
                x: (self.conf.window_mode.width - text.width(ctx)) / 2.0,
                y: (self.conf.window_mode.height - text.height(ctx)) / 2.0,
            };
            graphics::draw(ctx, &text, graphics::DrawParam::default().dest(top_left))?;
            graphics::present(ctx)?;
            return Ok(())
        }

        graphics::present(ctx)?;
        Ok(())
    }
}