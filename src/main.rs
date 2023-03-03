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

fn setup(mut commands: Commands, mut windows: ResMut<Windows>) {
    let window = windows.primary_mut();

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

        let start = grid.get(0,0);
        let end = grid.get(grid::NUM_ROWS-1,grid::NUM_COLS-1);
        let path = grid.astar_shortest_path(start, end, grid::SearchableGrid::euclidean_distance);
    
        for row in 0..grid::NUM_ROWS {
            for col in 0..grid::NUM_COLS {
                let grid_cell = grid.get(row,col);

                let mut color = grid_cell.get_color();

                if path.contains(&grid_cell) {
                    color = Color::BLUE;
                }

                let (x, y) = grid_cell.get_render_position(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);

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
            let (x, y) = grid.get_render_position(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);

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
    let window = windows.get_primary().unwrap();

    if buttons.pressed(MouseButton::Left) {
        // Left button was pressed
        if let Some(_position) = window.cursor_position() {
            // cursor is inside the window, position given
            let (row, col) = grid::screen_coord_to_row_col(
                _position.x as i32,
                _position.y as i32,
                grid::NUM_ROWS,
                grid::NUM_COLS,
                WINDOW_WIDTH,
                WINDOW_HEIGHT,
            );

            if row > grid::NUM_ROWS-1 || col > grid::NUM_COLS-1 || row < 0 || col < 0 {
                return;
            }

            let mut grid_cell = grid.get(row, col);
            grid_cell.typ = grid::GridCellType::Wall;

            grid.set(row, col, grid_cell);
        }
    }
}
