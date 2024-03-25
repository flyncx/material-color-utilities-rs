use crate::hct::hct::Hct;
use core::hash::Hash;
use std::{collections::HashMap, hash::Hasher};

/// A convenience class for retrieving colors that are constant in hue and
/// chroma, but vary in tone.
///
/// This class can be instantiated in two ways:
/// 1. [of] From hue and chroma. (preferred)
/// 2. [fromList] From a fixed-size ([TonalPalette.commonSize]) list of ints
/// representing ARBG colors. Correctness (constant hue and chroma) of the input
/// is not enforced. [get] will only return the input colors, corresponding to
/// [commonTones]. This also initializes the key color to black.
#[derive(Clone)]
pub struct TonalPalette {
    pub hue: f64,
    pub chroma: f64,
    pub key_color: Hct,
    /// A cache containing keys-value pairs where:
    /// - keys are integers that represent tones, and
    /// - values are colors in ARGB format.
    pub _cache: HashMap<i64, i64>,
    pub _is_from_cache: bool,
}

impl TonalPalette {
    /// Commonly-used tone values.
    const COMMON_TONES: [i64; 13] = [0, 10, 20, 30, 40, 50, 60, 70, 80, 90, 95, 99, 100];

    pub const COMMON_SIZE: usize = Self::COMMON_TONES.len();

    fn _from_hct(hct: Hct) -> TonalPalette {
        TonalPalette {
            _cache: HashMap::new(),
            hue: hct.get_hue(),
            chroma: hct.get_chroma(),
            key_color: hct,
            _is_from_cache: false,
        }
    }

    fn _from_hue_and_chroma(hue: f64, chroma: f64) -> TonalPalette {
        TonalPalette {
            hue,
            chroma,
            _cache: HashMap::new(),
            key_color: Self::create_key_color(hue, chroma),
            _is_from_cache: false,
        }
    }

    fn _from_cache(cache: HashMap<i64, i64>, hue: f64, chroma: f64) -> TonalPalette {
        TonalPalette {
            hue,
            chroma,
            _cache: cache,
            key_color: Self::create_key_color(hue, chroma),
            _is_from_cache: true,
        }
    }

    /// Create colors using [hue] and [chroma].
    pub fn of(hue: f64, chroma: f64) -> TonalPalette {
        return TonalPalette::_from_hue_and_chroma(hue, chroma);
    }

    /// Create a Tonal Palette from hue and chroma of [hct].
    pub fn from_hct(hct: Hct) -> TonalPalette {
        return TonalPalette::_from_hct(hct);
    }

    /// Create colors from a fixed-size list of ARGB color ints.
    ///
    /// Inverse of [TonalPalette.asList].
    pub fn from_list(colors: Vec<i64>) -> TonalPalette {
        assert!(colors.len() == Self::COMMON_SIZE);
        let mut cache: HashMap<i64, i64> = HashMap::new();

        for (index, tone_value) in Self::COMMON_TONES.iter().enumerate() {
            cache.insert(*tone_value, colors[index]);
        }
        // Approximately deduces the original hue and chroma that generated this
        // list of colors.
        // Uses the hue and chroma of the provided color with the highest chroma.
        let mut best_hue = 0.0;
        let mut best_chroma = 0.0;
        for argb in colors {
            let hct = Hct::from_int(argb);

            // If the color is too close to white, its chroma may have been
            // affected by a known issue, so we ignore it.
            // https://github.com/material-foundation/material-color-utilities/issues/140

            if hct.get_tone() > 98.0 {
                continue;
            };

            if hct.get_chroma() > best_chroma {
                best_hue = hct.get_hue();
                best_chroma = hct.get_chroma();
            }
        }

        return TonalPalette::_from_cache(cache, best_hue, best_chroma);
    }

