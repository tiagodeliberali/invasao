use amethyst::core::timing::Time;
use amethyst::{
    assets::{AssetStorage, Handle, Loader, AssetLoaderSystemData},
    core::transform::Transform,
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


pub struct Player {
}

impl Player {
    fn new() -> Player {
        Player {}
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialise_player(world: &mut World) -> Entity {
    let mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
        loader.load_from_data(
            Shape::Sphere(100, 100)
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

    let player_transform = Transform::default();

    world
        .create_entity()
        .with(mesh)
        .with(material)
        .with(Player::new())
        .with(player_transform)
        .build()
}