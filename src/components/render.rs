use sdl2::pixels::Color;

pub struct RenderComponent {
    pub color: Color,
}

impl RenderComponent {
    pub fn new(color: Color) -> Self {
        RenderComponent { color }
    }
}
