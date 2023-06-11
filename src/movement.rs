use bevy::prelude::*;

use crate::{SCREEN_HEIGHT, SCREEN_WIDTH, AppState, HALF_SCREEN_HEIGHT, HALF_SCREEN_WIDTH};

// This crate specifies trajectories for bullets and enemies
// One might add multiple flight components and they should add up
// As they all add something to the translation each frame,
// except for spline defined trajectories

// one can add two time the same trajectory or a spline to another trajectory using a parent node

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                (
                move_circle_flight,
                move_linear_flight,
                ease_out_sine_flight,
                constrain_on_screen
                )
                    .in_set(OnUpdate(AppState::InGame)))

            .add_system(destroy_on_screen_left)
            .add_system(destroy_on_up)

            .register_type::<CircleFlight>()
            .register_type::<LinearFlight>()
            .register_type::<EaseOutSineFlight>()
            .register_type::<ConstrainOnScreen>()
            ;
    }
}

#[derive(Component, Reflect, Default)]
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

#[derive(Component, Reflect, Default)]
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

#[derive(Component, Reflect, Default)]
pub struct EaseOutSineFlight {
    pub path: Vec2, 
    pub t: f32,
    pub time: f32,
}

fn ease_out_sine_flight (
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut EaseOutSineFlight)>,
    time: Res<Time>,
) {
    let delta = time.delta_seconds();
    for (entity, mut transform, mut flight) in query.iter_mut() {
        // EaseOutSine = sin(t*pi/2)
        // EaseOutSineDerivative cos(t*pi/2)*pi/2
        flight.t += delta/flight.time;
        if flight.t > 1. {
            commands.entity(entity).remove::<EaseOutSineFlight>();
        } else {
            let variation_s = (flight.t*std::f32::consts::FRAC_PI_2).cos()*std::f32::consts::FRAC_PI_2*delta;
            transform.translation += (variation_s * flight.path).extend(0.);
        }
    }
}

// insert this on bullets, as they should not surprise the player by appearing from out screen
#[derive(Component)]
pub struct DestroyOnScreenLeft {
    pub hitbox: f32,
}

fn destroy_on_screen_left(
    query: Query<(Entity, &Transform, &DestroyOnScreenLeft)>,
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

// insert this on player generated projectiles, as they can only go up
// exept of course if they don't
#[derive(Component)]
pub struct DestroyOnUp {
    pub hitbox: f32,
}

fn destroy_on_up (
    query: Query<(Entity, &Transform, &DestroyOnUp)>,
    mut commands: Commands,
) {
    for (entity, transform, des) in query.iter() {
        let position = transform.translation.truncate();
        let s = des.hitbox;
        if position.x-s > SCREEN_HEIGHT {
            commands.entity(entity).despawn_recursive();
        }
    }
}

// constrains ( probably only the player ) to the screen
#[derive(Component, Default, Reflect)]
pub struct ConstrainOnScreen { pub half_hitbox: f32 }

fn constrain_on_screen (
    mut query: Query<(&ConstrainOnScreen, &mut Transform)>,
) {
    for (constraint, mut transform) in query.iter_mut() {
        transform.translation.x = transform.translation.x.min(HALF_SCREEN_HEIGHT-constraint.half_hitbox).max(-HALF_SCREEN_HEIGHT+constraint.half_hitbox);
        transform.translation.y = transform.translation.y.min(HALF_SCREEN_WIDTH -constraint.half_hitbox).max(-HALF_SCREEN_WIDTH +constraint.half_hitbox);
    }
}
