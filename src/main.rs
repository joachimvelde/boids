use bevy::prelude::*;
use bevy_flycam::{KeyBindings, MovementSettings, PlayerPlugin};
use rand::distributions::{Distribution, Uniform};

static X_WORLD: f32 = 50.0;
static Y_WORLD: f32 = 20.0;
static Z_WORLD: f32 = 50.0;

static N_BOIDS: i32 = 500;
static R_BOIDS: f32 = 0.1;

static MAX_SPEED: f32 = 10.0;
static MIN_SPEED: f32 = 10.0;

static PROTECTED_RANGE: f32 = 1.0;
static AVOID_FACTOR: f32 = 1.0;

static VISIBLE_RANGE: f32 = 10.0;
static MATCHING_FACTOR: f32 = 0.01;

static CENTERING_FACTOR: f32 = 0.05;

static MARGIN: f32 = 2.0;
static TURN_FACTOR: f32 = 1.0;

#[derive(Component, Copy, Clone)]
struct Boid {
    dx: f32,
    dy: f32,
    dz: f32
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .insert_resource(MovementSettings {
            sensitivity: 0.00015,
            speed: 12.0
        })
        .insert_resource(KeyBindings {
            move_ascend: KeyCode::KeyE,
            move_descend: KeyCode::KeyQ,
            ..default()
        })
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(X_WORLD, 0.1, Z_WORLD)),
        material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    let x_bound = X_WORLD / 2.0;
    let y_bound = Y_WORLD;
    let z_bound = Z_WORLD / 2.0;

    let mut rng = rand::thread_rng();
    let x_gen = Uniform::from(-x_bound..x_bound);
    let y_gen = Uniform::from(0.0..y_bound);
    let z_gen = Uniform::from(-z_bound..z_bound);

    for _ in 0..N_BOIDS {
        let x: f32 = x_gen.sample(&mut rng);
        let y: f32 = y_gen.sample(&mut rng);
        let z: f32 = z_gen.sample(&mut rng);

        commands.spawn(PbrBundle {
            mesh: meshes.add(Sphere::new(R_BOIDS)),
            material: materials.add(Color::RED),
            transform: Transform::from_xyz(x, y, z),
            ..default()
        })
        .insert(Boid {
            dx: MIN_SPEED,
            dy: MIN_SPEED,
            dz: MIN_SPEED
        });
    }
}

fn update(time: Res<Time>, mut query: Query<(Entity, &mut Boid, &mut Transform)>) {
    let boids: Vec<(Entity, Boid, Transform)> = query.iter_mut().map(|(e, b, t)| (e, *b, *t)).collect();

    for (_, mut boid, mut transform) in &mut query.iter_mut() {

        // Separation - Avoid other boids
        let mut close_dx: f32 = 0.0;
        let mut close_dy: f32 = 0.0;
        let mut close_dz: f32 = 0.0;

        // Alignment - Match other boids' velocity
        let mut xvel_avg: f32 = 0.0;
        let mut yvel_avg: f32 = 0.0;
        let mut zvel_avg: f32 = 0.0;
        
        // Cohesion - Steer toward the middle of the flock
        let mut xpos_avg: f32 = 0.0;
        let mut ypos_avg: f32 = 0.0;
        let mut zpos_avg: f32 = 0.0;

        let mut neighbouring_boids: f32 = 0.0;

        for (_, other_boid, other_transform) in &boids {
            let distance: f32 = transform.translation.distance(other_transform.translation);

            // Separation
            if distance < PROTECTED_RANGE {
                close_dx += transform.translation.x - other_transform.translation.x;
                close_dy += transform.translation.y - other_transform.translation.y;
                close_dz += transform.translation.z - other_transform.translation.z;
            }

            if distance < VISIBLE_RANGE {
                // Alignment
                xvel_avg += other_boid.dx;
                yvel_avg += other_boid.dy;
                zvel_avg += other_boid.dz;
                
                // Cohesion
                xpos_avg += other_transform.translation.x;
                ypos_avg += other_transform.translation.y;
                zpos_avg += other_transform.translation.z;

                neighbouring_boids += 1.0;
            }
        }

        if neighbouring_boids > 0.0 {
            xvel_avg /= neighbouring_boids;
            yvel_avg /= neighbouring_boids;
            zvel_avg /= neighbouring_boids;

            xpos_avg /= neighbouring_boids;
            ypos_avg /= neighbouring_boids;
            zpos_avg /= neighbouring_boids;
        }

        // Separation
        boid.dx += close_dx * AVOID_FACTOR;
        boid.dy += close_dy * AVOID_FACTOR;
        boid.dz += close_dz * AVOID_FACTOR;

        // Alignment
        boid.dx += (xvel_avg - boid.dx) * MATCHING_FACTOR;
        boid.dy += (yvel_avg - boid.dy) * MATCHING_FACTOR;
        boid.dz += (zvel_avg - boid.dz) * MATCHING_FACTOR;
        
        // Cohesion
        boid.dx += (xpos_avg - transform.translation.x) * CENTERING_FACTOR;
        boid.dy += (ypos_avg - transform.translation.y) * CENTERING_FACTOR;
        boid.dz += (zpos_avg - transform.translation.z) * CENTERING_FACTOR;

        // Attempt to avoid screen edges
        if transform.translation.x > (X_WORLD / 2.0 - MARGIN) {
            boid.dx -= TURN_FACTOR;
        }
        if transform.translation.x < (- X_WORLD / 2.0 + MARGIN) {
            boid.dx += TURN_FACTOR;
        }
        if transform.translation.y > (Y_WORLD - MARGIN) {
            boid.dy -= TURN_FACTOR;
        }
        if transform.translation.y < MARGIN {
            boid.dy += TURN_FACTOR;
        }
        if transform.translation.z > (Z_WORLD / 2.0 - MARGIN) {
            boid.dz -= TURN_FACTOR;
        }
        if transform.translation.z < (- Z_WORLD / 2.0 + MARGIN) {
            boid.dz += TURN_FACTOR;
        }

        // Limit the speed
        let speed: f32 = (boid.dx*boid.dx + boid.dy*boid.dy + boid.dz*boid.dz).sqrt();
        if speed > MAX_SPEED {
            boid.dx = (boid.dx / speed) * MAX_SPEED;
            boid.dy = (boid.dy / speed) * MAX_SPEED;
            boid.dz = (boid.dz / speed) * MAX_SPEED;
        }
        if speed < MIN_SPEED {
            boid.dx = (boid.dx / speed) * MIN_SPEED;
            boid.dy = (boid.dy / speed) * MIN_SPEED;
            boid.dz = (boid.dz / speed) * MIN_SPEED;
        }

        // Update position
        transform.translation.x += boid.dx * time.delta_seconds();
        transform.translation.y += boid.dy * time.delta_seconds();
        transform.translation.z += boid.dz * time.delta_seconds();
        
        // Keep the boid inside bounds
        if transform.translation.x > X_WORLD / 2.0 {
            transform.translation.x = - X_WORLD / 2.0;
        }
        if transform.translation.x < - X_WORLD / 2.0 {
            transform.translation.x = X_WORLD / 2.0;
        }
        if transform.translation.y > Y_WORLD {
            transform.translation.y = 0.0
        }
        if transform.translation.y < 0.0 {
            transform.translation.y = Y_WORLD;
        }
        if transform.translation.z > Z_WORLD / 2.0 {
            transform.translation.z = - Z_WORLD / 2.0;
        }
        if transform.translation.z < - Z_WORLD / 2.0 {
            transform.translation.z = Z_WORLD / 2.0;
        }
    }
}
