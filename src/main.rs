use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            // This sets image filtering to nearest
            // This is done to prevent textures with low resolution (e.g. pixel art) from being blurred
            // by linear filtering.
            ImagePlugin::default_nearest(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .run();
}

#[derive(Component)]
enum Direction {
    Left,
    Right,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("dog.png"),
            transform: Transform::from_xyz(-200., -200., 0.).with_scale(Vec3::splat(0.1)),
            ..default()
        },
        Direction::Right,
    ));
    commands.spawn(SpriteBundle {
        texture: asset_server.load("bg.png"),
        transform: Transform::from_xyz(0., 0., -1.),
        ..default()
    });
}

fn sprite_movement(
    time: Res<Time>,
    mut sprite_position: Query<(&mut Direction, &mut Transform, &mut Sprite)>,
) {
    for (mut logo, mut transform, mut sprite) in &mut sprite_position {
        match *logo {
            Direction::Right => {
                transform.translation.x += 30. * time.delta_seconds();
                sprite.flip_x = false
            }
            Direction::Left => {
                transform.translation.x -= 30. * time.delta_seconds();
                sprite.flip_x = true
            }
        }

        if transform.translation.x > 200. {
            *logo = Direction::Left;
        } else if transform.translation.x < -200. {
            *logo = Direction::Right;
        }
    }
}
