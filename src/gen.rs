//  Copyright (c) Klas Henriksson 2022.
//  All rights reserved.

use std::collections::VecDeque;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Dir {
    East,
    North,
    West,
    South
}

#[derive(Debug)]
pub struct MazeCell {
    pub idx: usize,
    pub wall_north: bool,
    pub wall_south: bool,
    pub wall_east: bool,
    pub wall_west: bool,
    pub visited: bool,
}

pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<MazeCell>,
}

impl Maze {
    /// Constructs a empty maze.
    pub fn empty(width: usize, height: usize) -> Maze {
        let mut cells = Vec::with_capacity(width*height);
        for i in 0..width*height {
            cells.push(MazeCell {
                idx: i,
                wall_north: true,
                wall_south: true,
                wall_east: true,
                wall_west: true,
                visited: false
            });
        }

        Maze {
            width,
            height,
            cells,
        }
    }
}

pub trait MazeGenerator {
    /// Perform a step of the maze generator.
    fn step(&mut self, maze: &mut Maze);
    /// Returns true if the generation is complete. Othewerise false.
    fn is_finished(&self) -> bool;
    
    fn initialize(&mut self, maze: &mut Maze);
}

pub struct RecursiveBacktracker {
    gen_iteration: usize,
    visited_stack: Vec<usize>,
    finished: bool,
}

impl RecursiveBacktracker {
    pub fn new() -> Self {
        Self {
            gen_iteration: 0,
            finished: false,
            visited_stack: vec![]
        }
    }
}

impl MazeGenerator for RecursiveBacktracker {
    fn step(&mut self, maze: &mut Maze) {
        if self.visited_stack.is_empty() { // If the visited stack is empty and we arent on gen 0, we are done.
            self.finished = true;
            return;
        }

        //  Perform a step of the recursive backtracking algo.

        //  Get most recent cell
        let cell_idx = self.visited_stack.pop().unwrap();

        //  Fetch neighboring cells that havent been visited
        let nbors: Vec<usize> = get_neighbors(cell_idx, maze.width, maze.height)
            .iter()
            .filter_map(|&x| {
                if let Some(x) = x {
                    let ret = if !maze.cells[x].visited {
                        Some(x)
                    } else {
                        None
                    };

                    ret
                } else {
                    None
                }
            })
            .collect();

        if !nbors.is_empty() {
            //  Add ourselves back into the stack to facillitate backtracking
            self.visited_stack.push(cell_idx);

            //  Carve out a path to a random unvisited neighbor and add neighbor to the stack
            let index = self.gen_iteration % nbors.len();
            let nbor_cell_idx = nbors[index];
            remove_wall(cell_idx, nbor_cell_idx, maze.width, maze.cells.as_mut_slice());
            maze.cells[nbor_cell_idx].visited = true;
            self.visited_stack.push(nbor_cell_idx);
            self.gen_iteration += 1;
        } else {
            //  If we dont have any neighbors we backtrack to previous cell until we have one!
            //  We do this here to avoid steps that simply pop the visited stack.
            self.step(maze);
        }
    }

    fn is_finished(&self) -> bool {
        self.finished
    }

    fn initialize(&mut self, maze: &mut Maze) {
        // Add all walls
        for cell in maze.cells.iter_mut() {
            cell.wall_east = true;
            cell.wall_west = true;
            cell.wall_south = true;
            cell.wall_north = true;
        }

        let start_idx = rand::random::<usize>() % maze.cells.len();
        maze.cells[start_idx].visited = true;
        self.visited_stack.push(start_idx);
    }
}

