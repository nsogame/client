use amethyst::ecs::prelude::*;

pub struct BeatmapSystem;

impl<'s> System<'s> for BeatmapSystem {
    type SystemData = ();

    fn run(&mut self, _: Self::SystemData) {

    }
}
