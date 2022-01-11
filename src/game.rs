use ggez::{
    event::{self, EventHandler},
    graphics::{self, DrawMode, DrawParam},
    Context, GameResult,
    input,
    timer,
    mint::{Point2}
};

use Maze::bot::Bot;
use Maze::player::Player;
use Maze::maze_generator::Graph;

const CELL_SIZE: i32 = 45;
const WALL: char = 'W';
const FLOOR: char = '.';
const PLAYER: char = 'P';
const BOT: char = 'E';
const EXIT: char = 'V';
const UP: char = 'W';
const DOWN: char = 'S';
const LEFT: char = 'A';
const RIGHT: char = 'D';
const WALL_IMG: &str = "\\wall.png";
const FLOOR_IMG: &str = "\\floor.png";

pub struct MazeGame
{
    player: Player,
    ai: Bot,
    game_over: bool,
    result: bool,
    map: Vec<Vec<char>>,
    time_until_bot_speed_up: f32
}

impl MazeGame {
    pub fn new(ctx: &mut Context) -> GameResult<Self>
    {
        let player = Player::new(1, 1);
        let ai = Bot::new(7, 1);

        let mut graph = Graph::new(10, 10);
        graph.generate_maze(0, 0);
        let mut map =graph.draw_maze();

        map[player.y][player.x] = PLAYER;
        map[ai.y][ai.x] = BOT;
        let exit_y = map.len() - 2;
        let exit_x = map[0].len() - 2;
        map[exit_y][exit_x] = EXIT;
        
        let maze = MazeGame
        {
            player: player,
            ai: ai,
            game_over: false,
            result: false,
            map: map,
            time_until_bot_speed_up: 1.0
        };
        Ok(maze)
    }

    pub fn update_player_position(&mut self, new_x: usize, new_y: usize, keycode: event::KeyCode)
    {
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
        else if self.map[new_y][new_x] == EXIT 
        {
            self.game_over = true;
            self.result = true;
            return;
        }
        
        self.map[self.player.y][self.player.x] = FLOOR;
        self.player.update(new_x, new_y);
        self.map[self.player.y][self.player.x] = PLAYER;
    }

    pub fn update_bot_position(&mut self, new_x: usize, new_y: usize)
    {
        let mut is_on_exit = false;
        if self.map[new_y][new_x] == WALL 
        {
            self.ai.update_direction();
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

        match self.ai.is_on_exit
        {
            true => {self.map[self.ai.y][self.ai.x] = EXIT;},
            false => {self.map[self.ai.y][self.ai.x] = FLOOR;}
        }
        self.ai.update_position(new_x, new_y, is_on_exit);
        self.map[self.ai.y][self.ai.x] = BOT;
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
            self.ai.look_for_player(self.map.to_owned());
            let seconds = 1.0 / (DESIRED_FPS as f32);
            self.ai.time_until_next_step -= seconds;
            self.time_until_bot_speed_up -= seconds;
            if self.ai.time_until_next_step <= 0.0
            {
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
                        let wall = graphics::Image::new(ctx, WALL_IMG)?;
                        graphics::draw(ctx, &wall, draw_param)?;
                    }
                    FLOOR =>
                    {
                        let draw_param = DrawParam::new().dest(Point2{x:x_sq as f32, y:y_sq as f32});
                        let floor = graphics::Image::new(ctx, FLOOR_IMG)?;
                        graphics::draw(ctx, &floor, draw_param)?;
                    }
                    PLAYER =>
                    {
                        self.player.draw(ctx, x_sq, y_sq);
                    }
                    EXIT =>
                    {
                        let r = graphics::Rect::new_i32(x_sq, y_sq, CELL_SIZE, CELL_SIZE);
                        let mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), r, graphics::Color::GREEN)?;
                        graphics::draw(ctx, &mesh, DrawParam::default())?;
                    }
                    BOT =>
                    {
                        self.ai.draw(ctx, x_sq, y_sq);
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
                y: (1000.0 - text.height(ctx)) / 2.0,
            };
            graphics::draw(ctx, &text, graphics::DrawParam::default().dest(top_left))?;
            graphics::present(ctx)?;
            return Ok(())
        }

        graphics::present(ctx)?;
        Ok(())
    }
}