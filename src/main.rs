use std::time::Duration;

use bevy::{
    math::Vec2,
    prelude::*,
    utils::{HashMap, HashSet},
    window::PrimaryWindow,
};
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
                camera_follow_player,
                spawn_wall_collision,
                handle_velocity,
                animate,
                recover_stamina,
                enemy_attack,
                deal_enemy_damage,
            ),
        )
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<Enemy1Bundle>("Enemy")
        .register_ldtk_int_cell::<WallBundle>(1)
        .run();
}

//bro
#[derive(Component)]
struct AnimationTimer {
    timer: Timer,
}

impl Default for AnimationTimer {
    fn default() -> Self {
        AnimationTimer {
            timer: Timer::new(Duration::from_secs_f32(0.1), TimerMode::Repeating),
        }
    }
}

#[derive(Component)]
struct Health(u64);

impl Default for Health {
    fn default() -> Self {
        Self(100)
    }
}

#[derive(Default, Component)]
struct PlayerBulletFire{
    player: bool,
    enemy: bool,
}


#[derive(Default, Component)]
enum BulletType{
    #[default]
    Enemy,
    Player,
}

#[derive(Default, Component)]
struct Bullet;

#[derive(Bundle, LdtkEntity)]
struct BulletBundle{
    bullet: Bullet,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    collider: Collider,
    velocity: Velocity,
    bullet_type: BulletType,
}

impl Default for BulletBundle {
    fn default() -> Self {
        Self {
            bullet: Bullet::default(),
            sprite_sheet_bundle: SpriteSheetBundle::default(),
            collider: Collider::ball(0.4),
            velocity: Velocity::zero(),
            bullet_type: BulletType::default(),
        }
    }
}

#[derive(Default, Component)]
struct SpawnBullet{
    premission: bool
}


#[derive(Component)]
struct Stamina(i64);

impl Default for Stamina {
    fn default() -> Self {
        Self(100)
    }
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
#[derive(Component)]
struct EnemyBulletTimer{
    timer: Timer
}
impl Default for EnemyBulletTimer{
    fn default() -> Self {
        Self { timer: Timer::new(Duration::from_secs_f32(1.0),TimerMode::Repeating) }
    }
}
#[derive(Component)]
struct PlayerBulletTimer{
    timer:Timer
}

impl Default for PlayerBulletTimer{
    fn default() -> Self {
        Self { timer: Timer::new(Duration::from_secs_f32(1.0),TimerMode::Repeating) }
    }
}

#[derive(Default, Component, Clone)]
enum EnemyAttack {
    #[default]
    None,
    Attack,
}

#[derive(Component)]
struct EnemyAttackCooldown {
    timer: Timer,
}

impl Default for EnemyAttackCooldown {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::from_secs_f32(1.0), TimerMode::Repeating),
        }
    }
}

#[derive(Component, Clone)]
struct EnemyDamage(u64);

#[derive(Component)]
struct EnemyHealth(u64);


#[derive(Default, Component)]
struct Enemy1;

#[derive(Bundle, LdtkEntity)]
struct Enemy1Bundle {
    enemy: Enemy1,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
    collider: Collider,
    bullet_type: BulletType, 
    rigid_body: RigidBody,
    lock_axes: LockedAxes,
    collision_group: CollisionGroups,
    attack: EnemyAttack,
    damage: EnemyDamage,
    health: EnemyHealth,
    enemy_attack_cooldown: EnemyAttackCooldown,
}

impl Default for Enemy1Bundle {
    fn default() -> Self {
        Self {
            enemy: Enemy1,
            sprite_sheet_bundle: SpriteSheetBundle::default(),
            grid_coords: GridCoords::default(),
            collider: Collider::cuboid(10.0, 12.0),
            bullet_type: BulletType::Enemy,
            rigid_body: RigidBody::Dynamic,
            lock_axes: LockedAxes::ROTATION_LOCKED,
            collision_group: CollisionGroups::new(
                Group::from_bits(0b10).unwrap(),
                Group::from_bits(0b1).unwrap(),
            ),
            attack: EnemyAttack::default(),
            damage: EnemyDamage(45),
            health: EnemyHealth(200),
            enemy_attack_cooldown: EnemyAttackCooldown {
                timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
            },
        }
    }
}

#[derive(Component, PartialEq)]
enum Animation {
    Run(u8),
    Death(u8),
    Dash(u8),
    Attack(u8),
    ChargedAttack(u8),

}
#[derive(Component, PartialEq)]
enum AnimationEnemy1{
    Run(u8),
    Death(u8),
    Idle(u8),

}

#[derive(Component)]
struct StaminaRecoveryTimer {
    timer: Timer,
}

impl Default for StaminaRecoveryTimer {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
        }
    }
}

impl Default for Animation {
    fn default() -> Self {
        Self::Run(0)
    }
}

#[derive(Default, Component)]
enum PlayerAttack {
    #[default]
    None,
    Attack,
    ChargedAttack,
}

#[derive(Default, Component)]
struct Player;

#[derive(Bundle, LdtkEntity)]
struct PlayerBundle {
    player: Player,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
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
    animation_timer: AnimationTimer,
    bullet_type: BulletType,
    collision_group: CollisionGroups,
    animation: Animation,
    attack: PlayerAttack,
    stamina_recovery: StaminaRecoveryTimer,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player,
            sprite_sheet_bundle: SpriteSheetBundle::default(),
            grid_coords: GridCoords::default(),
            debufs: Debufs::default(),
            health: Health::default(),
            stamina: Stamina::default(),
            collider: Collider::cuboid(25.0, 25.0),
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
            bullet_type: BulletType::Player,
            animation_timer: AnimationTimer::default(),
            collision_group: CollisionGroups::new(
                Group::from_bits(0b10).unwrap(),
                Group::from_bits(0b1).unwrap(),
            ),
            animation: Animation::default(),
            attack: PlayerAttack::default(),
            stamina_recovery: StaminaRecoveryTimer::default(),
        }
    }
}

