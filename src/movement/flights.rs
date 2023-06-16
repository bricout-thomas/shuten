use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
pub struct CircleFlight { 
    pub t: f32, 
    pub amplitude: f32,
    pub angular_speed: f32, // in turns per second
}

pub fn move_circle_flight(
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

pub fn move_linear_flight(
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

pub fn ease_out_sine_flight (
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
