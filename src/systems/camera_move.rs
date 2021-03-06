use crate::entity3d::Entity3d;
use amethyst::{
    core::timing::Time,
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    renderer::camera::Camera,
    shrev::{EventChannel, ReaderId},
    winit::{DeviceEvent, Event},
};

const VELOCITY: f32 = 0.05;

#[derive(SystemDesc)]
#[system_desc(name(CameraMoveSystemDesc))]
pub struct CameraMoveSystem {
    #[system_desc(event_channel_reader)]
    event_reader: ReaderId<Event>,
}

impl CameraMoveSystem {
    pub fn new(event_reader: ReaderId<Event>) -> Self {
        CameraMoveSystem { event_reader }
    }
}

impl<'s> System<'s> for CameraMoveSystem {
    type SystemData = (
        Read<'s, EventChannel<Event>>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        Read<'s, Time>,
    );

    fn run(&mut self, (events, mut transforms, cameras, time): Self::SystemData) {
        for (_, mut transform) in (&cameras, &mut transforms).join() {
            let mut entity = Entity3d::new(&mut transform);

            for event in events.read(&mut self.event_reader) {
                if let Event::DeviceEvent { ref event, .. } = *event {
                    if let DeviceEvent::MouseMotion { delta: (_x, y) } = *event {
                        entity.rotate_vertical(VELOCITY * time.delta_seconds() * y as f32);
                    }
                }
            }
        }
    }
}
