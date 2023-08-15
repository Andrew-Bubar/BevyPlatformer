
use bevy::prelude::*;
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    //.add_plugin(bevy_editor_pls::prelude::EditorPlugin::default())

    /* Resources */
    .init_resource::<Animations>()

    /*Startup systems */
    .add_startup_system(spawn_cam)
    .add_startup_system(spawn_player)

    /* Running Systems */
    .add_system(move_player)
    .add_system(player_fall)
    .add_system(player_jump)
    .add_system(animate_sprite)
    .add_system(change_player_animation)
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
    }, 
    Player,
    SpriteAnimation {
        len: 11,
        frame_time: 1./20.
    },
    FrameTime(0.0),
 ));
}

const MOVE_SPEED: f32 = 100.;
const FALL_SPEED:f32 = 100.;

fn move_player(
    mut commands: Commands,
    mut player: Query<(&mut Transform, Entity), With<Player>>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
){
    let (mut transform, p_entity) = player.single_mut();

    //move left
    if input.any_pressed([KeyCode::A, KeyCode::Left]){

        transform.translation.x -= MOVE_SPEED * time.delta_seconds();
    }

    //move right
    else if input.any_pressed([KeyCode::D, KeyCode::Right]){

        transform.translation.x += MOVE_SPEED * time.delta_seconds();
    }

    if input.any_pressed([KeyCode::Up, KeyCode::Space, KeyCode::W]){
        commands.entity(p_entity).insert(Jump(100.));
    }
}

#[derive(Component)]
struct Jump(f32);

fn player_jump(
    mut commands: Commands,
    time:Res<Time>,
    input: Res<Input<KeyCode>>,
    mut player: Query<(Entity, &mut Transform, &mut Jump), With<Player>>,
){
    let Ok((player, mut transform, mut jump)) = player.get_single_mut() else {return;};

    let jump_power = (time.delta_seconds() * FALL_SPEED * 2.).min(jump.0);

    transform.translation.y += jump_power;

    jump.0 -= if input.any_pressed([KeyCode::Up, KeyCode::Space, KeyCode::W]){
        jump_power 
    } else { jump_power * 2.};

    

    if jump.0 == 0. {commands.entity(player).remove::<Jump>();}
}

fn player_fall(
    mut player: Query<&mut Transform, (With<Player>, Without<Jump>)>,
    time: Res<Time>,
){
    let Ok(mut player) = player.get_single_mut() else {return;};

    if player.translation.y > -4.0 {
        player.translation.y -= time.delta_seconds() * FALL_SPEED;
        if player.translation.y < -4.0 {player.translation.y = -4.0}
    }
}

#[derive(Resource)]
struct Animations {
    map: HashMap<Animation, (Handle<TextureAtlas>, SpriteAnimation)>,
}

impl Animations {
    fn add(
        &mut self, 
        id: Animation, 
        handle: Handle<TextureAtlas>, 
        animation: SpriteAnimation
    ){

        self.map.insert(id, (handle, animation));
    }

    fn get(
        &self, 
        id:Animation) -> Option<(Handle<TextureAtlas>, SpriteAnimation)> {
        self.map.get(&id).cloned()
    } 
}

impl FromWorld for Animations {
    fn from_world(world: &mut World) -> Self {
        let mut map = Animations {map: HashMap::new()};

        world.resource_scope(|world, mut texture_atles: Mut<Assets<TextureAtlas>>| {

        let asset_server = world.resource::<AssetServer>();

        //let mut texture_atles = world.resource_mut::<Assets<TextureAtlas>>();

        //locating the sprites to load
        let idle_atlas = TextureAtlas::from_grid(
            asset_server.load("Main Characters/Mask Dude/Idle (32x32).png"),
            Vec2::splat(32.),
            11, 1, None, None
        );
        let run_atlas = TextureAtlas::from_grid(
            asset_server.load("Main Characters/Mask Dude/Run (32x32).png"),
            Vec2::splat(32.),
            12, 1, None, None
        );

        let jump_atlas = TextureAtlas::from_grid(
            asset_server.load("Main Characters/Mask Dude/Jump (32x32).png"),
            Vec2::splat(32.),
            1, 1, None, None
        );
        
        //adding the textures to Sprite Animationds
        map.add(
        Animation::PlayerIdle,
        texture_atles.add(idle_atlas),
        SpriteAnimation { len: 11, frame_time: 1./20. }
        );

        map.add(
            Animation::PlayerRun,
            texture_atles.add(run_atlas),
            SpriteAnimation { len: 12, frame_time: 1./20. }
            );

        map.add(
            Animation::PlayerJump,
            texture_atles.add(jump_atlas),
            SpriteAnimation { len: 1, frame_time: 1./20. }
            );

        });
        map
    }
}



fn change_player_animation(
    mut player: Query<(
        &mut Handle<TextureAtlas>,
        &mut SpriteAnimation, 
        &mut TextureAtlasSprite), With<Player>>,
    mut player_jump: Query<Option<&Jump>, With<Player>>,
    input: Res<Input<KeyCode>>,
    mut textue_atlas: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    anim: Res<Animations>,
){

    let (mut atlas, mut animation, mut sprite) = player.single_mut();
    let (jump) = player_jump.single_mut();
    
    let set =
    if jump.is_some(){
        Animation::PlayerJump
    } else if input.any_pressed([KeyCode::A, KeyCode::Left, KeyCode::D, KeyCode::Right]) {
        Animation::PlayerRun
    } else {
        Animation::PlayerIdle
    };

    //flipping sprite
    if input.any_just_pressed([KeyCode::A, KeyCode::Left]) {
        sprite.flip_x = true;
    } 
    else if input.any_just_pressed([KeyCode::D, KeyCode::Right])
    && !input.any_pressed([KeyCode::A, KeyCode::Left]) {
        sprite.flip_x = false;
    }
    else if input.any_just_released([KeyCode::A, KeyCode::Left])
    && !input.any_pressed([KeyCode::A, KeyCode::Left])
    && input.any_pressed([KeyCode::D, KeyCode::Right]){
        sprite.flip_x = false;
    }

    let Some((new_atlas, new_animation)) = anim.get(set) else {error!("No Animation Jump Loaded"); return; };

    *atlas = new_atlas;
    sprite.index %= new_animation.len;
    *animation = new_animation;

}

//animate sprites
#[derive(Debug, Hash, PartialEq, Eq)]
enum Animation {
    PlayerRun,
    PlayerIdle,
    PlayerJump,
}

#[derive(Component, Clone, Copy)]
struct SpriteAnimation {
    len: usize,
    frame_time: f32,
}

#[derive(Component)]
struct FrameTime(f32);

fn animate_sprite(
    mut query: Query<(&mut TextureAtlasSprite, &SpriteAnimation, &mut FrameTime)>,
    time: Res<Time>,
){
    for (mut sprite, 
        animation, 
        mut frame_time) in query.iter_mut() {

        frame_time.0 += time.delta_seconds();

        if frame_time.0 > animation.frame_time {

            let frames = (frame_time.0 / animation.frame_time) as usize;
            sprite.index += frames;
            if sprite.index >= animation.len { sprite.index %= animation.len; }

            frame_time.0 -= animation.frame_time * frames as f32;
        }
    }
}