use bevy::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

pub const NUM_ROWS: i32 = 30;
pub const NUM_COLS: i32 = 30;

#[derive(Debug, Resource)]
pub struct SearchableGrid {
    pub grid: Vec<Vec<GridCellType>>,
    pub rows: i32,
    pub cols: i32,
}

#[derive(Resource, Component)]
pub struct GridCellLabel;

pub fn get_neighbors(at: (i32,i32), searchable_grid: &mut SearchableGrid) -> Vec<(i32,i32)> {
    let mut neighbors = Vec::<(i32,i32)>::new();

    // Up, Down, Left, Right
    // let dirs = [(0,1),(0,-1),(1,0),(-1,0),(-1,-1),(1,1),(-1,1),(1,-1)];
    let dirs: [(i32,i32); 4] = [(0,1),(0,-1),(1,0),(-1,0)];

    for dir in dirs {
        let (new_row,new_col) = (dir.0 + at.0, dir.1 + at.1);

        if new_col >= 0 && new_col < NUM_COLS && new_row >= 0 && new_row < NUM_ROWS {
            let grid_cell = searchable_grid.get(new_row,new_col);

            if grid_cell != GridCellType::Wall {
                neighbors.push((new_row,new_col));
            }
            continue;
        }
    }

    neighbors
}

#[derive(Debug, Resource, Reflect, Eq, PartialEq,Hash,Clone,Copy)]
pub enum GridCellType {
    Wall,
    Empty,
    Start,
    End,
}

impl SearchableGrid {
    pub fn manhatten_distance(start: (i32,i32), end: (i32,i32)) -> f32 {
        (start.0 as f32 - end.0 as f32).abs() + (start.1 as f32 - end.1 as f32).abs()
    }

    pub fn euclidean_distance(start: (i32,i32), end: (i32,i32)) -> f32 {
        ((start.0 as f32 - end.0 as f32).powf(2.) + (start.1 as f32 - end.1 as f32).powf(2.)).sqrt()
    }

    pub fn reconstruct_path(came_from:HashMap<(i32,i32),(i32,i32)>,current:(i32,i32)) -> Vec<(i32,i32)> {
        let mut total_path = Vec::<(i32,i32)>::new();
        let mut loc_current: (i32,i32) = current;

        while came_from.contains_key(&loc_current) {
            let found_current = came_from.get(&loc_current);

            if found_current == None {
                break;
            }

            total_path.push(*found_current.unwrap());
            loc_current = *found_current.unwrap();
        }

        // Make sure we don't try to draw a path over our green start tile
        total_path.pop();

        total_path.reverse();

        total_path
    }

    pub fn astar_shortest_path(&mut self, start: (i32, i32), goal: (i32, i32), h: fn((i32,i32),(i32,i32)) -> f32) -> Vec<(i32,i32)> {
        // Initialize open set
        let mut open_set = Vec::<(i32,i32)>::new();

        // Append start node to open
        open_set.push(start);

        // Initialize closed set
        let mut closed_set = HashSet::<(i32,i32)>::new();

        // came_from[n] is the cheapest cost of the path starting at start to node n
        let mut came_from = HashMap::<(i32,i32),(i32,i32)>::new();
        
        let mut g_score = HashMap::<(i32,i32),f32>::new();
        g_score.insert(start, 0.);

        let mut f_score = HashMap::<(i32,i32),f32>::new();
        f_score.insert(start, h(start,goal));

        while !open_set.is_empty() {
            let current = *open_set.get(0).unwrap();

            if current == goal {
                return SearchableGrid::reconstruct_path(came_from, current)
            }

            open_set.remove(0);
            closed_set.insert(current);

            for neighbor in get_neighbors(current, self) {
                
                if closed_set.contains(&neighbor) {
                    continue;
                }

                let tentative_g_score = g_score.get(&current).cloned().unwrap_or(f32::INFINITY);

                if tentative_g_score < g_score.get(&neighbor).cloned().unwrap_or(f32::INFINITY) {
                    came_from.insert(neighbor,current);
                    g_score.insert(neighbor, tentative_g_score);
                    f_score.insert(neighbor, tentative_g_score + h(neighbor,goal));

                    if !open_set.contains(&neighbor) {
                        open_set.push(neighbor);
                    }
                }
            }
        }

        Vec::<(i32,i32)>::new()

    }

    fn new_grid_vec() -> Vec<Vec<GridCellType>> {
        let mut grid = Vec::<Vec<GridCellType>>::new();

        for row in 0..NUM_ROWS {
            let row_grid = Vec::<GridCellType>::new();

            grid.push(row_grid);

            for col in 0..NUM_COLS {
                if row == NUM_ROWS - 1 && col == NUM_COLS - 1 {
                    grid[row as usize].push(GridCellType::End);
                } else if row == 0 && col == 0 {
                    grid[row as usize].push(GridCellType::Start);
                } else {
                    grid[row as usize].push(GridCellType::Empty);
                }
            }
        }

        grid
    }

    pub fn new(rows: i32, cols: i32) -> SearchableGrid {
        SearchableGrid { grid: SearchableGrid::new_grid_vec(), rows, cols }
    }

    pub fn get(&mut self, row: i32, col: i32) -> GridCellType {
        self.grid[row as usize][col as usize]
    }

    pub fn set(&mut self, row: i32, col: i32, grid_cell: GridCellType) {
        self.grid[row as usize][col as usize] = grid_cell;
    }

    pub fn reset_grid(&mut self) {
        self.grid = SearchableGrid::new_grid_vec();
    }
}

//                       -
//                       ^
//               (screen_height/2)
//                       |
// | <- (screen_width/2)-0-(screen_width/2) -> |
//                       |
//               (screen_height/2)
//                       v
//                       -
pub fn get_render_position(screen_width: f32, screen_height: f32, row: i32, col: i32) -> (f32, f32) {
    let width_per_row = screen_width / NUM_ROWS as f32;
    let height_per_column = screen_height / NUM_COLS as f32;

    (
        width_per_row * row as f32 - (screen_width / 2.) + 0.5 * width_per_row,
        height_per_column * col as f32 - (screen_height / 2.) + 0.5 * height_per_column,
    )
}

pub fn get_color(grid_cell: GridCellType) -> Color {
    let mut color = Color::WHITE;

    if grid_cell == GridCellType::Wall {
        color = Color::BLACK;
    }

    if grid_cell == GridCellType::End {
        color = Color::RED;
    }

    if grid_cell == GridCellType::Start {
        color = Color::GREEN;
    }

    color
}

// 0,h-------w,h
// |          |
// |          |
// |          |
// |          |
// 0,0--------w,0
pub fn screen_coord_to_row_col(
    x: f32,
    y: f32,
    screen_width: f32,
    screen_height: f32,
) -> (i32, i32) {
    let pixels_per_row = screen_width / NUM_ROWS as f32;
    let pixels_per_column = screen_height / NUM_COLS as f32;

    let row = x / pixels_per_row;
    let col = y / pixels_per_column;

    (row as i32, col as i32)
}