use amethyst::{core::timing::Time, ecs::prelude::*};

use crate::components::HitObject;

pub struct BeatmapSystem;

impl BeatmapSystem {
    pub fn new() -> Self {
        BeatmapSystem
    }
}

impl<'s> System<'s> for BeatmapSystem {
    type SystemData = (WriteStorage<'s, HitObject>, Read<'s, Time>);

    fn run(&mut self, (mut hitobjects, time): Self::SystemData) {
        for hitobject in &mut hitobjects.join() {}
    }
}
