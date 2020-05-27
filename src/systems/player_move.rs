use amethyst::{
    core::timing::Time,
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::player::Player;

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

            if let Some(mv_amount) = frontal_movement {
                let scaled_amount = VELOCITY * time.delta_seconds() * mv_amount as f32;
                let player_y = transform.translation().y;
                transform.set_translation_y(player_y + scaled_amount);
            }

            if let Some(mv_amount) = lateral_movement {
                let scaled_amount = VELOCITY * time.delta_seconds() * mv_amount as f32;
                let player_x = transform.translation().x;
                transform.set_translation_x(player_x + scaled_amount);
            }
        }
    }
}
