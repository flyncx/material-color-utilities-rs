use std::collections::HashMap;

use crate::utils::color_utils::ColorUtils;

use super::{
    quantizer::{Quantizer, QuantizerResult},
    quantizer_map::QuantizerMap,
};

pub struct QuantizerWu {
    pub weights: Vec<i64>,
    pub moments_r: Vec<i64>,
    pub moments_g: Vec<i64>,
    pub moments_b: Vec<i64>,
    pub moments: Vec<f64>,
    pub cubes: Vec<Box>,
}
impl Quantizer for QuantizerWu {
    fn quantize(
        &mut self,
        pixels: &Vec<i64>,
        color_count: i64,
        _: Option<bool>,
    ) -> QuantizerResult {
        let result = QuantizerMap {}.quantize(pixels, color_count, None);
        self.construct_histogram(result.color_to_count);
        self.compute_moments();
        let create_boxes_result = self.create_boxes(color_count);
        let results = self.create_result(create_boxes_result.result_count);
        let color_to_count: HashMap<i64, i64> =
            results.iter().map(|e| (e.clone(), 0 as i64)).collect();
        return QuantizerResult::new(color_to_count, None);
    }
    /* @override
    Future<QuantizerResult> quantize(Iterable<int> pixels, int colorCount) async {
          } */
}
impl QuantizerWu {
    // A histogram of all the input colors is constructed. It has the shape of a
    // cube. The cube would be too large if it contained all 16 million colors:
    // historical best practice is to use 5 bits  of the 8 in each channel,
    // reducing the histogram to a volume of ~32,000.
    const INDEX_BITS: i64 = 5;
    const MAX_INDEX: i64 = 32;
    const SIDE_LENGTH: i64 = 33;
    const TOTAL_SIZE: i64 = 35937;

    pub fn new() -> QuantizerWu {
        QuantizerWu {
            weights: Vec::new(),
            moments_r: Vec::new(),
            moments_g: Vec::new(),
            moments_b: Vec::new(),
            moments: Vec::new(),
            cubes: Vec::new(),
        }
    }

    pub fn get_index(r: i64, g: i64, b: i64) -> i64 {
        return (r << (Self::INDEX_BITS * 2))
            + (r << (Self::INDEX_BITS + 1))
            + (g << Self::INDEX_BITS)
            + r
            + g
            + b;
    }

    pub fn construct_histogram(&mut self, pixels: HashMap<i64, i64>) -> () {
        self.weights = vec![0; Self::TOTAL_SIZE as usize];
        self.moments_r = vec![0; Self::TOTAL_SIZE as usize];
        self.moments_g = vec![0; Self::TOTAL_SIZE as usize];
        self.moments_b = vec![0; Self::TOTAL_SIZE as usize];
        self.moments = vec![0.0; Self::TOTAL_SIZE as usize];
        for (pixel, count) in pixels {
            let red = ColorUtils::red_from_argb(pixel);
            let green = ColorUtils::green_from_argb(pixel);
            let blue = ColorUtils::blue_from_argb(pixel);
            let bits_to_remove = 8 - Self::INDEX_BITS;
            let i_r = (red >> bits_to_remove) + 1;
            let i_g = (green >> bits_to_remove) + 1;
            let i_b = (blue >> bits_to_remove) + 1;
            let index = Self::get_index(i_r, i_g, i_b);
            self.weights[index as usize] += count;
            self.moments_r[index as usize] += red * count;
            self.moments_g[index as usize] += green * count;
            self.moments_b[index as usize] += blue * count;
            self.moments[index as usize] +=
                (count * ((red * red) + (green * green) + (blue * blue))) as f64;
        }
    }

