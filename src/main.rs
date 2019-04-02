#[macro_use]
extern crate serde_derive;

mod audio;
mod beatmap;
mod bundle;
mod components;
mod resources;
mod state;
mod systems;

use amethyst::{
    prelude::*,
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage},
    ui::{DrawUi, UiBundle},
    core::transform::TransformBundle,
    utils::application_root_dir,
};

use crate::beatmap::Beatmap;
use crate::bundle::OsuBundle;
use crate::state::OsuState;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let path = format!("{}/resources/display_config.ron", application_root_dir());
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.02196, 0.23726, 0.21765, 1.0], 1.0)
            .with_pass(DrawFlat2D::new())
            .with_pass(DrawUi::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(OsuBundle)?
        .with_bundle(TransformBundle::new())?
        .with_bundle(RenderBundle::new(pipe, Some(config)))?
        .with_bundle(UiBundle::<String, String>::new())?;
    let mut game = Application::new("./", OsuState, game_data)?;

    game.run();

    Ok(())
}
