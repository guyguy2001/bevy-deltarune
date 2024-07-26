use bevy::prelude::*;
use bevy_tweening::*;

trait ColorLerper {
    fn lerp(&self, target: &Self, ratio: f32) -> Self;
}

impl ColorLerper for Color {
    fn lerp(&self, target: &Color, ratio: f32) -> Color {
        let self_srgba = self.to_srgba();
        let target_srgba = target.to_srgba();
        let r = self_srgba.red.lerp(target_srgba.red, ratio);
        let g = self_srgba.green.lerp(target_srgba.green, ratio);
        let b = self_srgba.blue.lerp(target_srgba.blue, ratio);
        let a = self_srgba.alpha.lerp(target_srgba.alpha, ratio);
        Color::srgba(r, g, b, a)
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
    fn lerp(&mut self, target: &mut dyn Targetable<ColorMaterial>, ratio: f32) {
        let value = self.start.lerp(&self.end, ratio);
        target.color = value.with_alpha(target.color.alpha());
    }
}
