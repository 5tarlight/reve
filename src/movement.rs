use bevy::{
    prelude::{Commands, Component, Entity, Plugin, Query, Res, Transform, Vec2},
    time::Time,
};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(move_to_point);
    }
}

#[derive(Component)]
pub struct Velocity(f32);

#[derive(Component)]
pub struct MoveToPoint(Vec2);

#[derive(Component)]
pub struct MoveToPosition(f32);

fn move_to_point(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Velocity, &MoveToPoint)>,
    time: Res<Time>,
) {
    for (entity, mut tf, v, p) in query.iter_mut() {
        let x_gap = p.0.x - tf.translation.x;
        let x_gap = x_gap as f64;
        let y_gap = p.0.y - tf.translation.y;
        let y_gap = y_gap as f64;
        let distance = f64::sqrt(x_gap * x_gap + y_gap * y_gap);
        let sin = y_gap / distance;
        let sin = sin as f32;
        let cos = x_gap / distance;
        let cos = cos as f32;
        let d = time.delta().as_secs_f32();

        let mut next_x = tf.translation.x + cos * v.0 * d;
        if tf.translation.x < p.0.x {
            // Move Right
            next_x = p.0.x.min(next_x);
        } else {
            // Move Bottom
            next_x = p.0.x.max(next_x);
        }

        let mut next_y = tf.translation.y + sin * v.0 * d;
        if tf.translation.y < p.0.y {
            // Move Up
            next_y = p.0.y.min(next_y);
        } else {
            // Move Down
            next_y = p.0.y.max(next_y);
        }

        if next_x == p.0.x && next_y == p.0.y {
            commands.entity(entity).remove::<MoveToPoint>();
        }

        tf.translation.x = next_x;
        tf.translation.y = next_y;
    }
}
