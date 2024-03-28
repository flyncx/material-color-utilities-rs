use std::collections::HashMap;

use crate::score::score::Score;

#[test]
fn prioritizes_chroma() {
    let mut colors_to_population: HashMap<i64, i64> = HashMap::new();
    colors_to_population.insert(0xff000000, 1);
    colors_to_population.insert(0xffffffff, 1);
    colors_to_population.insert(0xff0000ff, 1);

    let ranked = Score::score(colors_to_population, Some(4), None, None);

    assert_eq!(ranked.len(), (1));
    assert_eq!(ranked[0], (0xff0000ff));
}

#[test]
fn prioritizes_chroma_when_proportions_equal() {
    let mut colors_to_population: HashMap<i64, i64> = HashMap::new();
    colors_to_population.insert(0xffff0000, 1);
    colors_to_population.insert(0xff00ff00, 1);
    colors_to_population.insert(0xff0000ff, 1);

    let ranked = Score::score(colors_to_population, Some(4), None, None);

    assert_eq!(ranked.len(), (3));
    assert_eq!(ranked[0], (0xffff0000));
    assert_eq!(ranked[1], (0xff00ff00));
    assert_eq!(ranked[2], (0xff0000ff));
}

#[test]
fn generates_g_blue_when_no_colors_available() {
    let mut colors_to_population: HashMap<i64, i64> = HashMap::new();
    colors_to_population.insert(0xff000000, 1);
    let ranked = Score::score(colors_to_population, Some(4), None, None);
    assert_eq!(ranked.len(), (1));
    assert_eq!(ranked[0], (0xff4285f4));
}

#[test]
fn dedupes_nearby_hues() {
    let mut colors_to_population: HashMap<i64, i64> = HashMap::new();
    colors_to_population.insert(0xff008772, 1); // H 180 C 42 T 50
    colors_to_population.insert(0xff318477, 1); // H 184 C 35 T 50

    let ranked = Score::score(colors_to_population, Some(4), None, None);
    assert_eq!(ranked.len(), (1));
    assert_eq!(ranked[0], (0xff008772));
}

#[test]
fn maximizes_hue_distance() {
    let mut colors_to_population: HashMap<i64, i64> = HashMap::new();
    colors_to_population.insert(0xff008772, 1); // H 180 C 42 T 50
    colors_to_population.insert(0xff008587, 1); // H 198 C 50 T 50
    colors_to_population.insert(0xff007ebc, 1); // H 245 C 50 T 50

    let ranked = Score::score(colors_to_population, Some(2), None, None);
    assert_eq!(ranked.len(), (2));
    assert_eq!(ranked[0], (0xff007ebc));
    assert_eq!(ranked[1], (0xff008772));
}

#[test]
fn passes_generated_scenario_one() {
    let mut colors_to_population: HashMap<i64, i64> = HashMap::new();
    colors_to_population.insert(0xff7ea16d, 67);
    colors_to_population.insert(0xffd8ccae, 67);
    colors_to_population.insert(0xff835c0d, 49);

    let ranked = Score::score(colors_to_population, Some(3), Some(0xff8d3819), Some(false));
    assert_eq!(ranked.len(), (3));
    assert_eq!(ranked[0], (0xff7ea16d));
    assert_eq!(ranked[1], (0xffd8ccae));
    assert_eq!(ranked[2], (0xff835c0d));
}

#[test]
fn passes_generated_scenario_two() {
    let mut colors_to_population: HashMap<i64, i64> = HashMap::new();
    colors_to_population.insert(0xffd33881, 14);
    colors_to_population.insert(0xff3205cc, 77);
    colors_to_population.insert(0xff0b48cf, 36);
    colors_to_population.insert(0xffa08f5d, 81);

    let ranked = Score::score(colors_to_population, Some(4), Some(0xff7d772b), Some(true));
    assert_eq!(ranked.len(), (3));
    assert_eq!(ranked[0], (0xff3205cc));
    assert_eq!(ranked[1], (0xffa08f5d));
    assert_eq!(ranked[2], (0xffd33881));
}

#[test]
fn passes_generated_scenario_three() {
    let mut colors_to_population: HashMap<i64, i64> = HashMap::new();
    colors_to_population.insert(0xffbe94a6, 23);
    colors_to_population.insert(0xffc33fd7, 42);
    colors_to_population.insert(0xff899f36, 90);
    colors_to_population.insert(0xff94c574, 82);

    let ranked = Score::score(colors_to_population, Some(3), Some(0xffaa79a4), Some(true));
    assert_eq!(ranked.len(), (3));
    assert_eq!(ranked[0], (0xff94c574));
    assert_eq!(ranked[1], (0xffc33fd7));
    assert_eq!(ranked[2], (0xffbe94a6));
}

