use amethyst::{
    core::timing::Time,
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::camera::Camera,
};

const VELOCITY: f32 = 0.5;

#[derive(SystemDesc)]
pub struct CameraMoveSystem;

impl<'s> System<'s> for CameraMoveSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, cameras, input, time): Self::SystemData) {
        for (_, transform) in (&cameras, &mut transforms).join() {
            let frontal_mouse_movement = input.axis_value("mouse_y");

            if let Some(mv_amount) = frontal_mouse_movement {
                let scaled_amount = VELOCITY * time.delta_seconds() * mv_amount as f32;
                transform.prepend_rotation_x_axis(scaled_amount);
            }
        }
    }
}
