use bevy::{math::Vec2, prelude::*, utils::{HashMap, HashSet}, window::PrimaryWindow};
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::{
    dynamics::Velocity,
    plugin::{NoUserData, RapierPhysicsPlugin},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(70.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(LevelSelection::index(0))
        .add_systems(Startup, init)
        .add_systems(
            Update,
            (
                handle_input,
                set_settings,
                camera_follow_player,
                spawn_wall_collision,
                handle_velocity,
            ),
        )
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<EnemyBundle>("Enemy")
        .register_ldtk_int_cell::<WallBundle>(1)
        .run();
}
//bro
#[derive(Component)]
struct Health(u64);

impl Default for Health {
    fn default() -> Self {
        Self(100)
    }
}
#[derive(Component)]
struct RunTimer {
    timer: Timer,
}

#[derive(Component)]
struct Stamina(i64);

impl Default for Stamina {
    fn default() -> Self {
        Self(100)
    }
}

#[derive(Default, Component)]
struct Abilities {
    abilities: Vec<AbilitiesEnum>,
}

#[derive(Default)]
enum AbilitiesEnum {
    #[default]
    None,
    SwitchLight,
}

#[derive(Default, Component)]
struct Debufs {
    debufs: Vec<DebufsEnum>,
}

#[derive(Default)]
enum DebufsEnum {
    #[default]
    None,
    Poison,
    Fire,
}

#[derive(Deafault, Component)]
struct Enemy;

#[derive(Deafault, Component)]
struct EnemyBundle {
    enemy: Enemy,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,

}

#[derive(Default, LdtkEntity)]
struct Player;

#[derive(Bundle, LdtkEntity)]
struct PlayerBundle {
    player: Player,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
    abilities: Abilities,
    debufs: Debufs,
    health: Health,
    stamina: Stamina,
    collider: Collider,
    velocity: Velocity,
    rigid_body: RigidBody,
    locked_axes: LockedAxes,
    ccd: Ccd,
    damping: Damping,
    mass: ColliderMassProperties,
    bouncyness: Restitution,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player,
            sprite_sheet_bundle: SpriteSheetBundle::default(),
            grid_coords: GridCoords::default(),
            abilities: Abilities::default(),
            debufs: Debufs::default(),
            health: Health::default(),
            stamina: Stamina::default(),
            collider: Collider::ball(4.0),
            velocity: Velocity::zero(),
            rigid_body: RigidBody::Dynamic,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            ccd: Ccd::enabled(),
            damping: Damping::default(),
            mass: ColliderMassProperties::Density(100.0),
            bouncyness: Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
        }
    }
}

#[derive(Default, Component)]
struct Wall;

#[derive(Default, Bundle, LdtkIntCell)]
struct WallBundle {
    wall: Wall,
}

#[derive(Clone, Component)]
struct PanelSize(u32);

#[derive(Clone, Component)]
struct PanelStart(i32);

#[derive(Clone, Component)]
struct PanelEnd(i32);

#[derive(Clone, Bundle)]
struct FloorPanel {
    wall: TransformBundle,
    size: PanelSize,
    start: PanelStart,
    end: PanelEnd,
    collider: Collider,
}

<<<<<<< HEAD


fn change_sprite(
    mut commands: Commands,
    mut event_reader: EventReader<LevelEvent>,
    query: Query<(Entity, &mut SpriteSheetBundle), With<Player>>,
) {
    for event in event_reader.iter() {
        if let LevelEvent::Spawned(_) = event {
            // Assuming you want to change the sprite to the first sprite in the atlas
            let new_sprite_index = 0; // Adjust this index as needed

            for (entity, mut sprite_sheet_bundle) in query.iter_mut() {
                // Update the sprite index in the SpriteSheetBundle
                sprite_sheet_bundle.sprite = TextureAtlasSprite::new(new_sprite_index);

                // Optionally, you can also update the texture atlas if the sprite index changes
                // sprite_sheet_bundle.texture_atlas = new_texture_atlas_handle;
            }
        }
    }
}

