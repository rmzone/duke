use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use duke::helper;
use std::time::Duration;

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
        .add_plugins(TiledDebugPluginGroup) // see for how to display: https://bevy-cheatbook.github.io/cookbook/print-framerate.html
        .add_systems(Startup, startup)
        .add_systems(Update, execute_animations)
        .run()
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
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
            y_sort: true,
        },
    ));

    // TODO: make sure player and map have the same scale
    // TODO: track camera to player
    // TODO: collisions and display player behind tiles

    // Add dummy player
    // Load the sprite sheet using the `AssetServer`
    let texture = asset_server.load("entities/player/GUMDROP.E64.R.PNG");

    // The sprite sheet has 10 sprites arranged in a row, and they are all 64px x 64px
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 10, 48, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // The first (left-hand) sprite runs at 10 FPS
    let animation_config = AnimationConfig::new(1, 6, 10);

    // Create the first (left-hand) sprite
    commands.spawn((
        Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: animation_config.first_sprite_index,
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(2.0))
            .with_translation(Vec3::new(0.0, 0.0, 0.0)),
        animation_config,
    ));
}

// This system loops through all the sprites in the `TextureAtlas`, from  `first_sprite_index` to
// `last_sprite_index` (both defined in `AnimationConfig`).
fn execute_animations(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite)>) {
    for (mut config, mut sprite) in &mut query {
        // We track how long the current sprite has been displayed for
        config.frame_timer.tick(time.delta());

        // If it has been displayed for the user-defined amount of time (fps)...
        if config.frame_timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            if atlas.index == config.last_sprite_index {
                // ...and it IS the last frame, then we move back to the first frame and stop.
                atlas.index = config.first_sprite_index;
            } else {
                // ...and it is NOT the last frame, then we move to the next frame...
                atlas.index += 1;
                // ...and reset the frame timer to start counting all over again
                config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
            }
        }
    }
}

#[derive(Component)]
struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}
