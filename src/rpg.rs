use amethyst::{
    assets::AssetLoaderSystemData,
    core::{transform::Transform, Parent, math::Vector3},
    ecs::prelude::Entity,
    prelude::*,
    renderer::{
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
use amethyst_physics::prelude::*;

use crate::player::initialise_player;

fn initialise_camera(world: &mut World, parent: Entity) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 0.0);

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
        let row = (i % 10) as f32 * 2.1_f32 - 10_f32;
        let column = (i / 10) as f32 * 2.1_f32 - 10_f32;

        transform.set_translation_xyz(row, 0.0, column);

        let shape_component = {
            let desc = ShapeDesc::Cube {
                half_extents: Vector3::new(1.0, 1.0, 1.0),
            };
            let physics_world = world.fetch::<PhysicsWorld<f32>>();
            physics_world.shape_server().create(&desc)
        };
    
        let rigid_body_component = {
            let mut rb_desc = RigidBodyDesc::default();
            rb_desc.mode = BodyMode::Static;
    
            let physics_world = world.fetch::<PhysicsWorld<f32>>();
            physics_world.rigid_body_server().create(&rb_desc)
        };

        world
            .create_entity()
            .with(mesh.clone())
            .with(material.clone())
            .with(transform)
            .with(rigid_body_component)
            .with(shape_component)
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
    transform.set_translation_xyz(5.0, 5.0, 20.0);

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
