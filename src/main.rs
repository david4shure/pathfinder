use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
mod grid;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 800;
const NUM_ROWS: i32 = 50;
const NUM_COLS: i32 = 50;

fn main() {
    let mut grid = grid::SearchableGrid::new(50, 50);

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "I am a window!".to_string(),
                width: WINDOW_WIDTH as f32,
                height: WINDOW_HEIGHT as f32,
                ..default()
            },
            ..default()
        }))
        .insert_resource(grid)
        .add_system(redraw_grid_on_change)
        .add_system(mouse_button_input)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
    mut grid: ResMut<grid::SearchableGrid>,
) {
    let window = windows.primary_mut();

    println!("{} x {}", window.width(), window.height());

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
        println!("Grid changed!");

        for entity in &mut mesh_query {
            commands.entity(entity).despawn_recursive();
        }

        for row in grid.grid.iter() {
            for col in row.iter() {
                let (x, y) = col.get_render_position(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32);

                let mut color = Color::WHITE;

                println!("{:?}", color);

                if col.typ == grid::GridCellType::Wall {
                    color = Color::BLACK;
                }

                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                        transform: Transform::default()
                            .with_scale(Vec3::splat(12.))
                            .with_translation(Vec3::new(x, y, 0.)),
                        material: materials.add(ColorMaterial::from(color)),
                        ..default()
                    },
                    grid::GridCellLabel,
                ));
            }
        }
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
            println!("({},{})", _position.x, _position.y);
            let (row, col) = grid::screen_coord_to_row_col(
                _position.x as i32,
                _position.y as i32,
                NUM_ROWS,
                NUM_COLS,
                WINDOW_WIDTH,
                WINDOW_HEIGHT,
            );
            let mut grid_cell = grid.grid[row as usize][col as usize];
            grid_cell.typ = grid::GridCellType::Wall;

            println!("{:?}", grid.grid[row as usize][col as usize]);

            grid.grid[row as usize][col as usize] = grid_cell;
        }
    }
}
