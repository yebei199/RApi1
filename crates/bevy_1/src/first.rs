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
pub fn main_out() {
    // App::new().add_systems(Update, hello_world).run();
    main()
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
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
fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
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
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
        app.add_systems(Startup, add_people);
        app.add_systems(
            Update,
            (
                hello_world,
                (update_people, greet_people).chain(),
            ),
        );
    }
}
