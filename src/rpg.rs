use amethyst::core::timing::Time;
use amethyst::{
    assets::{AssetStorage, Handle, Loader, AssetLoaderSystemData},
    core::{
        transform::Transform,
        Parent,
    },
    ecs::prelude::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::{
        camera::{Camera},
        rendy::mesh::{Normal, Tangent, Position, TexCoord},
        formats::texture::ImageFormat,
        sprite::{SpriteRender, SpriteSheet, SpriteSheetFormat},
        Texture,
        Mesh,
        shape::Shape,
        mtl::{Material, MaterialDefaults},
        light::{Light, PointLight},
        palette::rgb::Rgb,
    },
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};

use crate::player::initialise_player;

fn initialise_camera(world: &mut World, parent: Entity) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 10.0);

    world.create_entity()
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
        },
    );

    let mut transform1 = Transform::default();
    transform1.set_translation_xyz(0.0, 0.0, 0.0);

    let mut transform2 = Transform::default();
    transform2.set_translation_xyz(4.0, 0.0, 0.0);

    let mut transform3 = Transform::default();
    transform3.set_translation_xyz(-4.0, 0.0, 0.0);

    world.create_entity()
        .with(mesh.clone())
        .with(material.clone())
        .with(transform1)
        .build();

    world.create_entity()
        .with(mesh.clone())
        .with(material.clone())
        .with(transform2)
        .build();

    world.create_entity()
        .with(mesh)
        .with(material)
        .with(transform3)
        .build();
}

fn initialize_light(world: &mut World) {
    let light: Light = PointLight {
        intensity: 10.0,
        color: Rgb::new(1.0, 1.0, 1.0),
        ..PointLight::default()
    }.into();

    let mut transform = Transform::default();
    transform.set_translation_xyz(5.0, 5.0, 20.0);

    world
        .create_entity()
        .with(light)
        .with(transform)
        .build();
}

#[derive(Default)]
pub struct Rpg {
}

impl SimpleState for Rpg {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        initialise_floor(world);
        initialize_light(world);
        let player = initialise_player(world);
        initialise_camera(world, player);
    }
}