#[test]
fn passes_generated_scenario_four() {
    let mut colors_to_population: HashMap<i64, i64> = HashMap::new();
    colors_to_population.insert(0xffdf241c, 85);
    colors_to_population.insert(0xff685859, 44);
    colors_to_population.insert(0xffd06d5f, 34);
    colors_to_population.insert(0xff561c54, 27);
    colors_to_population.insert(0xff713090, 88);

    let ranked = Score::score(colors_to_population, Some(5), Some(0xff58c19c), Some(false));

    assert_eq!(ranked.len(), (2));
    assert_eq!(ranked[0], (0xffdf241c));
    assert_eq!(ranked[1], (0xff561c54));
}

#[test]
fn passes_generated_scenario_five() {
    let mut colors_to_population: HashMap<i64, i64> = HashMap::new();
    colors_to_population.insert(0xffbe66f8, 41);
    colors_to_population.insert(0xff4bbda9, 88);
    colors_to_population.insert(0xff80f6f9, 44);
    colors_to_population.insert(0xffab8017, 43);
    colors_to_population.insert(0xffe89307, 65);

    let ranked = Score::score(colors_to_population, Some(3), Some(0xff916691), Some(false));

    assert_eq!(ranked.len(), (3));
    assert_eq!(ranked[0], (0xffab8017));
    assert_eq!(ranked[1], (0xff4bbda9));
    assert_eq!(ranked[2], (0xffbe66f8));
}

#[test]
fn passes_generated_scenario_six() {
    let mut colors_to_population: HashMap<i64, i64> = HashMap::new();
    colors_to_population.insert(0xff18ea8f, 93);
    colors_to_population.insert(0xff327593, 18);
    colors_to_population.insert(0xff066a18, 53);
    colors_to_population.insert(0xfffa8a23, 74);
    colors_to_population.insert(0xff04ca1f, 62);

    let ranked = Score::score(colors_to_population, Some(2), Some(0xff4c377a), Some(false));

    assert_eq!(ranked.len(), (2));
    assert_eq!(ranked[0], (0xff18ea8f));
    assert_eq!(ranked[1], (0xfffa8a23));
}

#[test]
fn passes_generated_scenario_seven() {
    let mut colors_to_population: HashMap<i64, i64> = HashMap::new();
    colors_to_population.insert(0xff2e05ed, 23);
    colors_to_population.insert(0xff153e55, 90);
    colors_to_population.insert(0xff9ab220, 23);
    colors_to_population.insert(0xff153379, 66);
    colors_to_population.insert(0xff68bcc3, 81);

    let ranked = Score::score(colors_to_population, Some(2), Some(0xfff588dc), Some(true));

    assert_eq!(ranked.len(), (2));
    assert_eq!(ranked[0], (0xff2e05ed));
    assert_eq!(ranked[1], (0xff9ab220));
}

#[test]
fn passes_generated_scenario_eight() {
    let mut colors_to_population: HashMap<i64, i64> = HashMap::new();
    colors_to_population.insert(0xff816ec5, 24);
    colors_to_population.insert(0xff6dcb94, 19);
    colors_to_population.insert(0xff3cae91, 98);
    colors_to_population.insert(0xff5b542f, 25);

    let ranked = Score::score(colors_to_population, Some(1), Some(0xff84b0fd), Some(false));

    assert_eq!(ranked.len(), (1));
    assert_eq!(ranked[0], (0xff3cae91));
}

#[test]
fn passes_generated_scenario_nine() {
    let mut colors_to_population: HashMap<i64, i64> = HashMap::new();
    colors_to_population.insert(0xff206f86, 52);
    colors_to_population.insert(0xff4a620d, 96);
    colors_to_population.insert(0xfff51401, 85);
    colors_to_population.insert(0xff2b8ebf, 3);
    colors_to_population.insert(0xff277766, 59);

    let ranked = Score::score(colors_to_population, Some(3), Some(0xff02b415), Some(true));

    assert_eq!(ranked.len(), (3));
    assert_eq!(ranked[0], (0xfff51401));
    assert_eq!(ranked[1], (0xff4a620d));
    assert_eq!(ranked[2], (0xff2b8ebf));
}

#[test]
fn passes_generated_scenario_ten() {
    let mut colors_to_population: HashMap<i64, i64> = HashMap::new();
    colors_to_population.insert(0xff8b1d99, 54);
    colors_to_population.insert(0xff27effe, 43);
    colors_to_population.insert(0xff6f558d, 2);
    colors_to_population.insert(0xff77fdf2, 78);

    let ranked = Score::score(colors_to_population, Some(4), Some(0xff5e7a10), Some(true));

    assert_eq!(ranked.len(), (3));
    assert_eq!(ranked[0], (0xff27effe));
    assert_eq!(ranked[1], (0xff8b1d99));
    assert_eq!(ranked[2], (0xff6f558d));
}