    pub fn compute_moments(&mut self) -> () {
        for r in 1..Self::SIDE_LENGTH {
            let mut area: Vec<i64> = vec![0; Self::SIDE_LENGTH as usize];
            let mut area_r: Vec<i64> = vec![0; Self::SIDE_LENGTH as usize];
            let mut area_g: Vec<i64> = vec![0; Self::SIDE_LENGTH as usize];
            let mut area_b: Vec<i64> = vec![0; Self::SIDE_LENGTH as usize];
            let mut area2: Vec<f64> = vec![0.0; Self::SIDE_LENGTH as usize];
            for g in 1..Self::SIDE_LENGTH {
                let mut line: i64 = 0;
                let mut line_r: i64 = 0;
                let mut line_g: i64 = 0;
                let mut line_b: i64 = 0;
                let mut line2: f64 = 0.0;
                for b in 1..Self::SIDE_LENGTH {
                    let index: i64 = Self::get_index(r, g, b);
                    line += self.weights[index as usize];
                    line_r += self.moments_r[index as usize];
                    line_g += self.moments_g[index as usize];
                    line_b += self.moments_b[index as usize];
                    line2 += self.moments[index as usize];

                    area[b as usize] += line;
                    area_r[b as usize] += line_r;
                    area_g[b as usize] += line_g;
                    area_b[b as usize] += line_b;
                    area2[b as usize] += line2;

                    let previous_index: i64 = Self::get_index(r - 1, g, b);
                    self.weights[index as usize] =
                        self.weights[previous_index as usize] + area[b as usize];
                    self.moments_r[index as usize] =
                        self.moments_r[previous_index as usize] + area_r[b as usize];
                    self.moments_g[index as usize] =
                        self.moments_g[previous_index as usize] + area_g[b as usize];
                    self.moments_b[index as usize] =
                        self.moments_b[previous_index as usize] + area_b[b as usize];
                    self.moments[index as usize] =
                        self.moments[previous_index as usize] + area2[b as usize];
                }
            }
        }
    }

    pub fn create_boxes(&mut self, max_color_count: i64) -> CreateBoxesResult {
        self.cubes = (0..max_color_count)
            .map(|_| Box::new(None, None, None, None, None, None, None))
            .collect();
        self.cubes[0] = Box::new(
            Some(0),
            Some(Self::MAX_INDEX),
            Some(0),
            Some(Self::MAX_INDEX),
            Some(0),
            Some(Self::MAX_INDEX),
            Some(0),
        );

        let mut volume_variance: Vec<f64> = vec![0.0; max_color_count as usize];
        let mut next: i64 = 0;
        let mut generated_color_count: i64 = max_color_count;
        for mut i in 1..max_color_count {
            let one = next as usize;
            let two = i as usize;
            if self.cut(one, two) {
                volume_variance[next as usize] = if self.cubes[next as usize].vol > 1 {
                    self.variance(self.cubes[next as usize])
                } else {
                    0.0
                };
                volume_variance[i as usize] = if self.cubes[i as usize].vol > 1 {
                    self.variance(self.cubes[i as usize])
                } else {
                    0.0
                };
            } else {
                volume_variance[next as usize] = 0.0;
                i -= 1;
            }

            next = 0;
            let mut temp: f64 = volume_variance[0];
            for j in 1..=i {
                if volume_variance[j as usize] > temp {
                    temp = volume_variance[j as usize];
                    next = j;
                }
            }
            if temp <= 0.0 {
                generated_color_count = i + 1;
                break;
            }
        }

        return CreateBoxesResult::new(max_color_count, generated_color_count);
    }

    pub fn create_result(&self, color_count: i64) -> Vec<i64> {
        let mut colors: Vec<i64> = Vec::new();
        for i in 0..color_count {
            let cube = &self.cubes[i as usize];
            let weight: i64 = Self::volume(cube.clone(), self.weights.clone());
            if weight > 0 {
                let r: i64 = ((Self::volume(cube.clone(), self.moments_r.clone()) / weight) as f64)
                    .round() as i64;
                let g: i64 = ((Self::volume(cube.clone(), self.moments_g.clone()) / weight) as f64)
                    .round() as i64;
                let b: i64 = ((Self::volume(cube.clone(), self.moments_b.clone()) / weight) as f64)
                    .round() as i64;
                let color: i64 = ColorUtils::argb_from_rgb(r, g, b);
                colors.push(color);
            }
        }
        return colors;
    }

