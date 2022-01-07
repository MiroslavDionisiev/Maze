use ggez::{
    event::{EventHandler, MouseButton},
    graphics::{self, Color, DrawMode, DrawParam, Image},
    Context, GameResult,
};
use std::collections::HashMap;

pub struct MazeGame
{
    elements: HashMap::<char, Image>,
    map: [[char; 8]; 9]
}

impl MazeGame {
    pub fn new(ctx: &mut Context) -> GameResult<Self>
    {
        let mut elements = HashMap::<char, Image>::new();
        let img = Image::new(ctx, "\\wall.png")?;
        elements.insert('W', img);

        let map = [
            ['W', 'W', 'W', 'W', 'W', 'W', 'W', 'W',],
            ['W', '.', '.', '.', '.', '.', '.', 'W',],
            ['W', '.', '.', '.', '.', '.', '.', 'W',],
            ['W', '.', '.', '.', '.', '.', '.', 'W',],
            ['W', '.', '.', '.', '.', '.', '.', 'W',],
            ['W', '.', '.', '.', '.', '.', '.', 'W',],
            ['W', '.', '.', '.', '.', '.', '.', 'W',],
            ['W', '.', '.', '.', '.', '.', '.', 'W',],
            ['W', 'W', 'W', 'W', 'W', 'W', 'W', 'W',]
        ];

        let maze = MazeGame
        {
            elements: elements,
            map: map
        };
        Ok(maze)
    }
}

impl EventHandler<ggez::GameError> for MazeGame 
{
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> 
    {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> 
    {
        for (y, row) in self.map.iter().enumerate()
        {
            for (x, cell) in row.iter().enumerate()
            {
                let x_sq = x as i32 * 30;
                let y_sq = y as i32 *30;

                let r = graphics::Rect::new_i32(x_sq, y_sq, 30, 30);
                let mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), r, graphics::Color::WHITE)?;
            
                graphics::draw(ctx, &mesh, DrawParam::default())?;

                if *cell == 'W'
                {
                    let img = match self.elements.get(cell) {
                        Some(i) => i,
                        None => continue,
                    };

                    let ddraw = (30 as f32 - img.width() as f32 * 1.5) / 2.;
                    let x_draw = x_sq as f32 + ddraw;
                    let y_draw = y_sq as f32 + ddraw;
                    let draw_param = DrawParam::new().dest([x_draw, y_draw]).scale([1.5, 1.5]);

                    graphics::draw(ctx, img, draw_param)?;
                }
            }
        }

        Ok(())
    }
}