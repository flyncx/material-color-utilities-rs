pub mod blend;
pub mod contrast;
pub mod dislike;
pub mod dynamiccolor;
pub mod hct;
pub mod palettes;
pub mod quantize;
pub mod scheme;
pub mod score;
pub mod temperature;
mod tests;
pub mod utils;

#[cfg(test)]
pub mod sanity {
    use image::GenericImageView;

    use crate::{
        dynamiccolor::{
            dynamic_scheme::DynamicScheme, material_dynamic_colors::MaterialDynamicColors,
            variant::Variant,
        },
        palettes::core_palette::CorePalette,
        quantize::{quantizer::Quantizer, quantizer_celebi::QuantizerCelebi},
        score::score::Score,
        utils::{color_utils::ColorUtils, string_utils::StringUtils},
    };

    #[test]
    #[ignore]
    fn surface() {
        let img = image::open("D:\\sample3.jpg").unwrap();
        let mut pixels: Vec<i64> = Vec::new();
        for (_x, _y, data) in img.pixels() {
            pixels.push(ColorUtils::argb_from_rgb(
                data[0] as i64,
                data[1] as i64,
                data[2] as i64,
            ))
        }
        let qr = QuantizerCelebi {}.quantize(&pixels, 128, None);
        let score = Score::score(&qr.color_to_count, None, None, None);
        let dominant = *score.first().unwrap();
        let core = CorePalette::of(dominant);
        let scheme = DynamicScheme::new(
            dominant,
            Variant::TonalSpot,
            true,
            None,
            core.primary,
            core.secondary,
            core.tertiary,
            core.neutral,
            core.neutral_variant,
        );
        //let scheme = DynamicScheme
        println!(
            "{}",
            StringUtils::hex_from_argb(MaterialDynamicColors::primary().get_argb(&scheme), None)
        )
    }
}
