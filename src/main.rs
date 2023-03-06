use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use std::time::Instant;

mod grid;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 800;

fn main() {
    let grid = grid::SearchableGrid::new(grid::NUM_ROWS, grid::NUM_COLS);

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Pathfinder".to_string(),
                width: WINDOW_WIDTH as f32,
                height: WINDOW_HEIGHT as f32,
                ..default()
            },
            ..default()
        }))
        .insert_resource(grid)
        .add_system(redraw_grid_on_change)
        .add_system(mouse_button_input)
        .add_system(keyboard_input.after(redraw_grid_on_change))
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn redraw_grid_on_change(
    mut grid: ResMut<grid::SearchableGrid>,
    mut mesh_query: Query<Entity, With<grid::GridCellLabel>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if grid.is_changed() {
        for entity in &mut mesh_query {
            commands.entity(entity).despawn_recursive();
        }

        let start = Instant::now();
        let path = grid.astar_shortest_path((0,0), (grid::NUM_ROWS-1,grid::NUM_COLS-1), grid::SearchableGrid::euclidean_distance);
        let duration = start.elapsed();
    
        println!("Pathfinding duration is {:?}", duration);
        
        for row in 0..grid::NUM_ROWS {
            for col in 0..grid::NUM_COLS {
                let grid_cell = grid.get(row,col);

                let mut color = grid::get_color(grid_cell);

                if path.contains(&(row,col)) {
                    color = Color::BLUE;
                }

                let (x, y) = grid::get_render_position(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32,row,col);

                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                        transform: Transform::default()
                            .with_scale(Vec3::splat((WINDOW_WIDTH as f32/grid::NUM_ROWS as f32) * 0.9))
                            .with_translation(Vec3::new(x, y, 0.)),
                        material: materials.add(ColorMaterial::from(color)),
                        ..default()
                    },
                    grid::GridCellLabel,
                ));
            }
        }

        for grid in path {
            let (x, y) = grid::get_render_position(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32, grid.0,grid.1);

            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                    transform: Transform::default()
                        .with_scale(Vec3::splat((WINDOW_WIDTH as f32/grid::NUM_ROWS as f32) * 0.9))
                        .with_translation(Vec3::new(x, y, 0.)),
                    material: materials.add(ColorMaterial::from(Color::BLUE)),
                    ..default()
                },
                grid::GridCellLabel,
            ));
        }
    }
}

fn keyboard_input(keys: Res<Input<KeyCode>>, mut grid: ResMut<grid::SearchableGrid>) {
    if keys.pressed(KeyCode::R) {
        grid.reset_grid();
    }
}

fn mouse_button_input(
    buttons: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut grid: ResMut<grid::SearchableGrid>,
) {
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

    if buttons.pressed(MouseButton::Left) {
        // Left button was pressed
        let (row, col) = grid::screen_coord_to_row_col(
            position_x,
            position_y,
            WINDOW_WIDTH as f32,
            WINDOW_HEIGHT as f32,
        );

        if row > grid::NUM_ROWS-1 || col > grid::NUM_COLS-1 || row < 0 || col < 0 {
            return;
        }

        grid.set(row, col, grid::GridCellType::Wall);
    }

    if buttons.pressed(MouseButton::Right) {
        let (row, col) = grid::screen_coord_to_row_col(
            position_x,
            position_y,
            WINDOW_WIDTH as f32,
            WINDOW_HEIGHT as f32,
        );

        if row > grid::NUM_ROWS-1 || col > grid::NUM_COLS-1 || row < 0 || col < 0 {
            return;
        }

        grid.set(row, col, grid::GridCellType::Empty);
    }
}
