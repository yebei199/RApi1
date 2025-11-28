use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
    z: f32,
}

fn setup_3d(mut commands: Commands) {
    // 使用 Camera3d 组件 + Transform 组件来创建 3D 相机
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 500.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(200.0, 200.0, 200.0),
    ));
}

fn setup_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::rng();

    for _ in 0..1000 {
        let vx = rng.random_range(-100.0..100.0);
        let vy = rng.random_range(-100.0..100.0);
        let vz = rng.random_range(-100.0..100.0);

        let color = Color::Srgba(Srgba {
            red: rng.random_range(0.0..1.0),
            green: rng.random_range(0.0..1.0),
            blue: rng.random_range(0.0..1.0),
            alpha: 1.0,
        });

        commands.spawn((
            Mesh3d(
                meshes.add(Cuboid::new(50.0, 10.0, 50.0)),
            ),
            MeshMaterial3d(materials.add(
                StandardMaterial {
                    base_color: color,
                    ..default()
                },
            )),
            Transform::from_xyz(0.0, 0.0, 0.0),
            Velocity {
                x: vx,
                y: vy,
                z: vz,
            },
        ));
    }
}

fn move_system(
    time: Res<Time>,
    mut q: Query<(&mut Transform, &Velocity)>,
) {
    let mut rng = rand::rng();
    let rang1 = rng.random_range(1.0..5.0)
        * time.delta().as_secs_f32();

    for (mut transform, v) in &mut q {
        transform.translation.x += v.x * rang1;
        transform.translation.y += v.y * rang1;
        transform.translation.z += v.z * rang1;
    }
}

pub struct MutiMovePlugin;
impl Plugin for MutiMovePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (setup_3d, setup_entities),
        )
        .add_systems(Update, move_system);
    }
}