#[derive(Debug)]
struct Field {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

#[derive(Debug)]
struct FieldIntersection {
    start: usize,
    end: usize,
    split_coord: usize,
}

impl Field {
    ///     Tries to subdivide itself into two fields. If *horizontally* is true it divides the field horizontally,
    ///     otherwise vertically.
    pub fn divide(self, horizontally: bool) -> Option<(Field, Field, FieldIntersection)> {
        if self.width < 2 || self.height < 2 {
            return None;
        }

        let r = rand::random::<usize>();
        if horizontally {
            let split_coord = r % (self.height - 1);

            let top_field = Field {
                x: self.x,
                y: self.y,
                width: self.width,
                height: split_coord + 1
            };
            
            let bot_field = Field {
                x: self.x,
                y: top_field.y + top_field.height,
                width: self.width,
                height: self.height - top_field.height
            };

            let intersection = FieldIntersection {
                start: self.x,
                end: self.x + self.width,
                split_coord: self.y + split_coord
            };

            Some((top_field, bot_field, intersection))
        } else {
            let split_coord = r % (self.width - 1);

            let left_field = Field {
                x: self.x,
                y: self.y,
                width: split_coord + 1,
                height: self.height
            };

            let right_field = Field {
                x: left_field.x + left_field.width,
                y: self.y,
                width: self.width - left_field.width,
                height: self.height
            };

            let intersection = FieldIntersection {
                start: self.y,
                end: self.y + self.height,
                split_coord: self.x + split_coord
            };

            Some((left_field, right_field, intersection))
        }
    }
}

pub struct RecursiveDivision {
    gen_iteration: usize,
    max_subdivides: usize,
    fields: VecDeque<Field>,
}

impl RecursiveDivision {
    pub fn new(max_subdivides: usize) -> Self {
        Self {
            gen_iteration: 0,
            max_subdivides,
            fields: VecDeque::new()
        }
    }
}

impl MazeGenerator for RecursiveDivision {
    fn step(&mut self, maze: &mut Maze) {
        if self.gen_iteration == self.max_subdivides {
            return;
        }

        //  Perform one subdivision.
        //  Note that this subdivides ALL current fields (thats the while loop) (this is what one considers a iteration)
        //  This is done to speed up the visualization and not have to watch each subfield subdivide itself one at a time
        let field_count = self.fields.len();
        let mut processed = 0;
        while processed < field_count {

            //  Fetch oldest field
            let field = self.fields.pop_front().unwrap();

            //  Determine if we should subdivide the field horizontally or vertically.
            //  If the field's height is larget than its width, we split it horizontally to
            //  construct more interesting mazes. Similarily if its width is greather than its height.
            let horiz = if field.height > field.width { 
                true
            } else if field.width > field.height {
                false
            } else {
                rand::random::<usize>() % 2 == 0
            };

            //  Try to subdivide it
            if let Some((field_1, field_2, intersection)) = field.divide(horiz) {

                //  Choose where to make a gap in the wall
                let skip = intersection.start + rand::random::<usize>() % (intersection.end-intersection.start);

                //  Add wall to construct the division
                if horiz {
                    let y = intersection.split_coord;
                    for x in intersection.start..intersection.end {
                        if x == skip {
                            continue;
                        }

                        let idx = to_idx(x, y, maze.width);
                        let nbor_idx = to_idx(x, y+1, maze.width);
                        add_wall(idx, nbor_idx, maze.width, maze.cells.as_mut_slice());
                    }

                } else {
                    let x = intersection.split_coord;
                    for y in intersection.start..intersection.end {
                        if y == skip {
                            continue;
                        }

                        let idx = to_idx(x, y, maze.width);
                        let nbor_idx = to_idx(x+1, y, maze.width);
                        add_wall(idx, nbor_idx, maze.width, maze.cells.as_mut_slice());
                    }
                }

                //  Add the two newly created fields into the queue
                self.fields.push_back(field_1);
                self.fields.push_back(field_2);
            }
            processed += 1;
        }

        self.gen_iteration += 1;
    }

    fn is_finished(&self) -> bool {
        self.gen_iteration == self.max_subdivides
    }

    fn initialize(&mut self, maze: &mut Maze) {
        //  Remove all walls
        for cell in maze.cells.iter_mut() {
            cell.wall_east = false;
            cell.wall_west = false;
            cell.wall_south = false;
            cell.wall_north = false;
        }

        self.fields.push_front(Field {
            x: 0,
            y: 0,
            width: maze.width,
            height: maze.height
        });
    }
}

pub struct BinaryTree {
    curr_idx: usize,
    end_idx: usize,
    dir_1: Dir,
    dir_2: Dir
}

impl BinaryTree {
    /// Construct a new BinaryTree maze generator.
    /// Note: Currently no direction validation is done, except for if dir_1 == dir_2
    /// So choosing direction along the same axis is allowed, but produces very boring mazes.
    pub fn new(dir_1: Dir, dir_2: Dir) -> Self {

        if dir_1 == dir_2 {
            panic!("dir_1 and dir_2 must not be equal!");
        }

        Self {
            curr_idx: 0,
            end_idx: 0,
            dir_1,
            dir_2,
        }
    }
}

impl MazeGenerator for BinaryTree {
    fn step(&mut self, maze: &mut Maze) {
        if self.is_finished() {
            return;
        }

        let r = rand::random::<usize>() % 2;
        let (mut dir, other_dir) = if r == 0 {
            (self.dir_1, self.dir_2)
        } else {
            (self.dir_2, self.dir_1)
        };
        
        let (x,y) = to_x_y(self.curr_idx, maze.width);
        
        //  Bound check if we are on the boundary
        if dir == Dir::North && y == 0 {
            if (x == 0 && other_dir == Dir::West) || (x == maze.width - 1 && other_dir == Dir::East) {
                self.curr_idx += 1;
                return;
            } else {
                dir = other_dir;
            }
        }
        else if dir == Dir::South && y == maze.height - 1 {
            if (x == 0 && other_dir == Dir::West) || (x == maze.width - 1 && other_dir == Dir::East) {
                self.curr_idx += 1;
                return;
            } else {
                dir = other_dir;
            }
        }
        else if dir == Dir::West && x == 0 {
            if (y == 0 && other_dir == Dir::North) || (y == maze.height - 1 && other_dir == Dir::South) {
                self.curr_idx += 1;
                return;
            } else {
                dir = other_dir
            }
        }
        else if dir == Dir::East && x == maze.width - 1 {
            if (y == 0 && other_dir == Dir::North) || (y == maze.height - 1 && other_dir == Dir::South) {
                self.curr_idx += 1;
                return;
            } else {
                dir = other_dir
            }
        }

        let (nbor_x, nbor_y) = match dir {
            Dir::North => (x, y - 1),
            Dir::South => (x, y + 1),
            Dir::East => (x + 1, y),
            Dir::West => (x - 1, y)
        };
        remove_wall(self.curr_idx, to_idx(nbor_x, nbor_y, maze.width), maze.width, maze.cells.as_mut_slice());
        self.curr_idx+=1;
    }

