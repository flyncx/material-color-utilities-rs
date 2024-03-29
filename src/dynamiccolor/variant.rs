use core::hash::Hash;
use std::hash::Hasher;

#[derive(Debug, Clone)]
pub enum Variant {
    /// All colors are grayscale, no chroma.
    Monochrome,
    /// Close to grayscale, a hint of chroma.
    Neutral,
    /// Pastel tokens, low chroma palettes (32).
    /// Default Material You theme at 2021 launch.
    TonalSpot,
    /// Pastel colors, high chroma palettes. (max).
    /// The primary palette's chroma is at maximum.
    /// Use Fidelity instead if tokens should alter their tone to match the palette vibrancy.
    Vibrant,
    /// Pastel colors, medium chroma palettes.
    /// The primary palette's hue is different from source color, for variety.
    Expressive,
    /// Almost identical to Fidelity.
    /// Tokens and palettes match source color.
    /// Primary Container is source color, adjusted to ensure contrast with surfaces.
    ///
    /// Tertiary palette is analogue of source color.
    /// Found by dividing color wheel by 6, then finding the 2 colors adjacent to source.
    /// The one that increases hue is used.
    Content,
    /// Tokens and palettes match source color.
    /// Primary Container is source color, adjusted to ensure contrast with surfaces.
    /// For example, if source color is black, it is lightened so it doesn't match surfaces in dark mode.
    ///
    /// Tertiary palette is complement of source color.
    Fidelity,
    /// A playful theme - the source color's hue does not appear in the theme.
    Rainbow,
    /// A playful theme - the source color's hue does not appear in the theme.
    FruitSalad,
}

impl Eq for Variant {}
impl PartialEq for Variant {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}
impl Hash for Variant {
    fn hash<H: Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}
