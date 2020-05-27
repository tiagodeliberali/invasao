use amethyst::{
    renderer::{
        camera::{Camera},
    },
    prelude::*,
    input::{InputHandler, ControllerButton, VirtualKeyCode, StringBindings},
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct CameraMoveSystem;

impl<'s> System<'s> for CameraMoveSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, cameras, input): Self::SystemData) {
        for (_, transform) in (&cameras, &mut transforms).join() {
            if let Some((x, y)) = input.mouse_position() {
                transform.prepend_translation_x(x);
                transform.prepend_translation_y(y);
            }
        }
    }
}