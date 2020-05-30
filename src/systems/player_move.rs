use crate::player::Player;
use amethyst::core::math::Vector3;
use amethyst::{
    core::timing::Time,
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

const VELOCITY: f32 = 5.0;

#[derive(SystemDesc)]
pub struct PlayerMoveSystem;

impl<'s> System<'s> for PlayerMoveSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, players, input, time): Self::SystemData) {
        for (_, transform) in (&players, &mut transforms).join() {
            let frontal_movement = input.axis_value("frontal");
            let lateral_movement = input.axis_value("lateral");
            let lateral_mouse_movement = input.axis_value("mouse_x");

            if let Some(mv_amount) = frontal_movement {
                let frontal_vector = transform.isometry().inverse().rotation * Vector3::y();
                let scaled_amount = VELOCITY * time.delta_seconds() * mv_amount as f32;
                transform.set_translation_y(
                    transform.translation().y + frontal_vector.y * scaled_amount,
                );
                transform.set_translation_x(
                    transform.translation().x - frontal_vector.x * scaled_amount,
                );
            }

            if let Some(mv_amount) = lateral_movement {
                let lateral_vector = transform.isometry().inverse().rotation * Vector3::x();
                let scaled_amount = VELOCITY * time.delta_seconds() * mv_amount as f32;
                transform.set_translation_y(
                    transform.translation().y - lateral_vector.y * scaled_amount,
                );
                transform.set_translation_x(
                    transform.translation().x + lateral_vector.x * scaled_amount,
                );
            }

            if let Some(mv_amount) = lateral_mouse_movement {
                let scaled_amount = VELOCITY * time.delta_seconds() * mv_amount as f32;
                transform.prepend_rotation_z_axis(-scaled_amount);
            }
        }
    }
}
