use std::collections::HashSet;
use rand::{self, Rng};

pub struct Cell {
    pub x: usize,
    pub y: usize,
    pub up: bool,
    pub left: bool,
    pub down: bool,
    pub right: bool,
}

pub struct Graph
{
    pub cells: Vec<Cell>,
    pub back_track: Vec<(usize, usize)>,
    pub visited: HashSet<(usize, usize)>,
    // because we start of a grid 
    pub width: usize,
    pub height: usize
}

impl Graph
{
    pub fn new(width: usize, height: usize) -> Self
    {
        let mut cells: Vec<Cell> = Vec::new();
        for i in 0..height
        {
            for j in 0..width
            {
                cells.push(Cell{
                    x: j,
                    y: i,
                    up: true,
                    down: true,
                    left: true,
                    right: true
                });
            }
        }

        Graph{cells: cells, back_track: Vec::new(), visited: HashSet::new(), width: width, height: height}
    }

    pub fn generate_maze(&mut self, x: usize, y: usize)
    {
        if !self.visited.contains(&(x, y)) 
        {
            self.visited.insert((x, y));
            self.back_track.push((x, y));
        }

        match self.get_random_neighbor(x, y)
        {
            Some(next_cell) =>
            {
                if next_cell.0 == x
                {
                    if next_cell.1 < y
                    {
                        self.cells[(self.width*next_cell.1) + next_cell.0].down = false;
                        self.cells[(self.width*y) + x].up = false;
                    }
                    else if next_cell.1 > y
                    {
                        self.cells[(self.width*next_cell.1) + next_cell.0].up = false;
                        self.cells[(self.width*y) + x].down = false;
                    }
                }
                else if next_cell.1 == y
                {
                    if next_cell.0 < x
                    {
                        self.cells[(self.width*next_cell.1) + next_cell.0].right = false;
                        self.cells[(self.width*y) + x].left = false;
                    }
                    else if next_cell.0 > x
                    {
                        self.cells[(self.width*next_cell.1) + next_cell.0].left = false;
                        self.cells[(self.width*y) + x].right = false;
                    }
                }
                else 
                {
                    panic!("Not neighbors");
                }
                self.generate_maze(next_cell.0, next_cell.1)
            },
            None => 
            {
                match self.back_track.pop()
                {
                    Some(cell) =>
                    {
                        self.generate_maze(cell.0, cell.1);
                        ()
                    },
                    None =>
                    {
                        ()
                    }
                }
            }
        }
    }

    pub fn draw_maze(&self) -> Vec<Vec<char>>
    {
        let mut maze: Vec<Vec<char>> = Vec::new();
        maze.push(Vec::new());
        maze[0].push('W');
        for _ in 0..self.width
        {
            maze[0].push('W');
            maze[0].push('W');
        }
        for i in 0..self.height
        {
            let mut row: Vec<char> = Vec::new();
            let mut row_bellow: Vec<char> = Vec::new();
            row.push('W');
            row_bellow.push('W');
            for j in 0..self.width
            {
                let index = (i*self.height) + j;
                row.push('.');
                if self.cells[index].right == false
                {
                    row.push('.');
                }
                else
                {
                    row.push('W');
                }
                if self.cells[index].down == false
                {
                    row_bellow.push('.');
                    row_bellow.push('W');
                }
                else
                {
                    row_bellow.push('W');
                    row_bellow.push('W');
                }
            }
            maze.push(row);
            maze.push(row_bellow);
        }
        maze
    }

    fn get_random_neighbor(&self, current_x: usize, current_y: usize) -> Option<(usize, usize)>
    {
        let unvisited_neighbors = self.get_unvisited_neighbors(current_x, current_y);
        if unvisited_neighbors.len() == 0 
        {
            None
        } 
        else 
        {
            let mut rng = rand::thread_rng();
            let random_number = rng.gen_range(0..unvisited_neighbors.len());
            Some(unvisited_neighbors[random_number])
        }
    }

    fn get_unvisited_neighbors(&self, current_x: usize, current_y: usize) -> Vec<(usize, usize)>
    {
        let mut unvisited: Vec<(usize, usize)> = Vec::new();
        if current_x > 0
        {
            if !self.visited.contains(&(current_x-1, current_y)) {
                unvisited.push((current_x-1, current_y));
            }
        }
        if current_x < self.width - 1
        {
            if !self.visited.contains(&(current_x+1, current_y)) {
                unvisited.push((current_x+1, current_y));
            }
        }
        if current_y > 0
        {
            if !self.visited.contains(&(current_x, current_y-1)) {
                unvisited.push((current_x, current_y-1));
            }
        }
        if current_y < self.height - 1
        {
            if !self.visited.contains(&(current_x, current_y+1)) {
                unvisited.push((current_x, current_y+1));
            }
        }
        unvisited
    }
}