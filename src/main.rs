use bevy::prelude::*;

const SPEED: f32 = 100.;

fn main() {
    App::new()
        .add_systems(Startup, setup)
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Update, move_player)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 0.3,
            ..OrthographicProjection::default_2d()
        }),
    ));

    let texture = asset_server.load("characters/player.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 10, 4, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        ),
        Transform::from_xyz(0., 0., 0.),
        Player,
    ));
}

fn move_player(
    time: Res<Time>,
    mut player_transform: Single<&mut Transform, With<Player>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let mut direction = Vec2::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }

    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
    }

    if keyboard.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
    }

    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }

    if direction != Vec2::ZERO {
        let delta = direction.normalize_or_zero() * time.delta_secs() * SPEED;

        player_transform.translation.x += delta.x;
        player_transform.translation.y += delta.y;
    }
}

#[derive(Component)]
struct Player;
