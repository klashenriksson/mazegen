//  Copyright (c) Klas Henriksson 2022.
//  All rights reserved.

mod gen;
mod viz;

use gen::{Maze, RecursiveBacktracker, MazeGenerator, RecursiveDivision};
use minifb::{Key, Window, WindowOptions};
use viz::{MazeVizDescritptor, Framebuffer};

const WIDTH: usize = 640*2;
const HEIGHT: usize = 360*2;

#[derive(Debug, Clone, Copy)]
enum GeneratorType {
    RecursiveBacktracker = 0,
    RecursiveDivision
}

fn create_generator(ty: GeneratorType) -> Box<dyn MazeGenerator> {
    match ty {
        GeneratorType::RecursiveBacktracker => Box::new(RecursiveBacktracker::new()),
        GeneratorType::RecursiveDivision => Box::new(RecursiveDivision::new(100))
    }
}

fn main() {
    let mut framebuffer = Framebuffer {
        buffer: vec![0; WIDTH * HEIGHT],
        width: WIDTH,
        height: HEIGHT
    };

    let mut window = Window::new(
        "Maze Gen",
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

    let mut current_type = GeneratorType::RecursiveDivision;
    let mut generator = create_generator(current_type);
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
        } else if window.is_key_pressed(Key::C, minifb::KeyRepeat::No) {
            current_type = match current_type {
                GeneratorType::RecursiveBacktracker => GeneratorType::RecursiveDivision,
                GeneratorType::RecursiveDivision => GeneratorType::RecursiveBacktracker
            };

            should_regen = true;
        }

        if window.is_key_pressed(Key::P, minifb::KeyRepeat::No) {
            step_interval /= 2.0;
        } else if window.is_key_pressed(Key::O, minifb::KeyRepeat::No) {
            step_interval *= 2.0;
        }

        if should_regen {
            maze = Maze::empty(maze_width, maze_height);
            generator = create_generator(current_type);
            generator.initialize(&mut maze);

            viz_desc.rescale(maze_width, maze_height);
        }

        let title = format!(
            "MazeGen. Controls: 1/2: Double/Half maze size. Curr: {}x{} R: Regen maze. P/O: (In)/(De)crease step freq. Curr: {}/s C: Cycle gen algo. Curr: {:?}",
            maze.width, maze.height, 1.0/step_interval, current_type
        );
        window.set_title(title.as_str());

        let now = std::time::SystemTime::now();
        let dur = now.duration_since(last_time).unwrap().as_secs_f64();
        if dur > step_interval {
            generator.step(&mut maze);
            last_time += std::time::Duration::from_secs_f64(dur);
        }

        framebuffer.clear(0x0);
        framebuffer.draw_maze(&maze, &viz_desc);
        window
            .update_with_buffer(&framebuffer.buffer.as_slice(), framebuffer.width, framebuffer.height)
            .unwrap();
    }
}