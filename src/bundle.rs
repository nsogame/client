use amethyst::{
    core::bundle::{self, SystemBundle},
    ecs::prelude::DispatcherBuilder,
};

pub struct OsuBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for OsuBundle {
    fn build(self, builder: &mut DispatcherBuilder<'a, 'b>) -> bundle::Result<()> {
        Ok(())
    }
}
