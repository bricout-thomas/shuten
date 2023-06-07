use bevy::prelude::*;

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(move_circle_flight)
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