=======
>>>>>>> 93245f43d867323f302c0c4c6ed9c379f4e9349e
fn init(
    window: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let window = window.get_single().unwrap();

    let mut camera = Camera2dBundle::default();

    camera.projection.scale = 0.2;

    camera.transform.translation.x = window.width() / 2.0;
    camera.transform.translation.y = window.height() / 2.0;

    commands.spawn(camera);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("gamejam.ldtk"),
        transform: Transform::from_xyz(
            window.width() / 2.0 - 256.0 / 2.0,
            window.height() / 2.0 - 256.0 / 2.0,
            0.0,
        ),
        ..Default::default()
    });
}

fn handle_input(mut player: Query<&mut Velocity, With<Player>>, keyb: Res<Input<KeyCode>>) {
    if let Ok(mut velocity) = player.get_single_mut() {
        if keyb.just_pressed(KeyCode::W) {
            velocity.linvel += Vec2::new(0.0, 50.0);
        } else if keyb.pressed(KeyCode::D) {
            let y = velocity.linvel.y;

            velocity.linvel = Vec2::new(45.0, y);
        } else if keyb.just_released(KeyCode::D) {
            let y = velocity.linvel.y;

            velocity.linvel = Vec2::new(20.0, y);
        } else if keyb.pressed(KeyCode::A) {
            let y = velocity.linvel.y;

            velocity.linvel = Vec2::new(-45.0, y);
        } else if keyb.just_released(KeyCode::A) {
            let y = velocity.linvel.y;

            velocity.linvel = Vec2::new(-20.0, y);
        }
    }
}

fn handle_velocity(
    mut player: Query<(&Velocity, &mut Transform), Changed<Velocity>>,
    time: Res<Time>,
) {
    if let Ok((velocity, mut transform)) = player.get_single_mut() {
        let x = velocity.linvel.x;
        let y = velocity.linvel.y;

        transform.translation.x += x * time.delta_seconds();
        transform.translation.y += y * time.delta_seconds();
    }
}

fn set_settings(mut player: Query<&mut Damping, With<Player>>, mut event: EventReader<LevelEvent>) {
    for event in event.read() {
        if let LevelEvent::Spawned(_) = event {
            let mut dampening = player.get_single_mut().unwrap();
            dampening.linear_damping = 1.0;
        }
    }
}

fn camera_follow_player(
    window: Query<&Window, With<PrimaryWindow>>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    if let Ok(mut cam_transform) = camera.get_single_mut() {
        if let Ok(player_transform) = player.get_single() {
            if let Ok(window) = window.get_single() {
                let target_y = player_transform.translation.y + window.height() / 2.0 - 256.0 / 2.0;
                let target_x = player_transform.translation.x + window.width() / 2.0 - 256.0 / 2.0;

                cam_transform.translation.x = target_x;
                cam_transform.translation.y = target_y;
            }
        }
    }
}

