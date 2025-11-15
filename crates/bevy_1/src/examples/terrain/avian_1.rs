use avian3d::prelude::*;
use bevy::prelude::*;

pub fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
        ))
        .add_systems(Startup, setup_3d)
        .run();
}

fn setup_3d(mut commands: Commands) {
    // 3D 摄像机（Camera3d 组件 + Transform）
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 500.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
    ));

    // 点光源（PointLight 组件 + Transform）
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(200.0, 200.0, 200.0),
        GlobalTransform::default(),
    ));

    // 一个动态立方体（Avian 3D 刚体 + 碰撞体）
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(0.5, 0.5, 0.5),
        Transform::from_xyz(0.0, 3.0, 0.0),
        GlobalTransform::default(),
    ));

    // 一个静态地面
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(10.0, 0.1, 10.0),
        Transform::from_xyz(0.0, -0.5, 0.0),
        GlobalTransform::default(),
    ));
}
