// Code mostly from https://bevyengine.org/examples/3d-rendering/update-gltf-scene/
//! Update a scene from a glTF file, either by spawning the scene as a child of another entity,
//! or by accessing the entities of the scene.
#![allow(unused_must_use)] // one dbg!

use bevy::{pbr::DirectionalLightShadowMap, prelude::*};
use bevy_ggrs::{
    ggrs::SessionBuilder, AddRollbackCommandExtension, GgrsApp, GgrsConfig, GgrsPlugin,
    GgrsSchedule, LocalInputs, LocalPlayers, ReadInputs, Session,
};

type Config = GgrsConfig<u8>;

fn main() {
    App::new()
        .add_plugins(GgrsPlugin::<Config>::default())
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(ReadInputs, read_local_inputs)
        .add_systems(GgrsSchedule, move_scene_entities)
        .insert_resource(Session::SyncTest(
            SessionBuilder::<Config>::new()
                .start_synctest_session()
                .unwrap(),
        ))
        // Removing this will change the entity counts (see: move_scene_entities)
        //.rollback_component_with_clone::<SceneRoot>()
        // These have no impact, can be commented or not
        .rollback_component_with_clone::<Transform>()
        .rollback_component_with_clone::<GlobalTransform>()
        .rollback_component_with_clone::<MovedScene>()
        .run();
}

fn read_local_inputs(mut commands: Commands, local_players: Res<LocalPlayers>) {
    commands.insert_resource(LocalInputs::<Config>(
        local_players.0.iter().map(|handle| (*handle, 0)).collect(),
    ));
}

#[derive(Component, Clone)]
struct MovedScene;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Transform::from_xyz(4.0, 25.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-0.5, 0.9, 1.5).looking_at(Vec3::new(-0.5, 0.3, 0.0), Vec3::Y),
        EnvironmentMapLight {
            diffuse_map: asset_server.load("environment_maps/pisa_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: asset_server.load("environment_maps/pisa_specular_rgb9e5_zstd.ktx2"),
            intensity: 150.0,
            ..default()
        },
    ));

    // Spawn the scene as a child of this entity at the given transform
    commands
        .spawn((
            Transform::from_xyz(-1.0, 0.0, 0.0),
            SceneRoot(asset_server.load(
                GltfAssetLabel::Scene(0).from_asset("models/FlightHelmet/FlightHelmet.gltf"),
            )),
        ))
        .add_rollback();

    // Spawn a second scene, and add a tag component to be able to target it later
    commands
        .spawn((
            SceneRoot(asset_server.load(
                GltfAssetLabel::Scene(0).from_asset("models/FlightHelmet/FlightHelmet.gltf"),
            )),
            MovedScene,
        ))
        .add_rollback();
}

// This system will move all entities that are descendants of MovedScene (which will be all entities spawned in the scene)
fn move_scene_entities(
    time: Res<Time>,
    moved_scene: Query<Entity, With<MovedScene>>,
    children: Query<&Children>,
    mut transforms: Query<&mut Transform>,
) {
    dbg!(
        // Outputs 56 if SceneRoot is rolled back, otherwise same as default (30)
        transforms.iter().count(),
        // Outputs 28 if SceneRoot is rolled back, otherwise two less than default (14)
        children.iter().count()
    );
    for moved_scene_entity in &moved_scene {
        let helmet_children = children.get(moved_scene_entity);
        dbg!(
            moved_scene_entity, // The helmet root seems to exist
            helmet_children     // Err, helmet root has no children component?
        );
        let mut offset = 0.;
        for entity in children.iter_descendants(moved_scene_entity) {
            dbg!(entity); // This doesn't happen, iter_descendants doesn't work
                          // Despite this, the helmet is visible
            if let Ok(mut transform) = transforms.get_mut(entity) {
                dbg!(&transform); // Obviously we never get here
                transform.translation = Vec3::new(
                    offset * ops::sin(time.elapsed_secs()) / 20.,
                    0.,
                    ops::cos(time.elapsed_secs()) / 20.,
                );
                offset += 0.5;
            }
        }
    }

    // This assert is used to exit sooner, so we can see how the numbers behave
    //assert!(children.iter().count() < 14);
    // Transform count starts off at 4, child count starts off at 0
    // Then over a single tick they go to the values documented above
}