fn spawn_wall_collision(
    mut commands: Commands,
    wall_query: Query<(&GridCoords, &Parent), Added<Wall>>,
    parent_query: Query<&Parent, Without<Wall>>,
    level_query: Query<(Entity, &LevelIid)>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) {
    /// Represents a wide wall that is 1 tile tall
    /// Used to spawn wall collisions
    #[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
    struct Plate {
        left: i32,
        right: i32,
    }

    /// A simple rectangle type representing a wall of any size
    struct Rect {
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
    }

    // Consider where the walls are
    // storing them as GridCoords in a HashSet for quick, easy lookup
    //
    // The key of this map will be the entity of the level the wall belongs to.
    // This has two consequences in the resulting collision entities:
    // 1. it forces the walls to be split along level boundaries
    // 2. it lets us easily add the collision entities as children of the appropriate level entity
    let mut level_to_wall_locations: HashMap<Entity, HashSet<GridCoords>> = HashMap::new();

    wall_query.iter().for_each(|(&grid_coords, parent)| {
        // An intgrid tile's direct parent will be a layer entity, not the level entity
        // To get the level entity, you need the tile's grandparent.
        // This is where parent_query comes in.
        if let Ok(grandparent) = parent_query.get(parent.get()) {
            level_to_wall_locations
                .entry(grandparent.get())
                .or_default()
                .insert(grid_coords);
        }
    });

    if !wall_query.is_empty() {
        level_query.iter().for_each(|(level_entity, level_iid)| {
            if let Some(level_walls) = level_to_wall_locations.get(&level_entity) {
                let ldtk_project = ldtk_project_assets
                    .get(ldtk_projects.single())
                    .expect("Project should be loaded if level has spawned");

                let level = ldtk_project
                    .as_standalone()
                    .get_loaded_level_by_iid(&level_iid.to_string())
                    .expect("Spawned level should exist in LDtk project");

                let LayerInstance {
                    c_wid: width,
                    c_hei: height,
                    grid_size,
                    ..
                } = level.layer_instances()[0];

                // combine wall tiles into flat "plates" in each individual row
                let mut plate_stack: Vec<Vec<Plate>> = Vec::new();

                for y in 0..height {
                    let mut row_plates: Vec<Plate> = Vec::new();
                    let mut plate_start = None;

                    // + 1 to the width so the algorithm "terminates" plates that touch the right edge
                    for x in 0..width + 1 {
                        match (plate_start, level_walls.contains(&GridCoords { x, y })) {
                            (Some(s), false) => {
                                row_plates.push(Plate {
                                    left: s,
                                    right: x - 1,
                                });
                                plate_start = None;
                            }
                            (None, true) => plate_start = Some(x),
                            _ => (),
                        }
                    }

                    plate_stack.push(row_plates);
                }

                // combine "plates" into rectangles across multiple rows
                let mut rect_builder: HashMap<Plate, Rect> = HashMap::new();
                let mut prev_row: Vec<Plate> = Vec::new();
                let mut wall_rects: Vec<Rect> = Vec::new();

                // an extra empty row so the algorithm "finishes" the rects that touch the top edge
                plate_stack.push(Vec::new());

                for (y, current_row) in plate_stack.into_iter().enumerate() {
                    for prev_plate in &prev_row {
                        if !current_row.contains(prev_plate) {
                            // remove the finished rect so that the same plate in the future starts a new rect
                            if let Some(rect) = rect_builder.remove(prev_plate) {
                                wall_rects.push(rect);
                            }
                        }
                    }
                    for plate in &current_row {
                        rect_builder
                            .entry(plate.clone())
                            .and_modify(|e| e.top += 1)
                            .or_insert(Rect {
                                bottom: y as i32,
                                top: y as i32,
                                left: plate.left,
                                right: plate.right,
                            });
                    }
                    prev_row = current_row;
                }

                commands.entity(level_entity).with_children(|level| {
                    // Spawn colliders for every rectangle..
                    // Making the collider a child of the level serves two purposes:
                    // 1. Adjusts the transforms to be relative to the level for free
                    // 2. the colliders will be despawned automatically when levels unload
                    for wall_rect in wall_rects {
                        level
                            .spawn_empty()
                            .insert(Collider::cuboid(
                                (wall_rect.right as f32 - wall_rect.left as f32 + 1.)
                                    * grid_size as f32
                                    / 2.,
                                (wall_rect.top as f32 - wall_rect.bottom as f32 + 1.)
                                    * grid_size as f32
                                    / 2.,
                            ))
                            .insert(RigidBody::Fixed)
                            .insert(Friction::new(1.0))
                            .insert(Transform::from_xyz(
                                (wall_rect.left + wall_rect.right + 1) as f32 * grid_size as f32
                                    / 2.,
                                (wall_rect.bottom + wall_rect.top + 1) as f32 * grid_size as f32
                                    / 2.,
                                0.,
                            ))
                            .insert(GlobalTransform::default());
                    }
                });
            }
        });
    }
}
