use bevy::{prelude::*, window::PrimaryWindow};
use bevy_ecs_ldtk::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(LdtkPlugin)
        .insert_resource(LevelSelection::index(0))
        .add_systems(Startup, init)
        .register_ldtk_entity::<PlayerBundle>("Player")
        .run();

}

#[derive(Default, Component)]
struct Player;

#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    player: Player,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
}

fn init(window: Query<&Window, With<PrimaryWindow>>, mut commands: Commands, asset_server: Res<AssetServer>) {
    let window = window.get_single().unwrap();

    let mut camera = Camera2dBundle::default();

    camera.transform.translation.x = window.width() / 2.0;
    camera.transform.translation.y = window.height() / 2.0;

    commands.spawn(camera);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("gamejam.ldtk"),
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..Default::default()
    });
}

fn camera_follows_player(mut camera: Query<&mut Transform, With<Camera>>, player: Query<&Transform, With<Player>>) {
    if let Ok(mut camera_pos) = camera.get_single_mut() {
        if let Ok(player_pos) = player.get_single() {
            *camera_pos = *player_pos;
        }
    }
}