    pub fn variance(&self, cube: Box) -> f64 {
        let dr = Self::volume(cube.clone(), self.moments_r.clone());
        let dg = Self::volume(cube.clone(), self.moments_g.clone());
        let db = Self::volume(cube.clone(), self.moments_b.clone());
        let xx = self.moments[Self::get_index(cube.r1, cube.g1, cube.b1) as usize]
            - self.moments[Self::get_index(cube.r1, cube.g1, cube.b0) as usize]
            - self.moments[Self::get_index(cube.r1, cube.g0, cube.b1) as usize]
            + self.moments[Self::get_index(cube.r1, cube.g0, cube.b0) as usize]
            - self.moments[Self::get_index(cube.r0, cube.g1, cube.b1) as usize]
            + self.moments[Self::get_index(cube.r0, cube.g1, cube.b0) as usize]
            + self.moments[Self::get_index(cube.r0, cube.g0, cube.b1) as usize]
            - self.moments[Self::get_index(cube.r0, cube.g0, cube.b0) as usize];

        let hypotenuse = dr * dr + dg * dg + db * db;
        let volume_ = Self::volume(cube, self.weights.clone());
        return xx - (hypotenuse / volume_) as f64;
    }

    pub fn cut(&mut self, one_index: usize, two_index: usize) -> bool {
        let whole_r = Self::volume(self.cubes[one_index].clone(), self.moments_r.clone());
        let whole_g = Self::volume(self.cubes[one_index].clone(), self.moments_g.clone());
        let whole_b = Self::volume(self.cubes[one_index].clone(), self.moments_b.clone());
        let whole_w = Self::volume(self.cubes[one_index].clone(), self.weights.clone());

        let max_rresult = self.maximize(
            self.cubes[one_index].clone(),
            Direction::Red,
            self.cubes[one_index].r0 + 1,
            self.cubes[one_index].r1,
            whole_r,
            whole_g,
            whole_b,
            whole_w,
        );
        let max_gresult = self.maximize(
            self.cubes[one_index].clone(),
            Direction::Green,
            self.cubes[one_index].g0 + 1,
            self.cubes[one_index].g1,
            whole_r,
            whole_g,
            whole_b,
            whole_w,
        );
        let max_bresult = self.maximize(
            self.cubes[one_index].clone(),
            Direction::Blue,
            self.cubes[one_index].b0 + 1,
            self.cubes[one_index].b1,
            whole_r,
            whole_g,
            whole_b,
            whole_w,
        );

        let cut_direction: Direction;
        let max_r = max_rresult.maximum;
        let max_g = max_gresult.maximum;
        let max_b = max_bresult.maximum;
        if max_r >= max_g && max_r >= max_b {
            cut_direction = Direction::Red;
            if max_rresult.cut_location < 0 {
                return false;
            }
        } else if max_g >= max_r && max_g >= max_b {
            cut_direction = Direction::Green;
        } else {
            cut_direction = Direction::Blue;
        }

        self.cubes[two_index].r1 = self.cubes[one_index].r1;
        self.cubes[two_index].g1 = self.cubes[one_index].g1;

        self.cubes[two_index].b1 = self.cubes[one_index].b1;
        match cut_direction {
            Direction::Red => {
                self.cubes[one_index].r1 = max_rresult.cut_location;
                self.cubes[two_index].r0 = self.cubes[one_index].r1;
                self.cubes[two_index].g0 = self.cubes[one_index].g0;
                self.cubes[two_index].b0 = self.cubes[one_index].b0;
            }
            Direction::Green => {
                self.cubes[one_index].g1 = max_gresult.cut_location;
                self.cubes[two_index].r0 = self.cubes[one_index].r0;
                self.cubes[two_index].g0 = self.cubes[one_index].g1;
                self.cubes[two_index].b0 = self.cubes[one_index].b0;
            }
            Direction::Blue => {
                self.cubes[one_index].b1 = max_bresult.cut_location;
                self.cubes[two_index].r0 = self.cubes[one_index].r0;
                self.cubes[two_index].g0 = self.cubes[one_index].g0;
                self.cubes[two_index].b0 = self.cubes[one_index].b1;
            }
        }

        self.cubes[one_index].vol = (self.cubes[one_index].r1 - self.cubes[one_index].r0)
            * (self.cubes[one_index].g1 - self.cubes[one_index].g0)
            * (self.cubes[one_index].b1 - self.cubes[one_index].b0);
        self.cubes[two_index].vol = (self.cubes[two_index].r1 - self.cubes[two_index].r0)
            * (self.cubes[two_index].g1 - self.cubes[two_index].g0)
            * (self.cubes[two_index].b1 - self.cubes[two_index].b0);

        return true;
    }

