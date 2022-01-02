struct Maze {
    width: usize,
    height: usize,

    /// Stores the state of the walls for this maze. true => wall present, false => no wall.
    walls: Vec<bool>
}

impl Maze {
    /// Generate a new maze with given width and height
    pub fn generate(width: usize, height: usize) {
        let inner_cells = (width - 2)*(height-2);
        let edge_cells = width * 2 + height * 2 - 4;
        let walls: Vec<bool> = Vec::with_capacity(inner_cells + edge_cells);
        let visited_stack = Vec::new();
    }
}

fn generate_step(cell_idx: usize, width: usize, height: usize, walls: &mut [bool], visited_stack: &mut Vec<usize>) {
    visited_stack.push(cell_idx);
    let nbors = get_neighbors(cell_idx, width, height);
    let unvisited_nbors: Vec<usize> = nbors
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
    }
    
}

fn to_x_y(cell_idx: usize, width: usize, height: usize) -> (usize, usize) {
    (cell_idx % width, cell_idx / width)
}

fn to_idx(x: usize, y: usize, width: usize, height: usize) -> usize {
    y * width + x
}

/// Returns the neighbor indices of cell with index *cell_idx*. Returns in order CCW order starting from north
/// (i.e) north, east, south, west
fn get_neighbors(cell_idx: usize, width: usize, height: usize) -> [Option<usize>;4] {

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
        east,
        south,
        west
    ]
}