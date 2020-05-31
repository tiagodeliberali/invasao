use amethyst::{
    assets::AssetLoaderSystemData,
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::{
        visibility::BoundingSphere,
        mtl::{Material, MaterialDefaults},
        rendy::mesh::{Normal, Position, Tangent, TexCoord},
        shape::Shape,
        Mesh,
    },
};

pub struct Player {}

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

    let mut player_transform = Transform::default();
    player_transform.set_translation_y(2.0);

    world
        .create_entity()
        .with(mesh)
        .with(material)
        .with(Player::new())
        .with(player_transform)
        .with(BoundingSphere::origin(1.74))
        .build()
}
