use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use std::time::Instant;

mod grid;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 800;

#[derive(Resource)]
struct StartPosition {
    row: i32,
    col: i32,
}

#[derive(Resource)]
struct EndPosition {
    row: i32,
    col: i32,
}

fn main() {
    let start_point = StartPosition{
        row:0,
        col:0,
    };

    let end_point = EndPosition {
        row:grid::NUM_ROWS-1,
        col:grid::NUM_COLS-1,
    };

    let grid = grid::SearchableGrid::new(grid::NUM_ROWS, grid::NUM_COLS, (start_point.row,start_point.col), (end_point.row,end_point.col));

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Pathfinder".to_string(),
                width: WINDOW_WIDTH as f32,
                height: WINDOW_HEIGHT as f32,
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .insert_resource(grid)
        .insert_resource(start_point)
        .insert_resource(end_point)
        .add_system(compute_path_and_redraw)
        .add_system(mouse_button_input)
        .add_system(update_start_or_end_point)
        .add_system(reset_grid_to_default)
        .add_startup_system(setup)
        .run();
}

fn get_row_col_from_window(windows: Res<Windows>) -> (i32,i32) {
    // Games typically only have one window (the primary window).
    // For multi-window applications, you need to use a specific window ID here.
    let _window = windows.get_primary().unwrap();

    let mut position_x = -1.0;
    let mut position_y = -1.0;

    if let Some(_position) = _window.cursor_position() {
        // cursor is inside the window, position given
        position_x = _position.x;
        position_y = _position.y;
    }

    // Left button was pressed
    let (row, col) = screen_coord_to_row_col(
        position_x,
        position_y,
        WINDOW_WIDTH as f32,
        WINDOW_HEIGHT as f32,
    );

    return (row,col)
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn compute_path_and_redraw(
    mut grid: ResMut<grid::SearchableGrid>,
    mut mesh_query: Query<Entity, With<grid::GridCellLabel>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    start: Res<StartPosition>,
    end: Res<EndPosition>,
) {
    if grid.is_changed() {
        for entity in &mut mesh_query {
            commands.entity(entity).despawn_recursive();
        }

        let start_time = Instant::now();
        let path = grid.astar_shortest_path(
            (start.row, start.col),
            (end.row, end.col),
            grid::SearchableGrid::manhatten_distance,
        );
        let duration = start_time.elapsed();

        println!("Pathfinding duration is {:?}", duration);

        for row in 0..grid::NUM_ROWS {
            for col in 0..grid::NUM_COLS {
                let grid_cell = grid.grid[row as usize][col as usize];

                let mut color = grid::get_color(grid_cell);

                if path.contains(&(row, col)) {
                    color = Color::BLUE;
                }

                let (x, y) =
                    get_render_position(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32, row, col);

                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                        transform: Transform::default()
                            .with_scale(Vec3::splat(
                                (WINDOW_WIDTH as f32 / grid::NUM_ROWS as f32) * 0.9,
                            ))
                            .with_translation(Vec3::new(x, y, 0.)),
                        material: materials.add(ColorMaterial::from(color)),
                        ..default()
                    },
                    grid::GridCellLabel,
                ));
            }
        }

        for grid in path {
            let (x, y) = get_render_position(
                WINDOW_WIDTH as f32,
                WINDOW_HEIGHT as f32,
                grid.0,
                grid.1,
            );

            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                    transform: Transform::default()
                        .with_scale(Vec3::splat(
                            (WINDOW_WIDTH as f32 / grid::NUM_ROWS as f32) * 0.9,
                        ))
                        .with_translation(Vec3::new(x, y, 0.)),
                    material: materials.add(ColorMaterial::from(Color::BLUE)),
                    ..default()
                },
                grid::GridCellLabel,
            ));
        }
    }
}

fn reset_grid_to_default(
    keys: Res<Input<KeyCode>>, 
    mut grid: ResMut<grid::SearchableGrid>,
    mut start: ResMut<StartPosition>,
    mut end: ResMut<EndPosition>,
) {
    if keys.pressed(KeyCode::Space) {
        start.row = 0;
        start.col = 0;
        end.row = grid::NUM_ROWS-1;
        end.col = grid::NUM_COLS-1;
        grid.reset_grid((start.row,start.col),(end.row,end.col));
    }
}

fn mouse_button_input(
    mouse_buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut grid: ResMut<grid::SearchableGrid>,
) {
    let (row,col) = get_row_col_from_window(windows);

    if row > grid::NUM_ROWS - 1 || col > grid::NUM_COLS - 1 || row < 0 || col < 0 {
        return;
    }

    if mouse_buttons.pressed(MouseButton::Left) {
        let original_type = grid.grid[row as usize][col as usize];

        if original_type != grid::GridCellType::Wall && original_type != grid::GridCellType::End && original_type != grid::GridCellType::Start {
            grid.set(row, col, grid::GridCellType::Wall);
        }
    }

    if mouse_buttons.pressed(MouseButton::Right) {
        let original_type = grid.grid[row as usize][col as usize];

        if original_type != grid::GridCellType::Empty && original_type != grid::GridCellType::End && original_type != grid::GridCellType::Start {
            grid.set(row, col, grid::GridCellType::Empty);
        }
    }
}

fn update_start_or_end_point(    
    keyboard_buttons: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    mut end_point: ResMut<EndPosition>,
    mut start_point: ResMut<StartPosition>,
    mut grid: ResMut<grid::SearchableGrid>,
) {
    let (row,col) = get_row_col_from_window(windows);

    if row > grid::NUM_ROWS - 1 || col > grid::NUM_COLS - 1 || row < 0 || col < 0 {
        return;
    }

    if keyboard_buttons.pressed(KeyCode::E) {
        end_point.row = row;
        end_point.col = col;
        grid.update_end_point((end_point.row,end_point.col))
    }

    if keyboard_buttons.pressed(KeyCode::S) {
        start_point.row = row;
        start_point.col = col;
        grid.update_start_point((start_point.row,start_point.col))
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
    screen_width: f32,
    screen_height: f32,
) -> (i32, i32) {
    let pixels_per_row = screen_width / grid::NUM_ROWS as f32;
    let pixels_per_column = screen_height / grid::NUM_COLS as f32;

    let row = x / pixels_per_row;
    let col = y / pixels_per_column;

    (row as i32, col as i32)
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
pub fn get_render_position(
    screen_width: f32,
    screen_height: f32,
    row: i32,
    col: i32,
) -> (f32, f32) {
    let width_per_row = screen_width / grid::NUM_ROWS as f32;
    let height_per_column = screen_height / grid::NUM_COLS as f32;

    (
        width_per_row * row as f32 - (screen_width / 2.) + 0.5 * width_per_row,
        height_per_column * col as f32 - (screen_height / 2.) + 0.5 * height_per_column,
    )
}