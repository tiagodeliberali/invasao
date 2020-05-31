use amethyst::{
    assets::AssetLoaderSystemData,
    core::{transform::Transform, math::Vector3},
    ecs::prelude::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::{
        mtl::{Material, MaterialDefaults},
        rendy::mesh::{Normal, Position, Tangent, TexCoord},
        shape::Shape,
        Mesh,
    },
};
use amethyst_physics::prelude::*;

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
    player_transform.set_translation_xyz(0.0, 8.0, 0.0);

    let shape = {
        let desc = ShapeDesc::Cube {
            half_extents: Vector3::new(2.5, 2.5, 2.5),
        };
        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.shape_server().create(&desc)
    };

    let rb = {
        let mut rb_desc = RigidBodyDesc::default();
        rb_desc.lock_rotation_x = true;
        rb_desc.lock_rotation_y = false;
        rb_desc.lock_rotation_z = true;
        rb_desc.contacts_to_report = 3;
        rb_desc.friction = 0.0;
        rb_desc.bounciness = 0.0;

        let physics_world = world.fetch::<PhysicsWorld<f32>>();
        physics_world.rigid_body_server().create(&rb_desc)
    };

    world
        .create_entity()
        .with(mesh)
        .with(material)
        .with(Player::new())
        .with(player_transform)
        .with(shape)
        .with(rb)
        .build()
}
