
mod gen;
mod viz;

use gen::{Maze, RecursiveBacktracker, MazeGenerator, RecursiveDivision};
use minifb::{Key, Window, WindowOptions};
use viz::{MazeVizDescritptor, Framebuffer};

const WIDTH: usize = 640*2;
const HEIGHT: usize = 360*2;

fn main() {
    let mut framebuffer = Framebuffer {
        buffer: vec![0; WIDTH * HEIGHT],
        width: WIDTH,
        height: HEIGHT
    };

    let title = "MazeGen. R to regenerate, 1 to double maze size, 2 to halve maze size";

    let mut window = Window::new(
        title,
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~12 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut maze_width = 5;
    let mut maze_height = 5;
    let mut maze = Maze::empty(maze_width, maze_height);

    let mut viz_desc = MazeVizDescritptor::new(50,50,WIDTH-100, HEIGHT-100, maze.width, maze.height);

    let mut generator = RecursiveDivision::new(100);
    generator.initialize(&mut maze);

    let mut step_interval = 0.1;
    let mut last_time = std::time::SystemTime::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut should_regen = false;

        if window.is_key_pressed(Key::Key1, minifb::KeyRepeat::No) {
            should_regen = true;
            maze_width *= 2;
            maze_height *= 2;
        } else if window.is_key_pressed(Key::Key2, minifb::KeyRepeat::No) {
            should_regen = true;
            maze_width = 1.max(maze_width/2);
            maze_height = 1.max(maze_height/2);
        } else if window.is_key_pressed(Key::R, minifb::KeyRepeat::No) {
            should_regen = true;
        }

        if window.is_key_pressed(Key::P, minifb::KeyRepeat::No) {
            step_interval /= 2.0;
        } else if window.is_key_pressed(Key::O, minifb::KeyRepeat::No) {
            step_interval *= 2.0;
        }

        if should_regen {
            maze = Maze::empty(maze_width, maze_height);
            generator = RecursiveDivision::new(100);
            generator.initialize(&mut maze);

            viz_desc.rescale(maze_width, maze_height);
        }

        let now = std::time::SystemTime::now();
        let dur = now.duration_since(last_time).unwrap().as_secs_f64();
        if dur > step_interval {
            loop {
                if generator.step(&mut maze) {
                    break;
                }
            }
            last_time += std::time::Duration::from_secs_f64(dur);
        }

        framebuffer.clear(0x0);
        framebuffer.draw_maze(&maze, &viz_desc);
        window
            .update_with_buffer(&framebuffer.buffer.as_slice(), framebuffer.width, framebuffer.height)
            .unwrap();
    }
}