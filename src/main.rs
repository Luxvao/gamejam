use bevy::{math::Vec2, prelude::*, window::PrimaryWindow};
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
        .insert_resource(LevelSelection::index(0))
        .add_systems(Startup, init)
        .add_systems(Update, (handle_input, handle_velocity, set_settings))
        .register_ldtk_entity::<PlayerBundle>("Player")
        .register_ldtk_entity::<EnemyBundle>("Enemy")
        .register_ldtk_int_cell::<WallBundle>(1)
        .run();
}

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
            collider: Collider::cuboid(8.0, 8.0),
            velocity: Velocity::zero(),
            rigid_body: RigidBody::Dynamic,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            ccd: Ccd::enabled(),
            damping: Damping::default(),
            mass: ColliderMassProperties::Density(100.0),
        }
    }
}

#[derive(Default, Component)]
struct Wall;

#[derive(Default, Bundle, LdtkIntCell)]
struct WallBundle {
    wall: Wall,
    collider: Collider,
}



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

fn init(
    window: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let window = window.get_single().unwrap();

    let mut camera = Camera2dBundle::default();

    camera.projection.scale = 0.5;

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
