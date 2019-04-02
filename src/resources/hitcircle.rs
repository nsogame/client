use amethyst::{assets::Handle, renderer::Mesh};

use crate::components::HitObject as HitObjectComponent;

#[derive(Clone)]
pub struct HitCircleResource {
    pub hitcircle_mesh: Handle<Mesh>,
    pub component: HitObjectComponent,
}