    fn is_finished(&self) -> bool {
        self.curr_idx > self.end_idx
    }

    fn initialize(&mut self, maze: &mut Maze) {
        self.end_idx = maze.cells.len()-1;

        // Add all walls
        for cell in maze.cells.iter_mut() {
            cell.wall_east = true;
            cell.wall_west = true;
            cell.wall_south = true;
            cell.wall_north = true;
        }
    }
}

/// Removes wall between cell_idx and nbor_cell_idx
fn remove_wall(cell_idx: usize, nbor_cell_idx: usize, width: usize, cells: &mut [MazeCell]) {
    let (x,y) = to_x_y(cell_idx, width);
    let (nbor_x, nbor_y) = to_x_y(nbor_cell_idx, width);

    if nbor_x > x {
        cells[cell_idx].wall_east = false;
        cells[nbor_cell_idx].wall_west = false;
    }
    if nbor_x < x {
        cells[cell_idx].wall_west = false;
        cells[nbor_cell_idx].wall_east = false;
    }
    if nbor_y < y {
        cells[cell_idx].wall_north = false;
        cells[nbor_cell_idx].wall_south = false;
    }
    if nbor_y > y {
        cells[cell_idx].wall_south = false;
        cells[nbor_cell_idx].wall_north = false;
    }
}

/// Adds a wall between cell_idx and nbor_cell_idx
fn add_wall(cell_idx: usize, nbor_cell_idx: usize, width: usize, cells: &mut [MazeCell]) {
    let (x,y) = to_x_y(cell_idx, width);
    let (nbor_x, nbor_y) = to_x_y(nbor_cell_idx, width);

    if nbor_x > x {
        cells[cell_idx].wall_east = true;
        cells[nbor_cell_idx].wall_west = true;
    }
    if nbor_x < x {
        cells[cell_idx].wall_west = true;
        cells[nbor_cell_idx].wall_east = true;
    }
    if nbor_y < y {
        cells[cell_idx].wall_north = true;
        cells[nbor_cell_idx].wall_south = true;
    }
    if nbor_y > y {
        cells[cell_idx].wall_south = true;
        cells[nbor_cell_idx].wall_north = true;
    }
}

pub fn to_x_y(cell_idx: usize, width: usize) -> (usize, usize) {
    (cell_idx % width, cell_idx / width)
}

pub fn to_idx(x: usize, y: usize, width: usize) -> usize {
    y * width + x
}

/// Returns the neighbor indices of cell with index *cell_idx*. Returns in order CCW order starting from north
/// (i.e) north, west, south, east
pub fn get_neighbors(cell_idx: usize, width: usize, height: usize) -> [Option<usize>;4] {
    let (x,y) = to_x_y(cell_idx, width);

    let north = if y > 0 {
        Some(to_idx(x,y-1, width))
    } else {
        None
    };

    let south = if y < height - 1 {
        Some(to_idx(x,y+1, width))
    } else {
        None
    };

    let west = if x > 0 {
        Some(to_idx(x-1, y, width))
    } else {
        None
    };

    let east = if x < width - 1 {
        Some(to_idx(x+1, y, width))
    } else {
        None
    };

    [
        north,
        west,
        south,
        east,
    ]
}