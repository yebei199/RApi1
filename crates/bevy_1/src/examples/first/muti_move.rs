use bevy::prelude::*;
use rand::Rng;
// 用于生成随机数

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

fn setup_2d(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_entities(mut commands: Commands) {
    let mut rng = rand::rng();

    for i in 0..1000 {
        // 随机方向和速度
        let vx = rng.random_range(-100.0..100.0);
        let vy = rng.random_range(-100.0..100.0);

        commands.spawn((
            Sprite {
                color: Color::Srgba(Srgba {
                    red: rng.random_range(0.0..1.0),
                    green: rng.random_range(0.0..1.0),
                    blue: rng.random_range(0.0..1.0),
                    alpha: 1.0,
                }),
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            Transform::from_xyz(i as f32 * 60.0, 0.0, 0.0),
            GlobalTransform::default(),
            Velocity { x: vx, y: vy },
        ));
    }
}

fn move_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity)>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x +=
            velocity.x * time.delta().as_secs_f32();
        transform.translation.y +=
            velocity.y * time.delta().as_secs_f32();
    }
}

pub struct MutiMovePlugin;
impl Plugin for MutiMovePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (setup_2d, setup_entities),
        );
        app.add_systems(Update, move_system);
    }
}