#[derive(Default, Component)]
struct Wall;

#[derive(Default, Bundle, LdtkIntCell)]
struct WallBundle {
    wall: Wall,
}

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

fn handle_input(
    mut player: Query<(&mut Stamina, &mut Velocity, &mut Animation, &TextureAtlasSprite), With<Player>>,
    keyb: Res<Input<KeyCode>>,
) {
    if let Ok((mut stamina, mut velocity, mut animation, sprite)) = player.get_single_mut() {
        if let Animation::Run(_) = *animation {
            if keyb.just_pressed(KeyCode::Space) {
                if stamina.0 < 25 {
                    return;
                }

                if sprite.flip_x {
                    velocity.linvel = Vec2::new(-140.0, 0.0);
                } else {
                    velocity.linvel = Vec2::new(140.0, 0.0);
                }

                *animation = Animation::Dash(0);

                stamina.0 -= 25;
            } else if keyb.just_pressed(KeyCode::F) {
                if stamina.0 < 10 {
                    return;
                }

                *animation = Animation::Attack(0);

                stamina.0 -= 10;
            } else if keyb.just_pressed(KeyCode::G) {
                if stamina.0 < 75 {
                    return;
                }

                *animation = Animation::ChargedAttack(0);

                stamina.0 -= 75;
            } else if keyb.just_pressed(KeyCode::W) {
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
            } else if keyb.just_pressed(KeyCode::K) {
                *animation = Animation::Death(0);
            }
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

fn animate(
    mut player: Query<(&mut TextureAtlasSprite, &mut AnimationTimer, &mut Animation, &mut Velocity), With<Player>>,
    time: Res<Time>,
    keyb: Res<Input<KeyCode>>,
) {
    for (mut sprite, mut timer, mut animation, mut velocity) in player.iter_mut() {
        timer.timer.tick(time.delta());

        match *animation {
            Animation::Run(ref mut phase) => {
                if keyb.pressed(KeyCode::D) {
                    sprite.flip_x = false;
                } else if keyb.pressed(KeyCode::A) {
                    sprite.flip_x = true;
                }

                if timer.timer.just_finished() {
                    if keyb.pressed(KeyCode::D) || keyb.pressed(KeyCode::A) {
                        sprite.index = 23 + *phase as usize;
                        *phase += 1;
                        *phase %= 6;
                    } else {
                        sprite.index = 92;
                    }
                }
            }
            Animation::Dash(ref mut phase) => {
                if timer.timer.just_finished() {
                    sprite.index = *phase as usize;
                    *phase += 1;

                    if *phase == 3 {
                        *animation = Animation::Run(0);
                        velocity.linvel = Vec2::new(0.0, 0.0);
                    }
                }
            }
            Animation::Death(ref mut phase) => {
                if timer.timer.just_finished() {
                    sprite.index = 92 + *phase as usize;
                    *phase += 1;
                    *phase %= 23;

                    if *phase == 22 {
                        *animation = Animation::Run(0);
                    }
                }
            }
            Animation::Attack(ref mut phase) => {
                if timer.timer.just_finished() {
                    sprite.index = 46 + *phase as usize;
                    *phase += 2;
                    *phase %= 13;

                    if *phase >= 11 {
                        *animation = Animation::Run(0);
                    }
                }
            }
            Animation::ChargedAttack(ref mut phase) => {
                if timer.timer.just_finished() {
                    sprite.index = 46 + *phase as usize;
                    *phase += 1;
                    *phase %= 13;

                    if *phase == 11 {
                        *animation = Animation::Run(0);
                    }
                }
            }
        }
    }
}

fn recover_stamina(mut player: Query<(&mut Stamina, &mut StaminaRecoveryTimer), With<Player>>, time: Res<Time>) {
    if let Ok((mut stamina, mut stamina_recovery_timer)) = player.get_single_mut() {
        stamina_recovery_timer.timer.tick(time.delta());

        if stamina_recovery_timer.timer.just_finished() {
            stamina.0 += 15;

            if stamina.0 > 100 {
                stamina.0 = 100;
            }
        }
    }
}

fn enemy_attack(window: Query<&Window, With<PrimaryWindow>>, mut commands: Commands, mut enemy: Query<(&mut EnemyAttackCooldown, &Transform, &EnemyDamage), With<Enemy1>>, player: Query<&Transform, With<Player>>, time: Res<Time>)  {
    let window = window.get_single().unwrap();

    for (mut enemy_cooldown, transform, damage) in enemy.iter_mut() {
        enemy_cooldown.timer.tick(time.delta());

        for player_transform in player.iter() {
            let enemy_x = transform.translation.x;
            let enemy_y = transform.translation.y;

            let player_x = player_transform.translation.x;
            let player_y = player_transform.translation.y;

            let x = enemy_x - player_x;
            let y = enemy_y - player_y;

            let x = x.abs();
            let y = y.abs();

            if x < 10.0 && y < 10.0 {
                commands.spawn(Collider::cuboid(10.0, 10.0))
                    .insert(TransformBundle::from_transform(Transform::from_xyz(enemy_x + window.width() / 2.0 - 256.0 / 2.0, enemy_y + window.height() / 2.0 - 256.0 / 2.0, 0.0)))
                    .insert(Sensor)
                    .insert(damage.clone())
                    .insert(EnemyAttack::Attack)
                    .insert(ActiveEvents::COLLISION_EVENTS);
            }
        }
    }
}

fn deal_damage(mut )