    pub fn maximize(
        &self,
        cube: Box,
        direction: Direction,
        first: i64,
        last: i64,
        whole_r: i64,
        whole_g: i64,
        whole_b: i64,
        whole_w: i64,
    ) -> MaximizeResult {
        let bottom_r = Self::bottom(cube.clone(), direction.clone(), self.moments_r.clone());
        let bottom_g = Self::bottom(cube.clone(), direction.clone(), self.moments_g.clone());
        let bottom_b = Self::bottom(cube.clone(), direction.clone(), self.moments_b.clone());
        let bottom_w = Self::bottom(cube.clone(), direction.clone(), self.weights.clone());

        let mut max: f64 = 0.0;
        let mut cut: i64 = -1;
        for i in first..last {
            let mut half_r: i64 =
                bottom_r + Self::top(cube.clone(), direction.clone(), i, self.moments_r.clone());
            let mut half_g: i64 =
                bottom_g + Self::top(cube.clone(), direction.clone(), i, self.moments_g.clone());
            let mut half_b: i64 =
                bottom_b + Self::top(cube.clone(), direction.clone(), i, self.moments_b.clone());
            let mut half_w: i64 =
                bottom_w + Self::top(cube.clone(), direction.clone(), i, self.weights.clone());

            if half_w == 0 {
                continue;
            }

            let mut temp_numerator: f64 =
                ((half_r * half_r) + (half_g * half_g) + (half_b * half_b)) as f64;
            let mut temp_denominator: f64 = half_w as f64;
            let mut temp: f64 = temp_numerator / temp_denominator;

            half_r = whole_r - half_r;
            half_g = whole_g - half_g;
            half_b = whole_b - half_b;
            half_w = whole_w - half_w;
            if half_w == 0 {
                continue;
            }
            temp_numerator = ((half_r * half_r) + (half_g * half_g) + (half_b * half_b)) as f64;
            temp_denominator = half_w as f64;
            temp += temp_numerator / temp_denominator;

            if temp > max {
                max = temp;
                cut = i;
            }
        }
        return MaximizeResult::new(cut, max);
    }

    pub fn volume(cube: Box, moment: Vec<i64>) -> i64 {
        return moment[Self::get_index(cube.r1, cube.g1, cube.b1) as usize]
            - moment[Self::get_index(cube.r1, cube.g1, cube.b0) as usize]
            - moment[Self::get_index(cube.r1, cube.g0, cube.b1) as usize]
            + moment[Self::get_index(cube.r1, cube.g0, cube.b0) as usize]
            - moment[Self::get_index(cube.r0, cube.g1, cube.b1) as usize]
            + moment[Self::get_index(cube.r0, cube.g1, cube.b0) as usize]
            + moment[Self::get_index(cube.r0, cube.g0, cube.b1) as usize]
            - moment[Self::get_index(cube.r0, cube.g0, cube.b0) as usize];
    }

