#[derive(Debug)]
pub struct MazeCell {
    pub idx: usize,
    pub wall_north: bool,
    pub wall_south: bool,
    pub wall_east: bool,
    pub wall_west: bool,
    visited: bool,
}

pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<MazeCell>,
}

impl Maze {
    /// Generate a new maze with given width and height
    pub fn generate(width: usize, height: usize, use_recursive: bool) -> Maze {
        let start_idx = rand::random::<usize>() % width*height;
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

        if use_recursive {
            recursive_generate_step(start_idx, width, height, cells.as_mut_slice());
        } else {
            let mut visited_stack = Vec::new();
            cells[start_idx].visited = true;
            visited_stack.push(start_idx);

            while !visited_stack.is_empty() {
                let cell_idx = visited_stack.pop().unwrap();
                let (x,y) = to_x_y(cell_idx, width, height);
                
                let nbors: Vec<usize> = get_neighbors(cell_idx, width, height)
                    .iter()
                    .filter_map(|&x| {
                        if let Some(x) = x {
                            let ret = if !cells[x].visited {
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
                    visited_stack.push(cell_idx);

                    let index = rand::random::<usize>() % nbors.len();
                    let nbor_cell_idx = nbors[index];
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

                    cells[nbor_cell_idx].visited = true;
                    visited_stack.push(nbor_cell_idx);
                }
            }
        }
        
        Maze {
            width,
            height,
            cells,
        }
    }
}

fn recursive_generate_step(cell_idx: usize, width: usize, height: usize, cells: &mut [MazeCell]) {
    cells[cell_idx].visited = true;

    let (x,y) = to_x_y(cell_idx, width, height);
    let mut nbors: Vec<usize> = get_neighbors(cell_idx, width, height)
        .iter()
        .filter_map(|&x| x)
        .collect();
    
    while !nbors.is_empty() {
        let index = rand::random::<usize>() % nbors.len();
        let nbor_cell_idx = nbors[index];
        if cells[nbor_cell_idx].visited {
            nbors.remove(index);
            continue;
        }

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

        recursive_generate_step(nbor_cell_idx, width, height, cells);
        nbors.remove(index);
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

    let south = if y < height - 1 {
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