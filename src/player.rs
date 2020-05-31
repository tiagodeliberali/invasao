use amethyst::{
    assets::AssetLoaderSystemData,
    core::transform::{Parent, Transform},
    ecs::prelude::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::{
        camera::Camera,
        mtl::{Material, MaterialDefaults},
        rendy::mesh::{Normal, Position, Tangent, TexCoord},
        shape::Shape,
        visibility::BoundingSphere,
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

fn initialise_camera(world: &mut World, parent: Entity) {
    let mut transform = Transform::default();
    transform.set_translation_y(4.0);
    // transform.set_rotation_z_axis(1.0);

    world
        .create_entity()
        .with(Camera::standard_3d(1024.0, 768.0))
        .with(Parent { entity: parent })
        .with(transform)
        .build();
}

pub fn initialise_player(world: &mut World) {
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

    let player = world
        .create_entity()
        .with(mesh)
        .with(material)
        .with(Player::new())
        .with(player_transform)
        .with(BoundingSphere::origin(1.74))
        .build();

    initialise_camera(world, player);
}
