use amethyst::{assets::Handle, renderer::Mesh};

#[derive(Clone)]
pub struct HitCircleResource {
    pub hitcircle_mesh: Handle<Mesh>,
}