    pub fn bottom(cube: Box, direction: Direction, moment: Vec<i64>) -> i64 {
        match direction {
            Direction::Red => {
                return -moment[Self::get_index(cube.r0, cube.g1, cube.b1) as usize]
                    + moment[Self::get_index(cube.r0, cube.g1, cube.b0) as usize]
                    + moment[Self::get_index(cube.r0, cube.g0, cube.b1) as usize]
                    - moment[Self::get_index(cube.r0, cube.g0, cube.b0) as usize];
            }
            Direction::Green => {
                return -moment[Self::get_index(cube.r1, cube.g0, cube.b1) as usize]
                    + moment[Self::get_index(cube.r1, cube.g0, cube.b0) as usize]
                    + moment[Self::get_index(cube.r0, cube.g0, cube.b1) as usize]
                    - moment[Self::get_index(cube.r0, cube.g0, cube.b0) as usize];
            }
            Direction::Blue => {
                return -moment[Self::get_index(cube.r1, cube.g1, cube.b0) as usize]
                    + moment[Self::get_index(cube.r1, cube.g0, cube.b0) as usize]
                    + moment[Self::get_index(cube.r0, cube.g1, cube.b0) as usize]
                    - moment[Self::get_index(cube.r0, cube.g0, cube.b0) as usize];
            }
        }
    }

    pub fn top(cube: Box, direction: Direction, position: i64, moment: Vec<i64>) -> i64 {
        match direction {
            Direction::Red => {
                return moment[Self::get_index(position, cube.g1, cube.b1) as usize]
                    - moment[Self::get_index(position, cube.g1, cube.b0) as usize]
                    - moment[Self::get_index(position, cube.g0, cube.b1) as usize]
                    + moment[Self::get_index(position, cube.g0, cube.b0) as usize];
            }
            Direction::Green => {
                return moment[Self::get_index(cube.r1, position, cube.b1) as usize]
                    - moment[Self::get_index(cube.r1, position, cube.b0) as usize]
                    - moment[Self::get_index(cube.r0, position, cube.b1) as usize]
                    + moment[Self::get_index(cube.r0, position, cube.b0) as usize];
            }
            Direction::Blue => {
                return moment[Self::get_index(cube.r1, cube.g1, position) as usize]
                    - moment[Self::get_index(cube.r1, cube.g0, position) as usize]
                    - moment[Self::get_index(cube.r0, cube.g1, position) as usize]
                    + moment[Self::get_index(cube.r0, cube.g0, position) as usize];
            }
        }
    }
}

#[derive(Clone)]
pub enum Direction {
    Red,
    Green,
    Blue,
}

pub struct MaximizeResult {
    // < 0 if cut impossible
    pub cut_location: i64,
    pub maximum: f64,
}
impl MaximizeResult {
    pub fn new(cut_location: i64, maximum: f64) -> MaximizeResult {
        MaximizeResult {
            cut_location,
            maximum,
        }
    }
}

pub struct CreateBoxesResult {
    pub requested_count: i64,
    pub result_count: i64,
}
impl CreateBoxesResult {
    pub fn new(requested_count: i64, result_count: i64) -> CreateBoxesResult {
        CreateBoxesResult {
            requested_count,
            result_count,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Box {
    pub r0: i64,
    pub r1: i64,
    pub g0: i64,
    pub g1: i64,
    pub b0: i64,
    pub b1: i64,
    pub vol: i64,
}
impl Box {
    pub fn new(
        r0: Option<i64>,
        r1: Option<i64>,
        g0: Option<i64>,
        g1: Option<i64>,
        b0: Option<i64>,
        b1: Option<i64>,
        vol: Option<i64>,
    ) -> Box {
        Box {
            r0: r0.unwrap_or(0),
            r1: r1.unwrap_or(0),
            g0: g0.unwrap_or(0),
            g1: g1.unwrap_or(0),
            b0: b0.unwrap_or(0),
            b1: b1.unwrap_or(0),
            vol: vol.unwrap_or(0),
        }
    }
}
impl ToString for Box {
    fn to_string(&self) -> String {
        format!("Box: R $r0 -> $r1 G  $g0 -> $g1 B $b0 -> $b1 VOL = $vol")
    }
}
