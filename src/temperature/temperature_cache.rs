use std::collections::HashMap;

use crate::{
    hct::hct::Hct,
    utils::{color_utils::ColorUtils, math_utils::MathUtils},
};

/// Design utilities using color temperature theory.
///
/// Analogous colors, complementary color, and cache to efficiently, lazily,
/// generate data for calculations when needed.
pub struct TemperatureCache {
    pub input: Hct,
    _hcts_by_temp: Vec<Hct>,
    _hcts_by_hue: Vec<Hct>,
    _temps_by_hct: HashMap<Hct, f64>,
    _input_relative_temperature: f64,
    _complement: Option<Hct>,
}
impl TemperatureCache {
    pub fn new(input: Hct) -> TemperatureCache {
        TemperatureCache {
            input,
            _hcts_by_temp: Vec::new(),
            _hcts_by_hue: Vec::new(),
            _temps_by_hct: HashMap::new(),
            _input_relative_temperature: -1.0,
            _complement: None,
        }
    }
    pub fn get_warmest(&mut self) -> Hct {
        self.get_hcts_by_temp()
            .last()
            .expect("Cannot get Warmest Hct")
            .clone()
    }
    pub fn get_coldest(&mut self) -> Hct {
        self.get_hcts_by_temp()
            .first()
            .expect("Cannot get Coldest hct")
            .clone()
    }

    /// A set of colors with differing hues, equidistant in temperature.
    ///
    /// In art, this is usually described as a set of 5 colors on a color wheel
    /// divided into 12 sections. This method allows provision of either of those
    /// values.
    ///
    /// Behavior is undefined when [count] or [divisions] is 0.
    /// When divisions < count, colors repeat.
    ///
    /// [count] The number of colors to return, includes the input color.
    /// [divisions] The number of divisions on the color wheel.
    pub fn analogous(&mut self, count: Option<i64>, divisions: Option<i64>) -> Vec<Hct> {
        let count = count.unwrap_or(5);
        let divisions = divisions.unwrap_or(12);

        let start_hue = self.input.get_hue().round() as i64;
        let start_hct = &self.get_hcts_by_hue()[start_hue as usize];
        let mut last_temp = self.relative_temperature(start_hct.clone());
        let mut all_colors: Vec<Hct> = Vec::new();
        all_colors.push(start_hct.clone());

        let mut absolute_total_temp_delta = 0.0;
        for i in 0..360 {
            let hue = MathUtils::sanitize_degrees_int(start_hue + i);
            let hct = &self.get_hcts_by_hue()[hue as usize];
            let temp = self.relative_temperature(hct.clone());
            let temp_delta = (temp - last_temp).abs();
            last_temp = temp;
            absolute_total_temp_delta += temp_delta;
        }
        let mut hue_addend = 1;
        let temp_step = absolute_total_temp_delta / divisions as f64;
        let mut total_temp_delta = 0.0;
        last_temp = self.relative_temperature(start_hct.clone());
        while (all_colors.len() as i64) < divisions {
            let hue = MathUtils::sanitize_degrees_int(start_hue + hue_addend);
            let hct = &self.get_hcts_by_hue()[hue as usize];
            let temp = self.relative_temperature(hct.clone());
            let temp_delta = (temp - last_temp).abs();
            total_temp_delta += temp_delta;

            let desired_total_temp_delta_for_index = (all_colors.len() as f64) * temp_step;
            let mut index_satisfied = total_temp_delta >= desired_total_temp_delta_for_index;
            let mut index_addend = 1;
            // Keep adding this hue to the answers until its temperature is
            // insufficient. This ensures consistent behavior when there aren't
            // [divisions] discrete steps between 0 and 360 in hue with [tempStep]
            // delta in temperature between them.
            //
            // For example, white and black have no analogues: there are no other
            // colors at T100/T0. Therefore, they should just be added to the array
            // as answers.
            while index_satisfied && (all_colors.len() as i64) < divisions {
                all_colors.push(hct.clone());
                let desired_total_temp_delta_for_index =
                    ((all_colors.len() + index_addend) as f64) * temp_step;
                index_satisfied = total_temp_delta >= desired_total_temp_delta_for_index;
                index_addend += 1;
            }
            last_temp = temp;
            hue_addend += 1;
            if hue_addend > 360 {
                while (all_colors.len() as i64) < divisions {
                    all_colors.push(hct.clone());
                }
                break;
            }
        }

        let mut answers: Vec<Hct> = Vec::new();
        answers.push(self.input.clone());

        // First, generate analogues from rotating counter-clockwise.
        let increase_hue_count = (((count - 1) as f64) / 2.0).floor() as i64;
        for i in 1..(increase_hue_count + 1) {
            let mut index = 0 - i;
            while index < 0 {
                index = all_colors.len() as i64 + index;
            }
            if index >= all_colors.len() as i64 {
                index = index % all_colors.len() as i64;
            }
            answers.insert(0, all_colors[index as usize].clone());
        }

        // Second, generate analogues from rotating clockwise.
        let decrease_hue_count = count - increase_hue_count - 1;
        for i in 1..(decrease_hue_count + 1) {
            let mut index = i;
            while index < 0 {
                index = all_colors.len() as i64 + index;
            }
            if index >= all_colors.len() as i64 {
                index = index % all_colors.len() as i64;
            }
            answers.push(all_colors[index as usize].clone());
        }

        return answers;
    }

