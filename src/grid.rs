use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct SearchableGrid {
    pub grid: Vec<Vec<GridCell>>,
}

#[derive(Resource, Component)]
pub struct GridCellLabel;

#[derive(Debug, Clone, Copy, Resource)]
pub struct GridCell {
    pub typ: GridCellType,
    pub row: i32,
    pub col: i32,
}

#[derive(Debug, Clone, Copy, Resource, Reflect, Eq, PartialEq)]
pub enum GridCellType {
    Wall,
    Empty,
}

impl SearchableGrid {
    pub fn new(rows: i32, cols: i32) -> SearchableGrid {
        // Initialize
        let mut grid: Vec<Vec<GridCell>> = Vec::new();

        for i in 0..rows {
            let slab: Vec<GridCell> = Vec::new();
            grid.push(slab);

            for j in 0..cols {
                grid[i as usize].push(GridCell {
                    typ: GridCellType::Empty,
                    row: i,
                    col: j,
                });
            }
        }

        SearchableGrid { grid }
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
    pub fn get_render_position(
        &self,
        screen_width: f32,
        screen_height: f32,
        rows: i32,
        cols: i32,
    ) -> (f32, f32) {
        let width_per_row = screen_width / rows as f32;
        let height_per_column = screen_height / cols as f32;

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
    x: f32,
    y: f32,
    rows: i32,
    cols: i32,
    screen_width: i32,
    screen_height: i32,
) {
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
