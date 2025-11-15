use bevy::prelude::*;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 摄像机
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-5.0, 5.0, 12.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
    ));

    // 光源
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
        GlobalTransform::default(),
    ));

    // 颜色
    let color = Color::srgb(0.3, 0.8, 0.4);

    // 网格 + 材质（注意：Mesh3d/MeshMaterial3d 分别是组件）
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(50.0, 10.0, 50.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: color,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
    ));

    // 再来一个立方体看看位置与光照
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.2),
            ..default()
        })),
        Transform::from_xyz(0.0, 3.0, 0.0),
        GlobalTransform::default(),
    ));
}