    /// A color that complements the input color aesthetically.
    ///
    /// In art, this is usually described as being across the color wheel.
    /// History of this shows intent as a color that is just as cool-warm as the
    /// input color is warm-cool.
    pub fn get_complement(&mut self) -> Hct {
        if let Some(_compliment) = &self._complement {
            return _compliment.clone();
        }
        let coldest_hue = self.get_coldest().get_hue();
        let coldest_temp = self.get_temps_by_hct()[&self.get_coldest()];

        let warmest_hue = self.get_warmest().get_hue();
        let warmest_temp = self.get_temps_by_hct()[&self.get_warmest()];
        let range = warmest_temp - coldest_temp;
        let start_hue_is_coldest_to_warmest =
            Self::is_between(self.input.get_hue(), coldest_hue, warmest_hue);
        let start_hue = {
            if start_hue_is_coldest_to_warmest {
                warmest_hue
            } else {
                coldest_hue
            }
        };
        let end_hue = {
            if start_hue_is_coldest_to_warmest {
                coldest_hue
            } else {
                warmest_hue
            }
        };
        let direction_of_rotation = 1.0;
        let mut smallest_error = 1000.0;
        let mut answer = self._hcts_by_hue[self.input.get_hue().round() as usize].clone();

        let complement_relative_temp = 1.0 - self.get_input_relative_temperature();
        // Find the color in the other section, closest to the inverse percentile
        // of the input color. This is the complement.
        for hue_addend in 0..360 {
            let hue_addend = hue_addend as f64;
            let hue =
                MathUtils::sanitize_degrees_double(start_hue + direction_of_rotation * hue_addend);
            if !Self::is_between(hue, start_hue, end_hue) {
                continue;
            }
            let possible_answer = &self._hcts_by_hue[hue.round() as usize];
            let relative_temp = (self._temps_by_hct[&possible_answer] - coldest_temp) / range;
            let error = (complement_relative_temp - relative_temp).abs();
            if error < smallest_error {
                smallest_error = error;
                answer = possible_answer.clone();
            }
        }
        self._complement = Some(answer.clone());
        return answer.clone();
    }

    /// Temperature relative to all colors with the same chroma and tone.
    /// Value on a scale from 0 to 1.
    pub fn relative_temperature(&mut self, hct: Hct) -> f64 {
        let range = self.get_temps_by_hct()[&self.get_warmest().clone()]
            - self.get_temps_by_hct()[&self.get_coldest()];
        let difference_from_coldest =
            self.get_temps_by_hct()[&hct] - self.get_temps_by_hct()[&self.get_coldest()];
        // Handle when there's no difference in temperature between warmest and
        // coldest: for example, at T100, only one color is available, white.
        if range == 0.0 {
            return 0.5;
        }
        return difference_from_coldest / range;
    }

    /// Relative temperature of the input color. See [relativeTemperature].
    pub fn get_input_relative_temperature(&mut self) -> f64 {
        if self._input_relative_temperature >= 0.0 {
            return self._input_relative_temperature;
        }

        let coldest_temp = self.get_temps_by_hct()[&self.get_coldest()];

        let range = self.get_temps_by_hct()[&self.get_warmest()] - coldest_temp;
        let difference_from_coldest = self.get_temps_by_hct()[&self.input] - coldest_temp;
        let input_relative_temp = {
            if range == 0.0 {
                0.5
            } else {
                difference_from_coldest / range
            }
        };

        self._input_relative_temperature = input_relative_temp;
        return self._input_relative_temperature;
    }

