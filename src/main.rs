#[allow(unused_imports)]
use bevy::prelude::*;

#[allow(unused_imports)]
use avian2d::prelude::*;

#[allow(unused_imports)]
use rand::prelude::*;

#[derive(Component)]
struct Plate {
    timer: Timer,
}

#[derive(Component)]
struct Player;

#[derive(Resource)]
struct PlateSpawnTimer(Timer);

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_bg(mut commands: Commands, asset_server: Res<AssetServer>,) {
    commands.spawn((
            Sprite::from_image(asset_server.load("bg.png")),
            Transform::from_xyz(0.0, 0.0, -10.0).with_scale(Vec3::splat(0.35)),
    ));
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Sprite::from_image(asset_server.load("platform.png")),
        Transform::from_xyz(0.0, -200.0, 0.0).with_scale(Vec3::splat(0.15)),
        Player,
        RigidBody::Kinematic,
        Collider::rectangle(2000.0, 400.0),
        Friction::new(100.0),
        Restitution::new(0.),
    ));
}

fn move_player(mut q_player: Query<&mut LinearVelocity, With<Player>>, keys: Res<ButtonInput<KeyCode>>) {
    let mut velocity = q_player.single_mut().unwrap();
    let mut x = 0.0;
    if keys.pressed(KeyCode::KeyA) {
        x -= 100.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        x += 100.0;
    }
    velocity.x = x;
}

fn spawn_plates(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<PlateSpawnTimer>,
    asset_server: Res<AssetServer>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = rand::rng();
        let random_loc = rng.random_range(-400.0..400.0);
        commands.spawn((
            Sprite::from_image(asset_server.load("plates.png")),
            Transform::from_xyz(random_loc, 350.0, 0.0).with_scale(Vec3::splat(0.1)),
            Plate {timer: Timer::from_seconds(2.0, TimerMode::Once)},
            RigidBody::Dynamic,
            Collider::rectangle(1500.0, 400.0),
            Friction::new(100.0),
            Restitution::new(0.0),
            ColliderDensity(1000.0),
            GravityScale(25.0),
        ));
    }
}

#[allow(dead_code)]
fn despawn_plates(
    q_plates: Query<(Entity, &Plate), With<Plate>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    time.delta();
    for (entity, plate) in q_plates {
        if plate.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

#[allow(dead_code)]
fn victory(
    q_plates: Query<&Plate>,
    mut commands: Commands,
) {
    let plates_count = q_plates.iter().len();
    if plates_count >= 2 {
        commands.spawn((
            Text::new("Victory"),
        ));
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        //.add_plugins(PhysicsDebugPlugin::default())
        .insert_resource(PlateSpawnTimer(Timer::from_seconds(4.0, TimerMode::Repeating)))
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_bg)
        .add_systems(Update, spawn_plates)
        .add_systems(Update, move_player)
        .run();
}
