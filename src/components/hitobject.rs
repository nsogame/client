use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Clone)]
pub struct HitObject {
    pub time: f64,
    pub kind: HitObjectKind,
}

#[derive(Clone)]
pub enum HitObjectKind {
    HitCircle,
}

impl Component for HitObject {
    type Storage = DenseVecStorage<Self>;
}
