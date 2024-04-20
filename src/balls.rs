use bevy::{
    prelude::*,
    window::{Window, WindowResolution, WindowPlugin},
    sprite::{MaterialMesh2dBundle, Mesh2dHandle}
};
use rand::distributions::{Distribution, Uniform};

static WIDTH:  f32 = 1600.0;
static HEIGHT: f32 = 1000.0;

#[derive(Component)]
struct Boid {
    velocity: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Boids".to_string(),
                resolution: WindowResolution::new(WIDTH, HEIGHT),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, (setup_camera, add_boids))
        .add_systems(Update, update_boids)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn add_boids(mut commands: Commands, asset_server: Res<AssetServer>) {
    let w = WIDTH / 2.0;
    let h = HEIGHT / 2.0;

    let mut rng = rand::thread_rng();
    let x_gen = Uniform::from(-w..w);
    let y_gen = Uniform::from(-h..h);

    for _ in 0..10 {
        let x: f32 = x_gen.sample(&mut rng);
        let y: f32 = y_gen.sample(&mut rng);

        commands.spawn(SpriteBundle {
            texture: asset_server.load("sprite.png"),
            transform: Transform::from_xyz(100.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(Boid { velocity: 5.0 });
    }
}

fn update_boids(query: Query<(&Boid, &mut Transform)>) {
    for (_boid, mut transform) in &query {
        // TODO: Update position of current boid
        println!("x: {}, y: {}", transform.translation.x, transform.translation.y);
    }
}
