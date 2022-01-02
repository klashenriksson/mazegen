use crate::gen::{Maze, self};

pub fn draw(maze: &Maze, framebuffer: &mut [u32], fb_width: usize, fb_height: usize) {
    for x in 1..maze.width-1 {
        for y in 1..maze.height-1 {
            let idx = gen::to_idx(x, y, maze.width, maze.height);
            draw_cell(x, y, framebuffer, fb_width, fb_height, 0xffffffff);
        }
    }

    for i in maze.carved_cells.iter() {
        let (x,y) = gen::to_x_y(*i, maze.width, maze.height);
        draw_cell(x,y,framebuffer, fb_width, fb_height, 0x0);
    }

    let (start_x, start_y) = gen::to_x_y(maze.start_idx, maze.width, maze.height);
    draw_cell(start_x, start_y, framebuffer, fb_width, fb_height, 0xfaabbf);
}

fn draw_cell(x: usize, y: usize, framebuffer: &mut [u32], fb_width: usize, fb_height: usize, color: u32) {
    let scale = 8;
    let x_start = x*scale;
    let x_end = x_start+scale;
    let y_start = y*scale;
    let y_end = y_start+scale;
    for x in x_start..x_end {
        for y in y_start..y_end {
            framebuffer[x + y * fb_width] = color;
        }
    }
}