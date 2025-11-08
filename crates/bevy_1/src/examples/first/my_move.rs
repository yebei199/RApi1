use bevy::prelude::*;

fn setup_2d(mut commands: Commands) {
    commands.spawn(Camera2d);
}
#[derive(Component)]
struct Moving;

fn setup_a_tangle(mut commands: Commands) {
    // 一个彩色矩形精灵
    commands.spawn((
        Sprite {
            color: Color::Srgba(Srgba {
                red: 0.25,
                green: 0.25,
                blue: 0.75,
                alpha: 1.0,
            }),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 10.0, 0.0),
        Moving,
    ));
}

fn move_system(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Moving>>,
) {
    for mut transform in &mut query {
        // 每秒向右移动 50 个单位
        transform.translation.x +=
            1500.0 * time.delta().as_secs_f32();
    }
}

pub struct MyMovePlugin;
impl Plugin for MyMovePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (setup_2d, setup_a_tangle),
        );
        app.add_systems(Update, (move_system,));
    }
}
