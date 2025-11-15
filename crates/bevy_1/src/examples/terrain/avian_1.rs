use avian3d::prelude::*;
use bevy::prelude::*;
pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_3d)
        .run();
}

fn setup_3d(
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

    // 光源（只影响照明，不会显示）
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
        GlobalTransform::default(),
    ));

    // 如果你想“看到光源”，额外放一个小球表示光源位置
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.3, 0.3, 0.3))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 1.0, 0.0),
            // emissive: Color::srgb(1.0, 1.0, 0.0), // 让它发光
            ..default()
        })),
        Transform::from_xyz(4.0, 8.0, 4.0),
        GlobalTransform::default(),
    ));

    // 静态地面：物理 + 渲染
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(50.0, 1.0, 50.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.8, 0.3),
            ..default()
        })),
        Transform::from_xyz(0.0, -0.5, 0.0),
        GlobalTransform::default(),
        RigidBody::Static,
        Collider::cuboid(25.0, 0.5, 25.0),
    ));

    // 动态立方体：物理 + 渲染
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.2),
            ..default()
        })),
        Transform::from_xyz(0.0, 3.0, 0.0),
        GlobalTransform::default(),
        RigidBody::Dynamic,
        Collider::cuboid(0.5, 0.5, 0.5),
    ));
}
