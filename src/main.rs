use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use duke::helper;

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
        // .add_systems(Update, switch_map)
        .run()
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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

    // Add dummy player
    let player =meshes.add(Capsule2d::new(25.0, 50.0));
    let color = Color::hsl(360., 0.95, 0.7);

    commands.spawn((
        Mesh2d(player),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(
            -0.0,
            0.0,
            0.0,
        ),
    ));
}
