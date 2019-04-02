extern crate amethyst;

mod audio;
mod game;

use amethyst::{
    prelude::*,
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage},
    ui::DrawUi,
    utils::application_root_dir,
};

use crate::game::OsuBundle;

struct Example;

impl SimpleState for Example {}

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
        .with_bundle(RenderBundle::new(pipe, Some(config)))?;
    let mut game = Application::new("./", Example, game_data)?;

    game.run();

    Ok(())
}
