use amethyst::ecs::prelude::{Component, DenseVecStorage};

pub struct HitObject;

impl Component for HitObject {
    type Storage = DenseVecStorage<Self>;
}
