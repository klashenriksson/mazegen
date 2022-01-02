enum Dir {
    North,
    South,
    West,
    East
}

#[derive(Debug)]
pub struct MazeCell {
    pub idx: usize,
    pub wall_north: bool,
    pub wall_south: bool,
    pub wall_east: bool,
    pub wall_west: bool,
}

pub struct Maze {
    pub width: usize,
    pub height: usize,

    /// Stores the state of the walls for this maze. true => wall present, false => no wall.
    pub start_idx: usize,
    pub cells: Vec<MazeCell>,

    pub carved_cells: Vec<usize>,
}

impl Maze {
    /// Generate a new maze with given width and height
    pub fn generate(width: usize, height: usize) -> Maze {

        let mut visited_stack = Vec::new();

        let start_idx = rand::random::<usize>() % width*height;
        let mut cells = Vec::with_capacity(width*height);
        for i in 0..width*height {
            cells.push(MazeCell {
                idx: i,
                wall_north: true,
                wall_south: true,
                wall_east: true,
                wall_west: true
            });
        }
        generate_step(start_idx, width, height, &mut visited_stack, cells.as_mut_slice());

        println!("adad: {:?}", cells);
        let mut queue = vec![start_idx];
        let mut carved_cells = vec![];
        while !queue.is_empty() {
            let idx = queue.pop().unwrap();
            let (x,y) = to_x_y(idx, width, height);
            carved_cells.push(idx);

            if !cells[idx].wall_east {
                let nbor_idx = to_idx(x+1, y, width, height);
                if !carved_cells.contains(&nbor_idx) {
                    queue.push(nbor_idx);
                }
            }

            if !cells[idx].wall_west {
                let nbor_idx = to_idx(x-1, y, width, height);
                if !carved_cells.contains(&nbor_idx) {
                    queue.push(nbor_idx);
                }
            }

            if !cells[idx].wall_north {
                let nbor_idx = to_idx(x, y-1, width, height);
                if !carved_cells.contains(&nbor_idx) {
                    queue.push(nbor_idx);
                }
            }

            if !cells[idx].wall_south {
                let nbor_idx = to_idx(x, y+1, width, height);
                if !carved_cells.contains(&nbor_idx) {
                    queue.push(nbor_idx);
                }
            }
        }

        Maze {
            width,
            height,
            cells,
            start_idx,
            carved_cells
        }
    }
}

fn generate_step(cell_idx: usize, width: usize, height: usize, visited_stack: &mut Vec<usize>, cells: &mut [MazeCell]) {
    visited_stack.push(cell_idx);
    let (x,y) = to_x_y(cell_idx, width, height);
    println!("x: {}, y: {}", x,y);

    let nbors = get_neighbors(cell_idx, width, height);
    let mut unvisited_nbors: Vec<usize> = nbors
        .iter()
        .filter_map(|&x| {
            if let Some(x) = x {
                let ret = if !visited_stack.contains(&x) {
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
    
    while !unvisited_nbors.is_empty() {
        let index = rand::random::<usize>() % unvisited_nbors.len();
        let nbor_cell_idx = unvisited_nbors[index];
        let (nbor_x, nbor_y) = to_x_y(nbor_cell_idx, width, height);

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
        unvisited_nbors.remove(index);

        generate_step(nbor_cell_idx, width, height, visited_stack, cells);
    }
    
}

pub fn to_x_y(cell_idx: usize, width: usize, height: usize) -> (usize, usize) {
    (cell_idx % width, cell_idx / width)
}

pub fn to_idx(x: usize, y: usize, width: usize, height: usize) -> usize {
    y * width + x
}

/// Returns the neighbor indices of cell with index *cell_idx*. Returns in order CCW order starting from north
/// (i.e) north, west, south, east
pub fn get_neighbors(cell_idx: usize, width: usize, height: usize) -> [Option<usize>;4] {

    let (x,y) = to_x_y(cell_idx, width, height);

    let north = if y > 0 {
        Some(to_idx(x,y-1, width, height))
    } else {
        None
    };

    let south = if y < height - 1{
        Some(to_idx(x,y+1, width, height))
    } else {
        None
    };

    let west = if x > 0 {
        Some(to_idx(x-1, y, width, height))
    } else {
        None
    };

    let east = if x < width - 1 {
        Some(to_idx(x+1, y, width, height))
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