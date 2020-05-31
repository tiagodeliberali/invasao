use amethyst::{
    assets::AssetLoaderSystemData,
    core::{transform::Transform, Parent},
    ecs::prelude::Entity,
    prelude::*,
    renderer::{
        visibility::BoundingSphere,
        camera::Camera,
        light::{Light, PointLight},
        mtl::{Material, MaterialDefaults},
        palette::rgb::Rgb,
        rendy::mesh::{Normal, Position, Tangent, TexCoord},
        shape::Shape,
        Mesh,
    },
    window::{MonitorIdent, Window},
};

use crate::player::initialise_player;

fn initialise_camera(world: &mut World, parent: Entity) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 3.0, 0.0);
    transform.set_rotation_x_axis(1.0);

    world
        .create_entity()
        .with(Camera::standard_3d(1024.0, 768.0))
        .with(Parent { entity: parent })
        .with(transform)
        .build();
}

fn initialise_floor(world: &mut World) {
    let mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
        loader.load_from_data(
            Shape::Cube
                .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                .into(),
            (),
        )
    });

    let material_defaults = world.read_resource::<MaterialDefaults>().0.clone();
    let material = world.exec(|loader: AssetLoaderSystemData<'_, Material>| {
        loader.load_from_data(
            Material {
                ..material_defaults
            },
            (),
        )
    });

    for i in 0..100 {
        let mut transform = Transform::default();
        let row = (i % 10) as f32 * 2.0_f32 - 10_f32;
        let column = (i / 10) as f32 * 2.0_f32 - 10_f32;

        transform.set_translation_xyz(row, 0.0, column);

        world
            .create_entity()
            .with(mesh.clone())
            .with(material.clone())
            .with(transform)
            .with(BoundingSphere::origin(1.74))
            .build();
    }
}

fn initialize_light(world: &mut World) {
    let light: Light = PointLight {
        intensity: 10.0,
        color: Rgb::new(1.0, 1.0, 1.0),
        ..PointLight::default()
    }
    .into();

    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 10.0, 0.0);

    world.create_entity().with(light).with(transform).build();
}

#[derive(Default)]
pub struct Rpg {}

impl SimpleState for Rpg {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        initialise_floor(world);
        initialize_light(world);
        let player = initialise_player(world);
        initialise_camera(world, player);
        enter_fullscreen(world);
    }
}

fn enter_fullscreen(world: &mut World) {
    let window = world.read_resource::<Window>();
    let monitor_ident = MonitorIdent::from_primary(&*window);
    let monitor_id = monitor_ident.monitor_id(&*window);

    window.set_fullscreen(Some(monitor_id));
}
