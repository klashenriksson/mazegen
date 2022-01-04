//  Copyright (c) Klas Henriksson 2022.
//  All rights reserved.

use crate::gen::{Maze, self};

const WALL_COLOR: u32 = 0xffffffff;

enum Dir {
    East,
    North,
    West,
    South
}

pub struct MazeVizDescritptor {
    pub offset_x: usize,
    pub offset_y: usize,
    pub width: usize,
    pub height: usize,
    x_scale: usize,
    y_scale: usize
}

impl MazeVizDescritptor {
    pub fn new(offset_x: usize, offset_y: usize, width: usize, height: usize, maze_width: usize, maze_height: usize) -> Self {
        let x_scale = width as f32 / maze_width as f32;
        let y_scale = height as f32 / maze_height as f32;

        Self {
            offset_x,
            offset_y,
            width,
            height,
            x_scale: x_scale.ceil() as usize,
            y_scale: y_scale.ceil() as usize,
        }
    }

    pub fn rescale(&mut self, maze_width: usize, maze_height: usize)
    {
        let x_scale = self.width as f32 / maze_width as f32;
        let y_scale = self.height as f32 / maze_height as f32;

        self.x_scale = x_scale.ceil() as usize;
        self.y_scale = y_scale.ceil() as usize;
    }
}

pub struct Framebuffer {
    pub buffer: Vec<u32>,
    pub width: usize,
    pub height: usize,
}

impl Framebuffer {
    pub fn clear(&mut self, color: u32) {
        for i in 0..self.width*self.height {
            self.buffer[i] = color;
        }
    }

    pub fn draw_maze(&mut self, maze: &Maze, desc: &MazeVizDescritptor) {
        for cell in maze.cells.iter() {
            let color = if cell.visited {
                0xff0000
            } else {
                0xff
            };

            let (x,y) = gen::to_x_y(cell.idx, maze.width);
            self.draw_cell(x,y,color,&desc);
        }
    
        for x in 0..maze.width {
            for y in 0..maze.height {
                let idx = gen::to_idx(x, y, maze.width);
                let cell = &maze.cells[idx];
    
                if cell.wall_east && x < maze.width - 1 {
                    self.draw_wall(x, y, Dir::East, WALL_COLOR, desc);
                }
    
                if cell.wall_west && x > 0 {
                    self.draw_wall(x, y, Dir::West, WALL_COLOR, desc);
                }
    
                if cell.wall_north && y > 0 {
                    self.draw_wall(x, y, Dir::North, WALL_COLOR, desc);
                }
    
                if cell.wall_south && y < maze.height - 1 {
                    self.draw_wall(x, y, Dir::South, WALL_COLOR, desc);
                }
            }
        }
    }

    fn draw_wall(&mut self, x: usize, y: usize, dir: Dir, color: u32, viz_desc: &MazeVizDescritptor) {
        let (x_scale, y_scale) = (viz_desc.x_scale, viz_desc.y_scale);
        let wall_size = 1.max(x_scale/10);
    
        let (x_start, x_end, y_start, y_end) = match dir {
            Dir::North => (x*x_scale, x*x_scale+x_scale + wall_size, y*y_scale, y*y_scale + wall_size),
            Dir::South => (x*x_scale, x*x_scale+x_scale + wall_size, y*y_scale+y_scale, y*y_scale+y_scale + wall_size),
            Dir::East => (x*x_scale+x_scale, x*x_scale+x_scale + wall_size, y*y_scale, y*y_scale+y_scale + wall_size),
            Dir::West => (x*x_scale, x*x_scale + wall_size, y*y_scale, y*y_scale+y_scale + wall_size)
        };
    
        for x in x_start..x_end {
            for y in y_start..y_end {
                self.draw(x + viz_desc.offset_x,y + viz_desc.offset_y,color, viz_desc);
            }
        }
    }
    
    fn draw_cell(&mut self, x: usize, y: usize, color: u32, viz_desc: &MazeVizDescritptor) {
        let (x_scale, y_scale) = (viz_desc.x_scale, viz_desc.y_scale);

        let x_start = x * x_scale;
        let x_end = x_start + x_scale;
        let y_start = y * y_scale;
        let y_end = y_start + y_scale;
        
        for x in x_start..x_end {
            for y in y_start..y_end {
                self.draw(x + viz_desc.offset_x,y + viz_desc.offset_y, color, viz_desc);
            }
        }
    }

    fn draw(&mut self, x: usize, y: usize, color: u32, viz_desc: &MazeVizDescritptor) {
        if x >= viz_desc.offset_x + viz_desc.width || y >= viz_desc.offset_y + viz_desc.height {
            return;
        }
        self.buffer[x + y * self.width] = color;
    }
}