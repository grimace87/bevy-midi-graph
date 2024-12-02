use crate::{assets::GameAssets, states::AppState};
use bevy::{gltf::GltfMesh, prelude::*};

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Loading), init_loading_ui)
            .add_systems(OnExit(AppState::Loading), remove_loading_ui)
            .add_systems(OnEnter(AppState::Splash), init_splash_ui);
    }
}

#[derive(Component)]
struct SplashUiRoot;

fn init_loading_ui(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands
        .spawn((
            SplashUiRoot,
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::End,
                align_items: AlignItems::End,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Node::DEFAULT
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    padding: UiRect::all(Val::Px(16.0)),
                    ..default()
                },
                Text("Loading...".to_owned()),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE.into()),
                TextLayout {
                    justify: JustifyText::Center,
                    ..default()
                },
            ));
        });
}

fn remove_loading_ui(
    mut commands: Commands,
    ui_query: Query<Entity, With<SplashUiRoot>>,
    camera_query: Query<Entity, With<Camera>>,
) {
    let ui = ui_query.get_single().unwrap();
    let camera = camera_query.get_single().unwrap();
    commands.entity(ui).despawn_recursive();
    commands.entity(camera).despawn_recursive();
}

fn init_splash_ui(
    mut commands: Commands,
    server: Res<AssetServer>,
    gltf_assets: Res<Assets<Gltf>>,
    gltf_mesh_assets: Res<Assets<GltfMesh>>,
    game_assets: Res<GameAssets>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 3.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let scene: &Gltf = gltf_assets.get(&game_assets.game_scene).unwrap();
    let gltf_mesh: &GltfMesh = gltf_mesh_assets.get(&scene.meshes[0]).unwrap();
    commands.spawn((
        Mesh3d(gltf_mesh.primitives[0].mesh.clone()),
        MeshMaterial3d(server.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.6, 0.2),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
