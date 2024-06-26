use std::hash::Hash;

use crate::hct::cam16::Cam16;

use super::tonal_palette::TonalPalette;

/// An intermediate concept between the key color for a UI theme, and a full
/// color scheme. 5 tonal palettes are generated, all except one use the same
/// hue as the key color, and all vary in chroma.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct CorePalette {
    pub primary: TonalPalette,
    pub secondary: TonalPalette,
    pub tertiary: TonalPalette,
    pub neutral: TonalPalette,
    pub neutral_variant: TonalPalette,
    pub error: TonalPalette,
}
impl CorePalette {
    fn _default_error() -> TonalPalette {
        TonalPalette::of(25.0, 84.0)
    }

    /// The number of generated tonal palettes.
    pub const SIZE: usize = 5;

    /// Create a [CorePalette] from a source ARGB color.
    pub fn of(argb: i64) -> CorePalette {
        let cam = Cam16::from_int(argb);
        return CorePalette::__(cam.hue, cam.chroma);
    }

    fn __(hue: f64, chroma: f64) -> CorePalette {
        CorePalette {
            primary: TonalPalette::of(hue, (48.0f64).max(chroma)),
            secondary: TonalPalette::of(hue, 16.0),
            tertiary: TonalPalette::of(hue + 60.0, 24.0),
            neutral: TonalPalette::of(hue, 4.0),
            neutral_variant: TonalPalette::of(hue, 8.0),
            error: Self::_default_error(),
        }
    }

    /// Create a content [CorePalette] from a source ARGB color.
    pub fn content_of(argb: i64) -> CorePalette {
        let cam = Cam16::from_int(argb);
        return CorePalette::_content_of(cam.hue, cam.chroma);
    }

    fn _content_of(hue: f64, chroma: f64) -> CorePalette {
        CorePalette {
            primary: TonalPalette::of(hue, chroma),
            secondary: TonalPalette::of(hue, chroma / 3.0),
            tertiary: TonalPalette::of(hue + 60.0, chroma / 2.0),
            neutral: TonalPalette::of(hue, (chroma / 12.0).min(4.0)),
            neutral_variant: TonalPalette::of(hue, (chroma / 6.0).min(8.0)),
            error: Self::_default_error(),
        }
    }

    /// Create a [CorePalette] from a fixed-size list of ARGB color ints
    /// representing concatenated tonal palettes.
    ///
    /// Inverse of [asList].
    pub fn from_list(colors: &Vec<i64>) -> CorePalette {
        assert!(colors.len() == Self::SIZE * TonalPalette::COMMON_SIZE);
        CorePalette {
            primary: TonalPalette::from_list(&_get_partition(colors, 0, TonalPalette::COMMON_SIZE)),
            secondary: TonalPalette::from_list(&_get_partition(
                colors,
                1,
                TonalPalette::COMMON_SIZE,
            )),
            tertiary: TonalPalette::from_list(&_get_partition(
                colors,
                2,
                TonalPalette::COMMON_SIZE,
            )),
            neutral: TonalPalette::from_list(&_get_partition(colors, 3, TonalPalette::COMMON_SIZE)),
            neutral_variant: TonalPalette::from_list(&_get_partition(
                colors,
                4,
                TonalPalette::COMMON_SIZE,
            )),
            error: Self::_default_error(),
        }
    }

    /// Returns a list of ARGB color [int]s from concatenated tonal palettes.
    ///
    /// Inverse of [CorePalette.fromList].
    pub fn as_list(&mut self) -> Vec<i64> {
        let mut list: Vec<i64> = Vec::new();

        list.extend(self.primary.get_as_list().iter());
        list.extend(self.secondary.get_as_list().iter());
        list.extend(self.tertiary.get_as_list().iter());
        list.extend(self.neutral.get_as_list().iter());
        list.extend(self.neutral_variant.get_as_list().iter());

        return list;
    }
}

// Returns a partition from a list.
//
// For example, given a list with 2 partitions of size 3.
// range = [1, 2, 3, 4, 5, 6];
//
// range.getPartition(0, 3) // [1, 2, 3]
// range.getPartition(1, 3) // [4, 5, 6]
fn _get_partition(list: &Vec<i64>, partition_number: usize, partition_size: usize) -> Vec<i64> {
    /* return list.sublist(
        partitionNumber * partitionSize,
        (partitionNumber + 1) * partitionSize,
    ); */
    let sublist =
        &list[(partition_number * partition_size)..((partition_number + 1) * partition_size)];
    return sublist.to_vec();
}

impl ToString for CorePalette {
    fn to_string(&self) -> String {
        return format!(
            "primary: {}\n
            secondary: {}\n
            tertiary: {}\n
            neutral: {}\n
            neutralVariant: {}\n
            error: {}\n",
            self.primary.to_string(),
            self.secondary.to_string(),
            self.tertiary.to_string(),
            self.neutral.to_string(),
            self.neutral_variant.to_string(),
            self.error.to_string()
        );
    }
}