    /// HCTs for all hues, with the same chroma/tone as the input.
    /// Sorted from coldest first to warmest last.
    pub fn get_hcts_by_temp(&mut self) -> Vec<Hct> {
        if !self._hcts_by_temp.is_empty() {
            let mut hcts_by_temp: Vec<Hct> = Vec::new();
            for hct in &self._hcts_by_temp {
                hcts_by_temp.push(hct.clone());
            }
            return hcts_by_temp;
        }

        let mut hcts: Vec<Hct> = self.get_hcts_by_hue();
        hcts.push(self.input.clone());
        let temperatures_by_hct = self.get_temps_by_hct();
        hcts.sort_by(|a, b| {
            let temperature_a = temperatures_by_hct[a];
            let temperature_b = temperatures_by_hct[b];
            return temperature_a
                .partial_cmp(&temperature_b)
                .unwrap_or(std::cmp::Ordering::Equal);
        });
        self._hcts_by_temp = hcts;
        return self._hcts_by_temp.clone();
    }

    /// A Map with keys of HCTs in [hctsByTemp], values of raw temperature.
    pub fn get_temps_by_hct(&mut self) -> HashMap<Hct, f64> {
        if !self._temps_by_hct.is_empty() {
            return self._temps_by_hct.clone();
        }
        let mut all_hcts: Vec<Hct> = self.get_hcts_by_hue();
        all_hcts.push(self.input.clone());
        let mut temperatures_by_hct: HashMap<Hct, f64> = HashMap::new();
        for e in all_hcts {
            temperatures_by_hct.insert(e.clone(), Self::raw_temperature(e));
        }
        //{for (var e in allHcts) e: rawTemperature(e)};
        self._temps_by_hct = temperatures_by_hct;
        return self._temps_by_hct.clone();
    }

    /// HCTs for all hues, with the same chroma/tone as the input.
    /// Sorted ascending, hue 0 to 360.
    pub fn get_hcts_by_hue(&mut self) -> Vec<Hct> {
        if !self._hcts_by_hue.is_empty() {
            return self._hcts_by_hue.clone();
        }
        let mut hcts: Vec<Hct> = Vec::new();
        for hue in 0..360 {
            let hue = hue as f64;
            let color_at_hue = Hct::from(hue, self.input.get_chroma(), self.input.get_tone());
            hcts.push(color_at_hue);
        }
        self._hcts_by_hue = hcts;

        return self._hcts_by_hue.clone();
    }

    /// Determines if an angle is between two other angles, rotating clockwise.
    pub fn is_between(angle: f64, a: f64, b: f64) -> bool {
        if a < b {
            return a <= angle && angle <= b;
        }
        return a <= angle || angle <= b;
    }

    /// Value representing cool-warm factor of a color.
    /// Values below 0 are considered cool, above, warm.
    ///
    /// Color science has researched emotion and harmony, which art uses to select
    /// colors. Warm-cool is the foundation of analogous and complementary colors.
    /// See:
    /// - Li-Chen Ou's Chapter 19 in Handbook of Color Psychology (2015).
    /// - Josef Albers' Interaction of Color chapters 19 and 21.
    ///
    /// Implementation of Ou, Woodcock and Wright's algorithm, which uses
    /// L*a*b* / LCH color space.
    /// Return value has these properties:
    /// - Values below 0 are cool, above 0 are warm.
    /// - Lower bound: -0.52 - (chroma ^ 1.07 / 20). L*a*b* chroma is infinite.
    ///   Assuming max of 130 chroma, -9.66.
    /// - Upper bound: -0.52 + (chroma ^ 1.07 / 20). L*a*b* chroma is infinite.
    ///   Assuming max of 130 chroma, 8.61.
    pub fn raw_temperature(color: Hct) -> f64 {
        let lab = ColorUtils::lab_from_argb(color.to_int());
        let hue = MathUtils::sanitize_degrees_double(
            (lab[2]).atan2(lab[1]) * 180.0 / std::f64::consts::PI,
        );
        let chroma = ((lab[1] * lab[1]) + (lab[2] * lab[2])).sqrt();
        let temperature = -0.5
            + 0.02
                * (chroma).powf(1.07)
                * (MathUtils::sanitize_degrees_double(hue - 50.0) * std::f64::consts::PI / 180.0)
                    .cos();
        return temperature;
    }
}
