pub struct Player
{
    pub x: usize,
    pub y: usize
}

impl Player
{
    pub fn new(x: usize, y: usize) -> Self
    {
        Player{x: x, y: y}
    }

    pub fn update(&mut self, x: usize, y: usize)
    {
        self.x = x;
        self.y = y;
    }
}