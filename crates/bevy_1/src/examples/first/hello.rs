use bevy::prelude::*;
#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}
fn print_position_system(query: Query<&Position>) {
    for position in &query {
        println!("position: {} {}", position.x, position.y);
    }
}
struct Entity(u64);
fn hello_world() {
    println!("hello world!,");
}
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);
fn add_people(mut commands: Commands) {
    commands.spawn((
        Person,
        Name("Elaina Proctor".to_string()),
    ));
    commands
        .spawn((Person, Name("Renzo Hume".to_string())));
    commands
        .spawn((Person, Name("Zayna Nieves".to_string())));
    commands
        .spawn((Person, Name("world killer".to_string())));
}
#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Person>>,
) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}
fn update_people(
    mut query: Query<&mut Name, With<Person>>,
) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break; // We don't need to change any other names.
        }
    }
}

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
            50.0 * time.delta().as_secs_f32();
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
        app.insert_resource(GreetTimer(
            Timer::from_seconds(1.0, TimerMode::Repeating),
        ));
        app.add_systems(
            Startup,
            (add_people, setup_2d, setup_a_tangle),
        );
        app.add_systems(
            Update,
            (
                hello_world,
                (update_people, greet_people).chain(),
                move_system,
            ),
        );
    }
}
