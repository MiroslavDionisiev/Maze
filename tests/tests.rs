use Maze::bot::Bot;
use Maze::player::Player;
use Maze::maze_generator::Graph;
use Maze::game::State;
use std::{thread, time};
use rand::{self, Rng};
use ggez::{
    event, GameResult,
    mint::{Point2}
};

const WALL: char = 'W';
const FLOOR: char = '.';
const PLAYER: char = 'P';
const BOT: char = 'E';
const EXIT: char = 'V';
const KEY: char = 'K';


pub struct MazeGameMock
{
    pub player: Player,
    pub ai: Bot,
    pub game_state: State,
    pub map: Vec<Vec<char>>,
    pub time_until_bot_speed_up: f32,
}

impl MazeGameMock {
    pub fn new() -> GameResult<Self>
    {
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
        
        let maze = MazeGameMock
        {
            player: player,
            ai: ai,
            game_state: State::Start,
            map: map,
            time_until_bot_speed_up: 1.0,
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
            self.game_state = State::Credits(String::from("Game Over!"));
            let ten_millis = time::Duration::from_secs_f32(0.5);
            thread::sleep(ten_millis);
            return;
        } 
        else if self.map[new_y][new_x] == EXIT && self.player.has_key == true
        {
            self.game_state = State::Credits(String::from("You found the exit!"));
            return;
        }
        else if self.map[new_y][new_x] == EXIT && self.player.has_key == false
        {
            is_on_exit = true;
        }
        else if self.map[new_y][new_x] == KEY
        {
            self.player.has_key = true;
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
            self.game_state = State::Credits(String::from("Game Over!"));
            let ten_millis = time::Duration::from_secs_f32(0.5);
            thread::sleep(ten_millis);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_speed_up() 
    {
        let mut bot = Bot::new(1,1);
        let old_speed = bot.time_for_step;
        bot.speed_up();
        assert!(old_speed>bot.time_for_step)
    }

    #[test]
    fn test_not_go_throught_wall()
    {
        let mut game = MazeGameMock::new().unwrap();
        let old_player_data = game.player.clone();
        game.update_player_position(old_player_data.x-1, old_player_data.y, event::KeyCode::W);
        assert_eq!(old_player_data.x, game.player.x);
        assert_eq!(old_player_data.y, game.player.y);
        assert_eq!(event::KeyCode::W, game.player.direction);
    }

    #[test]
    fn test_making_step()
    {
        let mut game = MazeGameMock::new().unwrap();
        let old_player_data = game.player.clone();
        if game.map[old_player_data.y+1][old_player_data.x] == FLOOR
        {
            game.update_player_position(old_player_data.x, old_player_data.y+1, event::KeyCode::S);
            assert_eq!(game.map[old_player_data.y+1][old_player_data.x], PLAYER);
            assert_eq!(old_player_data.x, game.player.x);
            assert_ne!(old_player_data.y, game.player.y);
        }
        else if game.map[old_player_data.y][old_player_data.x+1] == FLOOR
        {
            game.update_player_position(old_player_data.x+1, old_player_data.y, event::KeyCode::D);
            assert_eq!(game.map[old_player_data.y][old_player_data.x+1], PLAYER);
            assert_ne!(old_player_data.x, game.player.x);
            assert_eq!(old_player_data.y, game.player.y);
        }
    }

    #[test]
    fn test_face_bot()
    {
        let mut game = MazeGameMock::new().unwrap();
        game.ai.x = game.player.x + 1;
        game.ai.y = game.player.y;
        game.map[game.ai.y][game.ai.x] = BOT;
        game.update_player_position(game.player.x+1, game.player.y, event::KeyCode::D);
        
        assert_eq!(game.game_state, State::Credits(String::from("Game Over!")));
    }

    #[test]
    fn test_get_to_exit_with_key()
    {
        let mut game = MazeGameMock::new().unwrap();
        game.game_state = State::MainState;
        game.player.y = game.map.len() - 2;
        game.player.x = game.map[game.player.y].len() - 3;
        game.map[game.player.y][game.player.x] = PLAYER;
        game.player.has_key = true;
        game.update_player_position(game.player.x+1, game.player.y, event::KeyCode::D);
        
        assert_eq!(game.game_state, State::Credits(String::from("You found the exit!")));
    }

    #[test]
    fn test_get_to_exit_without_key()
    {
        let mut game = MazeGameMock::new().unwrap();
        game.game_state = State::MainState;
        game.player.y = game.map.len() - 2;
        game.player.x = game.map[game.player.y].len() - 3;
        game.map[game.player.y][game.player.x] = PLAYER;
        game.update_player_position(game.player.x+1, game.player.y, event::KeyCode::D);
        
        assert_eq!(game.game_state, State::MainState);
    }

    #[test]
    fn test_pick_up_key()
    {
        let mut game = MazeGameMock::new().unwrap();
        game.map[game.player.y][game.player.x+1] = KEY;
        game.game_state = State::MainState;
        assert_eq!(game.player.has_key, false);
        game.update_player_position(game.player.x+1, game.player.y, event::KeyCode::D);
        game.update_player_position(game.player.x-1, game.player.y, event::KeyCode::A);
        assert_eq!(game.map[game.player.y][game.player.x+1], FLOOR);
        assert_eq!(game.player.has_key, true);
    }
}