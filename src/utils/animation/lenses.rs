use bevy::prelude::*;
use bevy_tweening::*;

trait ColorLerper {
    fn lerp(&self, target: &Self, ratio: f32) -> Self;
}

impl ColorLerper for Color {
    fn lerp(&self, target: &Color, ratio: f32) -> Color {
        let r = self.r().lerp(target.r(), ratio);
        let g = self.g().lerp(target.g(), ratio);
        let b = self.b().lerp(target.b(), ratio);
        let a = self.a().lerp(target.a(), ratio);
        Color::rgba(r, g, b, a)
    }
}

/// A variation of bevy_tweening::ColorMaterialColorLens,
/// that lets the user still modify the alpha externally.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ColorMaterialRGBLens {
    /// Start color. Alpha is ignored.
    pub start: Color,
    /// End color. Alpha is ignored.
    pub end: Color,
}

impl Lens<ColorMaterial> for ColorMaterialRGBLens {
    fn lerp(&mut self, target: &mut ColorMaterial, ratio: f32) {
        let value = self.start.lerp(&self.end, ratio);
        target.color = value.with_a(target.color.a());
    }
}
