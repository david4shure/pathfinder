use bevy::prelude::*;
use std::collections::HashMap;

pub const NUM_ROWS: i32 = 40;
pub const NUM_COLS: i32 = 40;

#[derive(Debug, Resource, Clone, Copy)]
pub struct SearchableGrid {
    pub grid: [[GridCell; NUM_ROWS as usize]; NUM_COLS as usize],
    pub rows: i32,
    pub cols: i32,
}

#[derive(Resource, Component)]
pub struct GridCellLabel;

#[derive(Debug, Clone, Copy, Resource,Eq,PartialEq,Hash)]
pub struct GridCell {
    pub typ: GridCellType,
    pub row: i32,
    pub col: i32,
}

impl GridCell {
    pub fn get_neighbors(&self, searchable_grid: &mut SearchableGrid) -> Vec<GridCell> {
        let mut neighbors = Vec::<GridCell>::new();

        let row_col_tuple = (self.row, self.col);

        // Up, Down, Left, Right
        // let dirs = [(0,1),(0,-1),(1,0),(-1,0),(-1,-1),(1,1),(-1,1),(1,-1)];
        let dirs = [(0,1),(0,-1),(1,0),(-1,0)];


        for dir in dirs {
            let (new_row,new_col) = (dir.0 + row_col_tuple.0, dir.1 + row_col_tuple.1);

            if new_col >= 0 && new_col < NUM_COLS && new_row >= 0 && new_row < NUM_ROWS {
                let grid_cell = searchable_grid.get(new_row,new_col);

                if grid_cell.typ != GridCellType::Wall {
                    neighbors.push(grid_cell);
                }
                continue;
            }
        }

        neighbors
    }
}

#[derive(Debug, Clone, Copy, Resource, Reflect, Eq, PartialEq,Hash)]
pub enum GridCellType {
    Wall,
    Empty,
    Start,
    End,
}

impl SearchableGrid {
    pub fn manhatten_distance(start: GridCell, end: GridCell) -> f32 {
        (start.row as f32 - end.row as f32).abs() + (start.col as f32 - end.col as f32).abs()
    }

    pub fn euclidean_distance(start: GridCell, end: GridCell) -> f32 {
        ((start.row as f32 - end.row as f32).powf(2.) + (start.col as f32 - end.col as f32).powf(2.)).sqrt()
    }

    pub fn reconstruct_path(came_from:HashMap<GridCell,GridCell>,current:GridCell) -> Vec<GridCell> {
        let mut total_path = Vec::<GridCell>::new();
        let mut loc_current: GridCell = current;

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

    pub fn astar_shortest_path(&mut self, start: GridCell, goal: GridCell, h: fn(GridCell,GridCell) -> f32) -> Vec<GridCell> {
        // Initialize open set
        let mut open_set = Vec::<GridCell>::new();

        // Append start node to open
        open_set.push(self.get(0,0));

        // Initialize closed set
        let mut closed_set = Vec::<GridCell>::new();

        // came_from[n] is the cheapest cost of the path starting at start to node n
        let mut came_from = HashMap::<GridCell,GridCell>::new();
        
        // let a = map.get(&'a').cloned().unwrap_or(0);

        let mut g_score = HashMap::<GridCell,f32>::new();
        g_score.insert(start, 0.);

        let mut f_score = HashMap::<GridCell,f32>::new();
        f_score.insert(start, h(start,goal));

        while !open_set.is_empty() {
            let current = *open_set.get(0).unwrap();

            if current == goal {
                return SearchableGrid::reconstruct_path(came_from, current)
            }

            open_set.remove(0);
            closed_set.push(current);

            for neighbor in current.get_neighbors(self) {
                
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

        Vec::<GridCell>::new()

    }

    fn new_grid_vec() -> [[GridCell; NUM_ROWS as usize]; NUM_COLS as usize] {
        let mut grid = [[GridCell{typ: GridCellType::Empty, row: 0, col: 0}; NUM_COLS as usize]; NUM_ROWS as usize];

        for row in 0..NUM_ROWS {
            for col in 0..NUM_COLS {
                let mut grid_cell = grid[row as usize][col as usize];
                grid_cell.row = row;
                grid_cell.col = col;

                if row == NUM_ROWS - 1 && col == NUM_COLS - 1 {
                    grid_cell.typ = GridCellType::End;
                }

                if row == 0 && col == 0 {
                    grid_cell.typ = GridCellType::Start;
                }

                grid[row as usize][col as usize] = grid_cell;
            }
        }

        grid
    }

    pub fn new(rows: i32, cols: i32) -> SearchableGrid {
        SearchableGrid { grid: SearchableGrid::new_grid_vec(), rows, cols }
    }

    pub fn get(&mut self, row: i32, col: i32) -> GridCell {
        self.grid[row as usize][col as usize]
    }

    pub fn set(&mut self, row: i32, col: i32, grid_cell: GridCell) {
        self.grid[row as usize][col as usize] = grid_cell;
    }

    pub fn reset_grid(&mut self) {
        self.grid = SearchableGrid::new_grid_vec();
    }
}

impl GridCell {
    //                       -
    //                       ^
    //               (screen_height/2)
    //                       |
    // | <- (screen_width/2)-0-(screen_width/2) -> |
    //                       |
    //               (screen_height/2)
    //                       v
    //                       -
    pub fn get_render_position(&self, screen_width: f32, screen_height: f32) -> (f32, f32) {
        let width_per_row = screen_width / NUM_ROWS as f32;
        let height_per_column = screen_height / NUM_COLS as f32;

        (
            width_per_row * self.row as f32 - (screen_width / 2.) + 0.5 * width_per_row,
            height_per_column * self.col as f32 - (screen_height / 2.) + 0.5 * height_per_column,
        )
    }

    pub fn get_color(&self) -> Color {
        let mut color = Color::WHITE;

        if self.typ == GridCellType::Wall {
            color = Color::BLACK;
        }

        if self.typ == GridCellType::End {
            color = Color::RED;
        }

        if self.typ == GridCellType::Start {
            color = Color::GREEN;
        }

        color
    }
}

// 0,h-------w,h
// |          |
// |          |
// |          |
// |          |
// 0,0--------w,0
pub fn screen_coord_to_row_col(
    x: i32,
    y: i32,
    num_rows: i32,
    num_cols: i32,
    screen_width: i32,
    screen_height: i32,
) -> (i32, i32) {
    let pixels_per_row = screen_width / num_rows;
    let pixels_per_column = screen_height / num_cols;

    let row = x / pixels_per_row;
    let col = y / pixels_per_column;

    (row, col)
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
