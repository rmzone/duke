use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use duke::helper;
use bevy_spritesheet_animation::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins.build()
            .set(WindowPlugin{
                primary_window: Some(Window {
                    title: String::from(
                        "Duke",
                    ),
                    ..Default::default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()))
        .add_plugins(TiledPlugin::default())
        .add_plugins(helper::HelperPlugin)
        .add_plugins(AnimationPlugin)
        .add_plugins(TiledDebugPluginGroup) // see for how to display: https://bevy-cheatbook.github.io/cookbook/print-framerate.html
        .add_systems(Startup, startup)
        .add_systems(Update, control_character)
        .run()
}

fn control_character(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    character: Single<(
        Entity,
        &mut Sprite,
        &mut SpritesheetAnimation,
        &mut Transform,
        //Option<&Shooting>, TODO: add player direction
    )>,
    my_animations: Res<PlayerAnimations>,
    mut messages: MessageReader<AnimationEvent>,
){
    let (entity, mut sprite, mut animation, mut transform) = character.into_inner();

    if keyboard.pressed(KeyCode::ArrowLeft) {
        if animation.animation != my_animations.idle_west {
            animation.switch(my_animations.idle_west.clone());
        }
    }
    else if keyboard.pressed(KeyCode::ArrowRight) {
        if animation.animation != my_animations.idle_east {
            animation.switch(my_animations.idle_east.clone());
        }
    }
    else if keyboard.pressed(KeyCode::ArrowUp) {
        if animation.animation != my_animations.idle_north {
            animation.switch(my_animations.idle_north.clone());
        }
    }
    else if keyboard.pressed(KeyCode::ArrowDown) {
        if animation.animation != my_animations.idle_south {
            animation.switch(my_animations.idle_south.clone());
        }
    }
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut animations: ResMut<Assets<Animation>>,
) {
    commands.spawn(Camera2d);

    // Load a map asset and retrieve its handle
    let map_handle: Handle<TiledMapAsset> = asset_server.load("levels/level1.tmx");

    // Spawn a new entity with the TiledMap component
    commands.spawn((
        TiledMap(map_handle),
        TilemapAnchor::Center,
        // For isometric maps, it can be useful to tweak `bevy_ecs_tilemap` render settings.
        // [`TilemapRenderSettings`] provides the `y_sort`` parameter to sort chunks using their y-axis
        // position during rendering.
        // However, it applies to whole chunks, not individual tile, so we have to force the chunk
        // size to be exactly one tile along the y-axis.
        TilemapRenderSettings {
            render_chunk_size: UVec2::new(64, 1),
            y_sort: true, // important for the tilemap to display correctly
        },
    ));

    // DONE: make sure player and map have the same scale
    // TODO: track camera to player
    // TODO: collisions and display player behind tiles
    // TODO: remove helpers once all the base code is working (maybe have a toggle to help in future dev?)

    // Add dummy player
    // Load the sprite sheet using the `AssetServer`
    let player_image = asset_server.load("entities/player/GUMDROP.E64.R.PNG");
    let spritesheet = Spritesheet::new(&player_image, 10, 48);

    let idle_south_animation = spritesheet
        .create_animation()
        .add_horizontal_strip(0, 0, 7)
        .set_duration(AnimationDuration::PerRepetition(1500))
        .set_repetitions(AnimationRepeat::Loop)
        .build();

    let idle_east_animation = spritesheet
        .create_animation()
        .add_horizontal_strip(0, 1, 7)
        .set_duration(AnimationDuration::PerRepetition(1500))
        .set_repetitions(AnimationRepeat::Loop)
        .build();

    let idle_north_animation = spritesheet
        .create_animation()
        .add_horizontal_strip(0, 2, 7)
        .set_duration(AnimationDuration::PerRepetition(1500))
        .set_repetitions(AnimationRepeat::Loop)
        .build();

    let idle_west_animation = spritesheet
        .create_animation()
        .add_horizontal_strip(0, 3, 7)
        .set_duration(AnimationDuration::PerRepetition(1500))
        .set_repetitions(AnimationRepeat::Loop)
        .build();

    let idle_south_animation_handle = animations.add(idle_south_animation);
    let idle_east_animation_handle = animations.add(idle_east_animation);
    let idle_north_animation_handle = animations.add(idle_north_animation);
    let idle_west_animation_handle = animations.add(idle_west_animation);

    // Store the animations as a resource
    commands.insert_resource(PlayerAnimations {
        idle_south: idle_south_animation_handle.clone(),
        idle_east: idle_east_animation_handle,
        idle_north: idle_north_animation_handle,
        idle_west: idle_west_animation_handle,
    });

    let sprite = spritesheet
        // .with_loaded_image(&player_image)
        .with_size_hint(640, 3072)
        .sprite(&mut texture_atlas_layouts);

    commands.spawn((
        sprite,
        SpritesheetAnimation::new(idle_south_animation_handle),
        Transform::from_scale(Vec3::splat(2.0))
            .with_translation(Vec3::new(0.0, 0.0, 0.0)),

    ));

}

fn spawn_player() {
    // TODO: with PlayerBundle?
}

#[derive(Resource)]
struct PlayerAnimations {
    idle_south: Handle<Animation>,
    idle_east: Handle<Animation>,
    idle_north: Handle<Animation>,
    idle_west: Handle<Animation>,
    // ...
}
