use crate::gen::{Maze, self};

const PATH_COLOR: u32 = 0x0;
const WALL_COLOR: u32 = 0xffffffff;

enum Dir {
    East,
    North,
    West,
    South
}

pub fn draw(maze: &Maze, framebuffer: &mut [u32], fb_width: usize, fb_height: usize) {

    // first clear framebuffer
    for i in 0..fb_width*fb_height {
        framebuffer[i] = 0x0;
    }

    let x_scale = fb_width/maze.width;
    let y_scale = fb_height/maze.height;

    for x in 0..maze.width {
        for y in 0..maze.height {
            let idx = gen::to_idx(x, y, maze.width, maze.height);
            let cell = &maze.cells[idx];
            
            if cell.wall_east && x < maze.width - 1 {
                draw_wall(x, y, Dir::East, framebuffer, fb_width, WALL_COLOR, x_scale, y_scale);
            }

            if cell.wall_west && x > 0 {
                draw_wall(x, y, Dir::West, framebuffer, fb_width, WALL_COLOR, x_scale, y_scale);
            }

            if cell.wall_north && y > 0 {
                draw_wall(x, y, Dir::North, framebuffer, fb_width, WALL_COLOR, x_scale, y_scale);
            }

            if cell.wall_south && y < maze.height - 1 {
                draw_wall(x, y, Dir::South, framebuffer, fb_width, WALL_COLOR, x_scale, y_scale);
            }
        }
    }
}

fn draw_wall(x: usize, y: usize, dir: Dir, framebuffer: &mut [u32], fb_width: usize, color: u32, x_scale: usize, y_scale: usize) {
    let wall_size = 1.max(x_scale/10);

    let (x_start, x_end, y_start, y_end) = match dir {
        Dir::North => (x*x_scale, x*x_scale+x_scale, y*y_scale, y*y_scale + wall_size),
        Dir::South => (x*x_scale, x*x_scale+x_scale, y*y_scale+y_scale - wall_size, y*y_scale+y_scale),
        Dir::East => (x*x_scale+x_scale - wall_size, x*x_scale+x_scale, y*y_scale, y*y_scale+y_scale),
        Dir::West => (x*x_scale, x*x_scale + wall_size, y*y_scale, y*y_scale+y_scale)
    };

    for x in x_start..x_end {
        for y in y_start..y_end {
            framebuffer[x + y * fb_width] = color;
        }
    }
}