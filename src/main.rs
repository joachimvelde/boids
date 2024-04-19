use rand::distributions::{Distribution, Uniform};
use bevy::{
    prelude::*,
    window::{Window, WindowResolution, WindowPlugin}
};

static WIDTH: f32 = 1600.0;
static HEIGHT: f32 = 1000.0;
static N_BOIDS: i32 = 500;

static PROTECTED_RANGE: f32 = 30.0;
static AVOID_FACTOR: f32 = 1.0;

static VISIBLE_RANGE: f32 = 200.0;
static MATCHING_FACTOR: f32 = 0.1;

static CENTERING_FACTOR: f32 = 0.1;

#[derive(Component, Copy, Clone)]
struct Boid {
    dx: f32,
    dy: f32
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Boids".to_string(),
                resolution: WindowResolution::new(WIDTH, HEIGHT),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let w = WIDTH / 2.0;
    let h = HEIGHT / 2.0;

    let mut rng = rand::thread_rng();
    let x_gen = Uniform::from(-w..w);
    let y_gen = Uniform::from(-h..h);

    for _ in 0..N_BOIDS {
        let x: f32 = x_gen.sample(&mut rng);
        let y: f32 = y_gen.sample(&mut rng);

        commands.spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(5.0, 5.0)),
                ..default()
            },
            transform: Transform::from_xyz(x, y, 0.0),
            ..default()
        })
        .insert(Boid {
            dx: 100.0,
            dy: 100.0
        });
    }
}

// We have to do some goofy shit with the query to be able to use nested loops over the boids
fn update(time: Res<Time>, mut query: Query<(Entity, &mut Boid, &mut Transform)>) {
    let boids: Vec<(Entity, Boid, Transform)> = query.iter_mut().map(|(e, b, t)| (e, *b, *t)).collect();

    for (_, mut boid, mut transform) in &mut query.iter_mut() {
        // Separation - Avoid other boids
        let mut close_dx: f32 = 0.0;
        let mut close_dy: f32 = 0.0;

        // Alignment - Match other boids' velocity
        let mut xvel_avg: f32 = 0.0;
        let mut yvel_avg: f32 = 0.0;

        // Cohesion - Steer toward the middle of the flock
        let mut xpos_avg: f32 = 0.0;
        let mut ypos_avg: f32 = 0.0;

        let mut neighbouring_boids: f32 = 0.0;

        for (_, other_boid, other_transform) in &boids {
            let distance: f32 = transform.translation.distance(other_transform.translation);

            // Separation
            if distance < PROTECTED_RANGE {
                close_dx += transform.translation.x - other_transform.translation.x;
                close_dy += transform.translation.y - other_transform.translation.y;
            }

            if distance < VISIBLE_RANGE {
                // Alignment
                xvel_avg += other_boid.dx;
                yvel_avg += other_boid.dy;

                // Cohesion
                xpos_avg += other_transform.translation.x;
                ypos_avg += other_transform.translation.y;

                neighbouring_boids += 1.0;
            }

        }

        if neighbouring_boids > 0.0 {
            xvel_avg /= neighbouring_boids;
            yvel_avg /= neighbouring_boids;

            xpos_avg /= neighbouring_boids;
            ypos_avg /= neighbouring_boids;
        }

        // Separation
        boid.dx += close_dx * AVOID_FACTOR;
        boid.dy += close_dy * AVOID_FACTOR;

        // Alignment
        boid.dx += (xvel_avg - boid.dx) * MATCHING_FACTOR;
        boid.dy += (yvel_avg - boid.dy) * MATCHING_FACTOR;

        // Cohesion
        boid.dx += (xpos_avg - transform.translation.x) * CENTERING_FACTOR;
        boid.dy += (ypos_avg - transform.translation.y) * CENTERING_FACTOR;

        // Update position
        transform.translation.x += boid.dx * time.delta_seconds();
        transform.translation.y += boid.dy * time.delta_seconds();

        // To make things more interesting we can randomly flip the direction of boids
        // let mut rng = rand::thread_rng();
        // let gen = Uniform::from(0.0..10.0);
        // if gen.sample(&mut rng) < 0.01 {
        //     boid.dx = -boid.dx;
        //     boid.dy = -boid.dy;
        // }

        // Keep boid on screen
        if transform.translation.x > WIDTH / 2.0 {
            transform.translation.x = - WIDTH / 2.0;
        }
        if transform.translation.x < - WIDTH / 2.0 {
            transform.translation.x = WIDTH / 2.0;
        }
        if transform.translation.y > HEIGHT / 2.0 {
            transform.translation.y = - HEIGHT / 2.0;
        }
        if transform.translation.y < - HEIGHT / 2.0 {
            transform.translation.y = HEIGHT / 2.0;
        }
    }
}
