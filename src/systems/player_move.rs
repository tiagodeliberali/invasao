use crate::player::Player;
use amethyst::{
    shrev::{EventChannel, ReaderId},
    winit::{DeviceEvent, Event},
    core::math::Vector3,
    core::timing::Time,
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

const MOUSE_VELOCITY: f32 = 0.05;
const KEYBOARD_VELOCITY: f32 = 5.0;

#[derive(SystemDesc)]
#[system_desc(name(PlayerMoveSystemDesc))]
pub struct PlayerMoveSystem {
    #[system_desc(event_channel_reader)]
    event_reader: ReaderId<Event>,
}

impl PlayerMoveSystem {
    pub fn new(event_reader: ReaderId<Event>) -> Self {
        PlayerMoveSystem { event_reader }
    }
}

impl<'s> System<'s> for PlayerMoveSystem {
    type SystemData = (
        Read<'s, EventChannel<Event>>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (events, mut transforms, players, input, time): Self::SystemData) {
        for (_, transform) in (&players, &mut transforms).join() {
            let frontal_movement = input.axis_value("frontal");
            let lateral_movement = input.axis_value("lateral");

            if let Some(mv_amount) = frontal_movement {
                let frontal_vector = transform.isometry().inverse().rotation * Vector3::y();
                let scaled_amount = KEYBOARD_VELOCITY * time.delta_seconds() * mv_amount as f32;
                transform.set_translation_y(
                    transform.translation().y + frontal_vector.y * scaled_amount,
                );
                transform.set_translation_x(
                    transform.translation().x - frontal_vector.x * scaled_amount,
                );
            }

            if let Some(mv_amount) = lateral_movement {
                let lateral_vector = transform.isometry().inverse().rotation * Vector3::x();
                let scaled_amount = KEYBOARD_VELOCITY * time.delta_seconds() * mv_amount as f32;
                transform.set_translation_y(
                    transform.translation().y - lateral_vector.y * scaled_amount,
                );
                transform.set_translation_x(
                    transform.translation().x + lateral_vector.x * scaled_amount,
                );
            }

            for event in events.read(&mut self.event_reader) {
                if let Event::DeviceEvent { ref event, .. } = *event {
                    if let DeviceEvent::MouseMotion { delta: (x, _y) } = *event {
                        let scaled_amount = MOUSE_VELOCITY * time.delta_seconds() * x as f32;
                        transform.prepend_rotation_z_axis(scaled_amount);
                    }
                }
            }
        }
    }
}
