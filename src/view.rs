use std::fs::File;
use std::io::{BufRead, BufReader};
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::math::vec3;
use bevy::prelude::*;
use bevy_prototype_debug_lines::{DebugLines};

const CAMERA_SPEED_PER_SEC: f32 = 2.0;
const MAP_SIZE: f32 = 1000.;
const MAP_SCALE: f32 = 6.;

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Map;

#[derive(Component)]
struct Total(u32);

#[derive(Component)]
struct Track {
    cord: Vec3,
    file: FileStruct,
}

pub struct ViewMap;

struct MouseLoc {
    cursor: Vec2,
    delta: Vec2,
    cursor_wnd: Vec2,
}

#[derive(Clone)]
struct FileStruct {
    name: String,
    color: Color
}

struct GreetTimer(Timer);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.scale = 5.0;

    commands.spawn()
        .insert_bundle(camera)
        .insert(MainCamera);

    commands.spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("Sanandreas_map.png"),
            transform: Transform {
                translation: Vec3::ZERO,
                rotation: Default::default(),
                scale: Vec3::new(MAP_SCALE, MAP_SCALE, 0.0)
            },
            global_transform: GlobalTransform {
                translation: Vec3::ZERO,
                rotation: Default::default(),
                scale: Vec3::ZERO
            },

            ..Default::default()
        })
        .insert(Map)
    ;

    for ff in [
        FileStruct {name: "tracks".to_string(), color: Color::GREEN},
        FileStruct {name: "tracks2".to_string(), color: Color::PINK},
        FileStruct {name: "tracks3".to_string(), color: Color::INDIGO},
        FileStruct {name: "tracks4".to_string(), color: Color::YELLOW_GREEN},
    ] {
        let path = format!("resources/{}.dat", ff.name);
        let f = File::open(path).expect("Unable to open file");
        let l = BufReader::new(f);


        let mut count: u32 = 0;

        // println!("First: {}", count);

        for line in l.lines() {
            let line = line.expect("Unable to read line");
            if count == 0 {
                count = line.parse().unwrap();
            } else {
                let split = line.split(" ");

                let vec: Vec<&str> = split.collect();

                commands.spawn().insert(Track { cord: vec3(vec[0].parse().unwrap(), vec[1].parse().unwrap(), vec[2].parse().unwrap()), file: ff.clone() });
            }
        }
    }
}

fn print_coordinates(mut lines: ResMut<DebugLines>, query: Query<&Track>,) {
    let mut iter = query.iter();
    let mut previous = iter.next().unwrap();

    let z_index = 10.;
    let duration = 0.;

    let map_center = MAP_SIZE * MAP_SCALE / 2.;

    for current in iter {
        if current.file.name == previous.file.name {
            lines.line_colored(
                Vec3::new(current.cord.x, current.cord.y, z_index),
                Vec3::new(previous.cord.x, previous.cord.y, z_index),
                duration,
                current.file.color,
            );
        }

        previous = current;
    }

    lines.line_colored(
        Vec3::new(-map_center, 0., z_index),
        Vec3::new(map_center, 0., z_index),
        duration,
        Color::RED,
    );

    lines.line_colored(
        Vec3::new(0., -map_center, z_index),
        Vec3::new(0., map_center, z_index),
        duration,
        Color::RED,
    );
}

fn player_camera_control(
    time: Res<Time>,
    mb: Res<Input<MouseButton>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mouse_pos: ResMut<MouseLoc>,
    mut t_camera: Query<&mut Transform, With<MainCamera>>,
    mut o_camera: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
    let dist = CAMERA_SPEED_PER_SEC * time.delta().as_secs_f32();

    let mut camera_transform = t_camera.single_mut();
    let mut camera_orientation = o_camera.single_mut();
    let mut log_scale = camera_orientation.scale.ln();

    for mouse_wheel_event in mouse_wheel_events.iter() {
        if mouse_wheel_event.y == -1.0 {
            log_scale += dist;
        } else {
            log_scale -= dist;
        }

        camera_orientation.scale = log_scale.exp();
    }

    let z = 999.;

    if mb.pressed(MouseButton::Left) {
        camera_transform.translation = -Vec3::new(mouse_pos.cursor.x, mouse_pos.cursor.y, z);
        camera_transform.translation.z = z;
    }

    if mb.pressed(MouseButton::Right) {
        camera_transform.translation = Vec3::new(0., 0., z);
    }
}

fn my_cursor_system(
    mut mouse_pos: ResMut<MouseLoc>,
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<&Transform, With<MainCamera>>
) {
    // get the primary window
    let wnd = wnds.get_primary().unwrap();

    // check if the cursor is in the primary window
    if let Some(pos) = wnd.cursor_position() {
        // get the size of the window
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let p = pos - size / 2.0;

        // assuming there is exactly one main camera entity, so this is OK
        let camera_transform = q_camera.single();

        // apply the camera transform
        let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
        mouse_pos.cursor_wnd = Vec2::new(pos_wld.x, pos_wld.y);
    }
}

fn move_max(
    keyboard_input: Res<Input<KeyCode>>,
    mut max_positions: Query<&mut Transform, With<Map>>,
) {
    for mut transform in max_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::LShift) {
            transform.translation.z += 1.0;
            println!("{}", transform.translation.z);
        }

        if keyboard_input.pressed(KeyCode::RShift) {
            transform.translation.z = 0.0;
            println!("{}", transform.translation.z);
        }
        if keyboard_input.pressed(KeyCode::LControl) {
            transform.translation.z -= 1.0;
            println!("{}", transform.translation.z);
        }
    }
}


/// This system prints out all mouse events as they come in
fn print_mouse_events_system(
    mut mouse_pos: ResMut<MouseLoc>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_motion: EventReader<MouseMotion>,

) {
    for event in cursor_moved_events.iter() {
        mouse_pos.cursor = event.position;
    }
    for event in mouse_motion.iter() {
        mouse_pos.delta += event.delta;
    }
}

impl Plugin for ViewMap {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(MouseLoc{
                cursor: Vec2::new(0.0, 0.0),
                delta: Vec2::new(0.0, 0.0),
                cursor_wnd: Vec2::new(0.0, 0.0),
            })
            .insert_resource(GreetTimer(Timer::from_seconds(0.1, true)))
            .add_startup_system(setup)
            // .add_system(demo)
            .add_system(player_camera_control)
            .add_system(print_mouse_events_system)
            .add_system(my_cursor_system)
            .add_system(move_max)
            .add_system_to_stage(CoreStage::Last, print_coordinates)
        ;
    }
}