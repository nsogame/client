mod hitcircle;

use amethyst::{
    assets::{AssetStorage, Loader},
    ecs::prelude::World,
    renderer::{PngFormat, Texture, TextureMetadata},
};

fn load_single_texture_png(name: &'static str, world: &mut World) {
    let loader = world.read_resource::<Loader>();
    loader.load(
        name,
        PngFormat,
        TextureMetadata::srgb_scale(),
        (),
        &world.read_resource::<AssetStorage<Texture>>(),
    );
}

pub fn load_skin(world: &mut World) {
    load_single_texture_png("resources/skin/hitcircle.png", world);
}
