use bevy::prelude::*;

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

// This crate specifies trajectories for bullets and enemies
// One might add multiple flight components and they should add up
// As they all add something to the translation

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(move_circle_flight)
            .add_system(move_linear_flight)
            .add_system(destroy_on_screen_leave)
            ;
    }
}

#[derive(Component)]
pub struct CircleFlight { 
    pub t: f32, 
    pub amplitude: f32,
    pub angular_speed: f32, // in turns per second
}

fn move_circle_flight(
    mut query: Query<(&mut Transform, &mut CircleFlight)>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds(); 
    for (mut transform, mut circle_flight) in query.iter_mut() {
        let r = circle_flight.t + circle_flight.angular_speed * delta;
        circle_flight.t = r;
        transform.translation += Vec3::new(r.sin(), r.cos(), 0.) * circle_flight.amplitude * delta;
    }
}

#[derive(Component)]
pub struct LinearFlight {
    pub velocity: Vec2,
}

impl LinearFlight {
    pub fn from_angle(angle: f32, speed: f32) -> Self {
        LinearFlight {
            velocity: Vec2::from_angle(angle) * speed,
        }
    }
    pub fn from_target(target: Vec2, shoot_position: Vec2, speed: f32) -> Self {
        LinearFlight {
            velocity: (target - shoot_position).normalize() * speed,
        }
    }
}

fn move_linear_flight(
    mut query: Query<(&mut Transform, &LinearFlight)>,
    time: Res<Time>,
) {
    for (mut transform, linear_flight) in query.iter_mut() {
        transform.translation += linear_flight.velocity.extend(0.) * time.delta_seconds();
    }
}

#[derive(Component)]
pub struct DestroyOnScreenLeave {
    pub hitbox: f32,
}

fn destroy_on_screen_leave(
    query: Query<(Entity, &Transform, &DestroyOnScreenLeave)>,
    mut commands: Commands,
) {
    for (entity, transform, des) in query.iter() {
        let position = transform.translation.truncate();
        let s = des.hitbox;
        if position.x+s < -SCREEN_WIDTH || position.x-s > SCREEN_WIDTH || position.y+s < -SCREEN_HEIGHT || position.x-s > SCREEN_HEIGHT {
            commands.entity(entity).despawn_recursive();
        }
    }
}
