
mod gen;
mod viz;

use gen::Maze;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640*2;
const HEIGHT: usize = 360*2;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
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

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut maze_width = 5;
    let mut maze_height = 5;
    let mut maze = Maze::generate(maze_width, maze_height, false);

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

        if should_regen {
            window.set_title("Regenerating..");
            maze = Maze::generate(maze_width, maze_height, false);
            window.set_title(title);
        }

        viz::draw(&maze, buffer.as_mut_slice(), WIDTH, HEIGHT);
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}