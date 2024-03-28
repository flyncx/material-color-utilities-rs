use std::collections::HashMap;

use crate::{hct::hct::Hct, utils::math_utils::MathUtils};

struct _ScoredHCT {
    hct: Hct,
    score: f64,
}
impl _ScoredHCT {
    pub fn _compare_to(&self, other: &_ScoredHCT) -> i64 {
        if self.score > other.score {
            return -1;
        } else if self.score == other.score {
            return 0;
        } else {
            return 1;
        }
    }
}

impl Eq for _ScoredHCT {}
impl PartialEq for _ScoredHCT {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}
impl PartialOrd for _ScoredHCT {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let res = self._compare_to(other);
        match res {
            1 => Some(std::cmp::Ordering::Greater),
            0 => Some(std::cmp::Ordering::Equal),
            -1 => Some(std::cmp::Ordering::Less),
            _ => None,
        }
    }
}

impl Ord for _ScoredHCT {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score
            .partial_cmp(&other.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    }
}

/// Given a large set of colors, remove colors that are unsuitable for a UI
/// theme, and rank the rest based on suitability.
///
/// Enables use of a high cluster count for image quantization, thus ensuring
/// colors aren't muddied, while curating the high cluster count to a much
///  smaller number of appropriate choices.
pub struct Score {}
impl Score {
    const _TARGET_CHROMA: f64 = 48.0; // A1 Chroma
    const _WEIGHT_PROPORTION: f64 = 0.7;
    const _WEIGHT_CHROMA_ABOVE: f64 = 0.3;
    const _WEIGHT_CHROMA_BELOW: f64 = 0.1;
    const _CUTOFF_CHROMA: f64 = 5.0;
    const _CUTOFF_EXCITED_PROPORTION: f64 = 0.01;

    /// Given a map with keys of colors and values of how often the color appears,
    /// rank the colors based on suitability for being used for a UI theme.
    ///
    /// [colorsToPopulation] is a map with keys of colors and values of often the
    /// color appears, usually from a source image.
    /// [desired] max count of colors to be returned in the list.
    /// [fallbackColorARGB] color to be returned if no other options available.
    /// [filter] whether to filter out undesireable combinations.
    ///
    /// The list returned is of length <= [desired]. The recommended color is the
    /// first item, the least suitable is the last. There will always be at least
    /// one color returned. If all the input colors were not suitable for a theme,
    /// a default fallback color will be provided, Google Blue. The default
    /// number of colors returned is 4, simply because thats the # of colors
    /// display in Android 12's wallpaper picker.
    pub fn score(
        colors_to_population: HashMap<i64, i64>,
        desired: Option<i64>,
        fallback_color_argb: Option<i64>,
        filter: Option<bool>,
    ) -> Vec<i64> {
        let desired = desired.unwrap_or(4);
        let fallback_color_argb = fallback_color_argb.unwrap_or(0xff4285f4);
        let filter = filter.unwrap_or(true);

        // Get the HCT color for each Argb value, while finding the per hue count and
        // total count.
        let mut colors_hct: Vec<Hct> = Vec::new();
        let mut hue_population = [0; 360]; //List<int>.filled(360, 0);
        let mut population_sum = 0;
        for (key, value) in colors_to_population {
            let argb = key;
            let population = value;
            let hct = Hct::from_int(argb);
            colors_hct.push(hct.clone());
            let hue = hct.get_hue().floor();
            hue_population[hue as usize] += population;
            population_sum += population;
        }

        // Hues with more usage in neighboring 30 degree slice get a larger number.
        let mut hue_excited_proportions = [0.0; 360];
        let mut hue: i64 = 0;
        while hue < 360 {
            let proportion = (hue_population[hue as usize] as f64) / (population_sum as f64);
            let mut i: i64 = hue - 14;
            while i < hue + 16 {
                let neighbor_hue = MathUtils::sanitize_degrees_int(i);
                hue_excited_proportions[neighbor_hue as usize] += proportion as f64;
                i += 1;
            }
            hue += 1;
        }

        // Scores each HCT color based on usage and chroma, while optionally
        // filtering out values that do not have enough chroma or usage.
        let mut scored_hcts: Vec<_ScoredHCT> = Vec::new();
        for hct in colors_hct {
            let hue = MathUtils::sanitize_degrees_int(hct.get_hue().round() as i64);
            let proportion = hue_excited_proportions[hue as usize];
            if filter
                && (hct.get_chroma() < Self::_CUTOFF_CHROMA
                    || proportion <= Self::_CUTOFF_EXCITED_PROPORTION)
            {
                continue;
            }

            let proportion_score = proportion * 100.0 * Self::_WEIGHT_PROPORTION;
            let chroma_weight = {
                if hct.get_chroma() < Self::_TARGET_CHROMA {
                    Self::_WEIGHT_CHROMA_BELOW
                } else {
                    Self::_WEIGHT_CHROMA_ABOVE
                }
            };

            let chroma_score = (hct.get_chroma() - Self::_TARGET_CHROMA) * chroma_weight;
            let score = proportion_score + chroma_score;
            scored_hcts.push(_ScoredHCT { hct, score })
        }
        // Sorted so that colors with higher scores come first.
        scored_hcts.sort();

        // Iterates through potential hue differences in degrees in order to select
        // the colors with the largest distribution of hues possible. Starting at
        // 90 degrees(maximum difference for 4 colors) then decreasing down to a
        // 15 degree minimum.
        let mut chosen_colors: Vec<&Hct> = Vec::new();
        let mut difference_degrees: i64 = 90;
        while difference_degrees >= 15 {
            chosen_colors.clear();
            for entry in &scored_hcts {
                let hct = &entry.hct;
                let duplicate_hue = chosen_colors.iter().find(|chosen_hct| {
                    MathUtils::difference_degrees(hct.get_hue(), chosen_hct.get_hue())
                        < difference_degrees as f64
                });

                if duplicate_hue.is_none() {
                    chosen_colors.push(hct);
                }
                if chosen_colors.len() as i64 >= desired {
                    break;
                }
            }
            if chosen_colors.len() as i64 >= desired {
                break;
            }
            difference_degrees -= 1;
        }
        let mut colors: Vec<i64> = Vec::new();
        if chosen_colors.is_empty() {
            colors.push(fallback_color_argb);
        }
        for chosen_hct in chosen_colors {
            colors.push(chosen_hct.to_int());
        }
        return colors;
    }
}
