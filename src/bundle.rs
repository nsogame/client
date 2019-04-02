use amethyst::{
    core::bundle::{self, SystemBundle},
    ecs::prelude::DispatcherBuilder,
};

use crate::systems::*;

pub struct OsuBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for OsuBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> bundle::Result<()> {
        builder.add(BeatmapSystem::new(), "beatmap_system", &[]);
        Ok(())
    }
}