    /// Creates a key color from a [hue] and a [chroma].
    /// The key color is the first tone, starting from T50, matching the given hue and chroma.
    /// Key color [Hct]
    pub fn create_key_color(hue: f64, chroma: f64) -> Hct {
        let start_tone: f64 = 50.0;
        let mut smallest_delta_hct: Hct = Hct::from(hue, chroma, start_tone);
        let mut smallest_delta: f64 = (smallest_delta_hct.get_chroma() - chroma).abs();
        // Starting from T50, check T+/-delta to see if they match the requested
        // chroma.
        //
        // Starts from T50 because T50 has the most chroma available, on
        // average. Thus it is most likely to have a direct answer and minimize
        // iteration.
        for delta in 1..50 {
            let delta = delta as f64;
            // Termination condition rounding instead of minimizing delta to avoid
            // case where requested chroma is 16.51, and the closest chroma is 16.49.
            // Error is minimized, but when rounded and displayed, requested chroma
            // is 17, key color's chroma is 16.
            if chroma.round() == smallest_delta_hct.get_chroma().round() {
                return smallest_delta_hct;
            }

            let hct_add: Hct = Hct::from(hue, chroma, start_tone + delta);
            let hct_add_delta: f64 = (hct_add.get_chroma() - chroma).abs();
            if hct_add_delta < smallest_delta {
                smallest_delta = hct_add_delta;
                smallest_delta_hct = hct_add;
            }

            let hct_subtract: Hct = Hct::from(hue, chroma, start_tone - delta);
            let hct_subtract_delta: f64 = (hct_subtract.get_chroma() - chroma).abs();
            if hct_subtract_delta < smallest_delta {
                smallest_delta = hct_subtract_delta;
                smallest_delta_hct = hct_subtract;
            }
        }

        return smallest_delta_hct;
    }

    /// Returns a fixed-size list of ARGB color ints for common tone values.
    ///
    /// Inverse of [fromList].
    pub fn get_as_list(&mut self) -> Vec<i64> {
        Self::COMMON_TONES
            .iter()
            .map(|tone| self.get(*tone))
            .collect()
    }

    /// Returns the ARGB representation of an HCT color at the given [tone].
    ///
    /// If the palette is constructed from a list of colors
    /// (i.e. using [fromList]), the color provided at construction is returned
    /// if possible; otherwise the result is generated from the deduced
    /// [hue] and [chroma].
    ///
    /// If the palette is constructed from a hue and chroma (i.e. using [of] or
    /// [fromHct]), the result is generated from the given [hue] and [chroma].
    pub fn get(&mut self, tone: i64) -> i64 {
        *self
            ._cache
            .entry(tone)
            .or_insert_with(|| Hct::from(self.hue, self.chroma, tone as f64).to_int())
    }

    /// Returns the HCT color at the given [tone].
    ///
    /// If the palette is constructed from a list of colors
    /// (i.e. using [fromList]), the color provided at construction is returned
    /// if possible; otherwise the result is generated from the deduced
    /// [hue] and [chroma].
    ///
    /// If the palette is constructed from a hue and chroma (i.e. using [of] or
    /// [fromHct]), the result is generated from the given [hue] and [chroma].
    pub fn get_hct(&self, tone: f64) -> Hct {
        let maybe_hct_int = self._cache.get(&(tone as i64));

        match maybe_hct_int {
            Some(hct_int) => Hct::from_int(*hct_int),
            None => Hct::from(self.hue, self.chroma, tone),
        }
    }
}

impl ToString for TonalPalette {
    fn to_string(&self) -> String {
        let mut copy = self.clone();
        if self._is_from_cache {
            return format!("TonalPalette.of({}, {})", copy.hue, copy.chroma);
        } else {
            let list = copy.get_as_list();
            let mp: Vec<String> = list.iter().map(|it| it.to_string()).collect();
            return format!("TonalPalette.fromList([{}])", mp.join(", "));
        }
    }
}

impl Eq for TonalPalette {}
impl PartialEq for TonalPalette {
    fn eq(&self, other: &Self) -> bool {
        if !self._is_from_cache && !other._is_from_cache {
            // Both created with .of or .fromHct
            return self.hue == other.hue && self.chroma == other.chroma;
        } else {
            let mut self_cpy = self.clone();
            let mut other_cpy = other.clone();
            return ListEquality::equals(&self_cpy.get_as_list(), &other_cpy.get_as_list());
        }
    }
}
impl Hash for TonalPalette {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if !self._is_from_cache {
            self.hue.to_ne_bytes().hash(state);
            self.chroma.to_ne_bytes().hash(state);
        } else {
            let list = self.clone().get_as_list();
            hash_vec_i64(&list, state);
        }
    }
}

struct ListEquality {}
impl ListEquality {
    pub fn equals(a: &Vec<i64>, b: &Vec<i64>) -> bool {
        if a.len() == b.len() {
            for (ela, elb) in a.iter().zip(b.iter()) {
                if ela != elb {
                    return false;
                }
            }
            return true;
        } else {
            return false;
        }
    }
}

fn hash_vec_i64<H: Hasher>(vec: &Vec<i64>, state: &mut H) {
    for element in vec {
        element.hash(state);
    }
}
