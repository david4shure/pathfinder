use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct SearchableGrid {
    pub grid: Vec<GridCell>,
    pub rows: i32,
    pub cols: i32,
}

#[derive(Resource, Component)]
pub struct GridCellLabel;

#[derive(Debug, Clone, Copy, Resource)]
pub struct GridCell {
    pub typ: GridCellType,
    pub row: i32,
    pub col: i32,
    pub rows: i32,
    pub cols: i32,
}

#[derive(Debug, Clone, Copy, Resource, Reflect, Eq, PartialEq)]
pub enum GridCellType {
    Wall,
    Empty,
}

impl SearchableGrid {
    pub fn new(rows: i32, cols: i32) -> SearchableGrid {
        // Initialize

        let total_flat_size = rows * cols;

        let mut grid: Vec<GridCell> = Vec::with_capacity(total_flat_size as usize);

        for i in 0..total_flat_size {
            grid.push(GridCell {
                typ: GridCellType::Empty,
                row: i / rows,
                col: i % cols,
                rows,
                cols,
            });
        }
        SearchableGrid { grid, rows, cols }
    }

    pub fn get(&self, row: i32, col: i32) -> GridCell {
        self.grid[(row * self.rows + col) as usize]
    }

    pub fn set(&mut self, row: i32, col: i32, grid_cell: GridCell) {
        self.grid[(row * self.rows + col) as usize] = grid_cell
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
        let width_per_row = screen_width / self.rows as f32;
        let height_per_column = screen_height / self.cols as f32;

        (
            width_per_row * self.row as f32 - (screen_width / 2.),
            height_per_column * self.col as f32 - (screen_height / 2.),
        )
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
