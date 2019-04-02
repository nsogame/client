mod hitcircle;

use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::nalgebra::{Vector2, Vector3},
    ecs::prelude::World,
    renderer::{Material, MaterialDefaults, Mesh, PngFormat, PosTex, Texture, TextureMetadata},
};

use crate::components::{HitObject as HitObjectComponent, HitObjectKind};

use self::hitcircle::HitCircleResource;

fn create_png_vertices(left: f32, bottom: f32, right: f32, top: f32) -> Vec<PosTex> {
    vec![
        PosTex {
            position: Vector3::new(left, bottom, 0.0),
            tex_coord: Vector2::new(0.0, 0.0),
        },
        PosTex {
            position: Vector3::new(right, bottom, 0.0),
            tex_coord: Vector2::new(1.0, 0.0),
        },
        PosTex {
            position: Vector3::new(left, top, 0.0),
            tex_coord: Vector2::new(0.0, 1.0),
        },
        PosTex {
            position: Vector3::new(right, top, 0.0),
            tex_coord: Vector2::new(1.0, 1.0),
        },
        PosTex {
            position: Vector3::new(left, top, 0.),
            tex_coord: Vector2::new(0.0, 1.0),
        },
        PosTex {
            position: Vector3::new(right, bottom, 0.0),
            tex_coord: Vector2::new(1.0, 0.0),
        },
    ]
}

fn load_single_texture_png(
    name: &'static str,
    png_size: [f32; 2],
    world: &mut World,
) -> (Handle<Mesh>, Material) {
    let loader = world.read_resource::<Loader>();
    let albedo = loader.load(
        name,
        PngFormat,
        TextureMetadata::srgb_scale(),
        (),
        &world.read_resource::<AssetStorage<Texture>>(),
    );

    let mat_defaults = world.read_resource::<MaterialDefaults>();
    let material = Material {
        albedo,
        ..mat_defaults.0.clone()
    };

    let vertices = create_png_vertices(0.0, 0.0, png_size[0], png_size[1]);
    let mesh = loader.load_from_data(
        vertices.into(),
        (),
        &world.read_resource::<AssetStorage<Mesh>>(),
    );
    (mesh, material)
}

fn load_hitcircle_resource(world: &mut World) -> HitCircleResource {
    let (mesh, material) =
        load_single_texture_png("resources/skin/hitcircle.png", [128.0, 128.0], world);
    let resource = HitCircleResource {
        hitcircle_mesh: mesh,
        component: HitObjectComponent {
            time: 0.0,
            kind: HitObjectKind::HitCircle,
        }
    };
    world.add_resource(resource.clone());
    resource
}

pub fn load_skin(world: &mut World) {
}
