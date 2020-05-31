use amethyst::{
    controls::{CursorHideSystemDesc, MouseFocusUpdateSystemDesc},
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderPbr3D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};
use amethyst_nphysics::NPhysicsBackend;
use amethyst_physics::PhysicsBundle;

mod entity3d;
mod player;
mod rpg;
mod systems;

use rpg::Rpg;
use systems::{CameraMoveSystemDesc, PlayerMoveSystemDesc};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");
    let binding_path = app_root.join("config").join("bindings.ron");

    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderPbr3D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(PhysicsBundle::<f32, NPhysicsBackend>::new())?
        .with_system_desc(
            PlayerMoveSystemDesc::default(),
            "player_move_system",
            &["input_system"],
        )
        .with_system_desc(
            CameraMoveSystemDesc::default(),
            "camera_move_system",
            &["input_system"],
        )
        .with_system_desc(MouseFocusUpdateSystemDesc::default(), "mouse_focus", &[])
        .with_system_desc(CursorHideSystemDesc::default(), "cursor_hide", &[]);

    let mut game = Application::new(assets_dir, Rpg::default(), game_data)?;
    game.run();

    Ok(())
}
