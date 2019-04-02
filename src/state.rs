use amethyst::{
    assets::{AssetStorage, Loader},
    prelude::*,
};

use crate::resources::load_skin;

pub struct OsuState;

impl SimpleState for OsuState {
    fn on_start(&mut self, state_data: StateData<'_, GameData<'_, '_>>) {
        let world = state_data.world;
        load_skin(world);
    }
}
