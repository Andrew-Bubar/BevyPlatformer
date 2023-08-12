
use bevy::{prelude::*, transform::commands};

fn main() {
    println!("Hello, world!");

    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(bevy_editor_pls::prelude::EditorPlugin::default())

    /*Startup systems */
    .add_startup_system(spawn_cam)
    .add_startup_system(spawn_player)
    .run()
}

//camera
fn spawn_cam(
    mut commands: Commands,
){
    commands.spawn(Camera2dBundle::default());
}

//Player Stuff
#[derive(Component)]
struct Player;

//spawn player
fn spawn_player(
    mut commands: Commands,
    mut texture_atlas: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
){

    let atlas = TextureAtlas::from_grid(
        asset_server.load("Main Characters/Mask Dude/Idle (32x32).png"), 
        Vec2::splat(32.),
        11,
        1,
        None,
        None);

    commands.spawn((SpriteSheetBundle {
        texture_atlas: texture_atlas.add(atlas),
        sprite: TextureAtlasSprite { index: 0, ..Default::default() },
        ..Default::default()
    }, Player ));
}

