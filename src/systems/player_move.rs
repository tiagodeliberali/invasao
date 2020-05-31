use crate::entity3d::Entity3d;
use crate::player::Player;
use amethyst::{
    core::timing::Time,
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
    shrev::{EventChannel, ReaderId},
    winit::{DeviceEvent, Event},
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
        for (_, mut transform) in (&players, &mut transforms).join() {
            let mut entity = Entity3d::new(&mut transform);

            if let Some(mv_amount) = input.axis_value("frontal") {
                entity.walk_forward(KEYBOARD_VELOCITY * time.delta_seconds() * mv_amount as f32)
            }

            if let Some(mv_amount) = input.axis_value("lateral") {
                entity.walk_right(KEYBOARD_VELOCITY * time.delta_seconds() * mv_amount as f32)
            }

            for event in events.read(&mut self.event_reader) {
                if let Event::DeviceEvent { ref event, .. } = *event {
                    if let DeviceEvent::MouseMotion { delta: (x, _y) } = *event {
                        entity.rotate_horizontal(MOUSE_VELOCITY * time.delta_seconds() * x as f32);
                    }
                }
            }
        }
    }
}
