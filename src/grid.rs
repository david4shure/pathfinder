use bevy::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};

pub const NUM_ROWS: i32 = 40;
pub const NUM_COLS: i32 = 40;

type HeuristicFunc = fn((i32, i32), (i32, i32)) -> f32;

#[derive(Debug, Resource)]
pub struct SearchableGrid {
    pub grid: Vec<Vec<GridCellType>>,
    pub start: (i32,i32),
    pub end: (i32,i32),
    pub rows: i32,
    pub cols: i32,
}

#[derive(Resource, Component)]
pub struct GridCellLabel;

pub fn get_neighbors(at: (i32, i32), searchable_grid: &mut SearchableGrid) -> Vec<(i32, i32)> {
    let mut neighbors = Vec::<(i32, i32)>::new();

    // Up, Down, Left, Right
    let dirs = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (-1, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
    ];

    // Use this line if you only want to move in the 4 cardinal directions
    // let dirs: [(i32,i32); 4] = [(0,1),(0,-1),(1,0),(-1,0)];

    for dir in dirs {
        let (new_row, new_col) = (dir.0 + at.0, dir.1 + at.1);

        if new_col >= 0 && new_col < NUM_COLS && new_row >= 0 && new_row < NUM_ROWS {
            let grid_cell = searchable_grid.grid[new_row as usize][new_col as usize];

            if grid_cell != GridCellType::Wall {
                neighbors.push((new_row, new_col));
            }
        }
    }

    neighbors
}

#[derive(Debug, Resource, Reflect, Eq, PartialEq, Hash, Clone, Copy)]
pub enum GridCellType {
    Wall,
    Empty,
    Start,
    End,
}

impl SearchableGrid {
    #[allow(dead_code)]
    pub fn manhatten_distance(start: (i32, i32), end: (i32, i32)) -> f32 {
        (start.0 as f32 - end.0 as f32).abs() + (start.1 as f32 - end.1 as f32).abs()
    }

    #[allow(dead_code)]
    pub fn euclidean_distance(start: (i32, i32), end: (i32, i32)) -> f32 {
        ((start.0 as f32 - end.0 as f32).powf(2.) + (start.1 as f32 - end.1 as f32).powf(2.)).sqrt()
    }

    pub fn reconstruct_path(
        came_from: HashMap<(i32, i32), (i32, i32)>,
        current: (i32, i32),
    ) -> Vec<(i32, i32)> {
        let mut total_path = Vec::<(i32, i32)>::new();
        let mut loc_current: (i32, i32) = current;

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

    pub fn astar_shortest_path(
        &mut self,
        start: (i32, i32),
        goal: (i32, i32),
        h: HeuristicFunc,
    ) -> Vec<(i32, i32)> {
        // Initialize open set
        let mut open_set = VecDeque::<(i32, i32)>::new();

        // Append start node to open
        open_set.push_back(start);

        // Initialize closed set
        let mut closed_set = HashSet::<(i32, i32)>::new();

        // came_from[n] is the cheapest cost of the path starting at start to node n
        let mut came_from = HashMap::<(i32, i32), (i32, i32)>::new();

        let mut g_score = HashMap::<(i32, i32), f32>::new();
        g_score.insert(start, 0.);

        let mut f_score = HashMap::<(i32, i32), f32>::new();
        f_score.insert(start, h(start, goal));

        while !open_set.is_empty() {
            let current = open_set.pop_front().unwrap();

            if current == goal {
                return SearchableGrid::reconstruct_path(came_from, current);
            }

            closed_set.insert(current);

            for neighbor in get_neighbors(current, self) {
                if closed_set.contains(&neighbor) {
                    continue;
                }

                let tentative_g_score = g_score.get(&current).cloned().unwrap_or(f32::INFINITY);

                if tentative_g_score < g_score.get(&neighbor).cloned().unwrap_or(f32::INFINITY) {
                    came_from.insert(neighbor, current);
                    g_score.insert(neighbor, tentative_g_score);
                    f_score.insert(neighbor, tentative_g_score + h(neighbor, goal));

                    open_set.push_back(neighbor);
                }
            }
        }

        Vec::<(i32, i32)>::new()
    }

    fn new_grid_vec(start: (i32,i32), end: (i32,i32)) -> Vec<Vec<GridCellType>> {
        let mut grid = Vec::<Vec<GridCellType>>::new();

        for row in 0..NUM_ROWS {
            let row_grid = Vec::<GridCellType>::new();

            grid.push(row_grid);

            for col in 0..NUM_COLS {
                if row == end.0 && col == end.1 {
                    grid[row as usize].push(GridCellType::End);
                } else if row == start.0 && col == start.1 {
                    grid[row as usize].push(GridCellType::Start);
                } else {
                    grid[row as usize].push(GridCellType::Empty);
                }
            }
        }

        grid
    }

    pub fn new(rows: i32, cols: i32, start: (i32,i32), end: (i32,i32)) -> SearchableGrid {
        SearchableGrid {
            grid: SearchableGrid::new_grid_vec(start,end),
            start,
            end,
            rows,
            cols,
        }
    }

    pub fn set(&mut self, row: i32, col: i32, grid_cell: GridCellType) {
        self.grid[row as usize][col as usize] = grid_cell;
    }

    pub fn reset_grid(&mut self, start:(i32,i32),end:(i32,i32)) {
        self.start = start;
        self.end = end;
        self.grid = SearchableGrid::new_grid_vec(self.start,self.end);
    }

    pub fn update_end_point(&mut self, end:(i32,i32)) {
        self.grid[self.end.0 as usize][self.end.1 as usize] = GridCellType::Empty;
        self.end = end;
        self.grid[self.end.0 as usize][self.end.1 as usize] = GridCellType::End;
    }

    pub fn update_start_point(&mut self, start:(i32,i32)) {
        self.grid[self.start.0 as usize][self.start.1 as usize] = GridCellType::Empty;
        self.start = start;
        self.grid[self.start.0 as usize][self.start.1 as usize] = GridCellType::Start;
    }
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