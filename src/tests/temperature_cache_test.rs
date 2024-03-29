use crate::{hct::hct::Hct, temperature::temperature_cache::TemperatureCache};

#[test]
fn raw_temperature() {
    let blue_temp = TemperatureCache::raw_temperature(&Hct::from_int(0xff0000ff));
    assert_approx_eq::assert_approx_eq!(blue_temp, -1.393, 0.001);

    let red_temp = TemperatureCache::raw_temperature(&Hct::from_int(0xffff0000));
    assert_approx_eq::assert_approx_eq!(red_temp, 2.351, 0.001);

    let green_temp = TemperatureCache::raw_temperature(&Hct::from_int(0xff00ff00));
    assert_approx_eq::assert_approx_eq!(green_temp, -0.267, 0.001);

    let white_temp = TemperatureCache::raw_temperature(&Hct::from_int(0xffffffff));
    assert_approx_eq::assert_approx_eq!(white_temp, -0.5, 0.001);

    let black_temp = TemperatureCache::raw_temperature(&Hct::from_int(0xff000000));
    assert_approx_eq::assert_approx_eq!(black_temp, -0.5, 0.001);
}

#[test]
fn relative_temperature() {
    let blue_temp =
        TemperatureCache::new(Hct::from_int(0xff0000ff)).get_input_relative_temperature();
    assert_approx_eq::assert_approx_eq!(blue_temp, 0.0, 0.001);

    let red_temp =
        TemperatureCache::new(Hct::from_int(0xffff0000)).get_input_relative_temperature();
    assert_approx_eq::assert_approx_eq!(red_temp, 1.0, 0.001);

    let green_temp =
        TemperatureCache::new(Hct::from_int(0xff00ff00)).get_input_relative_temperature();
    assert_approx_eq::assert_approx_eq!(green_temp, 0.467, 0.001);

    let white_temp =
        TemperatureCache::new(Hct::from_int(0xffffffff)).get_input_relative_temperature();
    assert_approx_eq::assert_approx_eq!(white_temp, 0.5, 0.001);

    let black_temp =
        TemperatureCache::new(Hct::from_int(0xff000000)).get_input_relative_temperature();
    assert_approx_eq::assert_approx_eq!(black_temp, 0.5, 0.001);
}

#[test]
fn complement() {
    let blue_complement = TemperatureCache::new(Hct::from_int(0xff0000ff))
        .get_complement()
        .to_int();
    assert_eq!(blue_complement, (0xff9d0002));

    let red_complement = TemperatureCache::new(Hct::from_int(0xffff0000))
        .get_complement()
        .to_int();
    assert_eq!(red_complement, (0xff007bfc));

    let green_complement = TemperatureCache::new(Hct::from_int(0xff00ff00))
        .get_complement()
        .to_int();
    assert_eq!(green_complement, (0xffffd2c9));

    let white_complement = TemperatureCache::new(Hct::from_int(0xffffffff))
        .get_complement()
        .to_int();
    assert_eq!(white_complement, (0xffffffff));

    let black_complement = TemperatureCache::new(Hct::from_int(0xff000000))
        .get_complement()
        .to_int();
    assert_eq!(black_complement, (0xff000000));
}

#[test]
fn analogous() {
    let blue_analogous: Vec<i64> = TemperatureCache::new(Hct::from_int(0xff0000ff))
        .analogous(None, None)
        .iter()
        .map(|e| e.to_int())
        .collect();
    assert_eq!(blue_analogous[0], (0xff00590c));
    assert_eq!(blue_analogous[1], (0xff00564e));
    assert_eq!(blue_analogous[2], (0xff0000ff));
    assert_eq!(blue_analogous[3], (0xff6700cc));
    assert_eq!(blue_analogous[4], (0xff81009f));

    let red_analogous: Vec<i64> = TemperatureCache::new(Hct::from_int(0xffff0000))
        .analogous(None, None)
        .iter()
        .map(|e| e.to_int())
        .collect();

    assert_eq!(red_analogous[0], (0xfff60082));
    assert_eq!(red_analogous[1], (0xfffc004c));
    assert_eq!(red_analogous[2], (0xffff0000));
    assert_eq!(red_analogous[3], (0xffd95500));
    assert_eq!(red_analogous[4], (0xffaf7200));

    let green_analogous: Vec<i64> = TemperatureCache::new(Hct::from_int(0xff00ff00))
        .analogous(None, None)
        .iter()
        .map(|e| e.to_int())
        .collect();

    assert_eq!(green_analogous[0], (0xffcee900));
    assert_eq!(green_analogous[1], (0xff92f500));
    assert_eq!(green_analogous[2], (0xff00ff00));
    assert_eq!(green_analogous[3], (0xff00fd6f));
    assert_eq!(green_analogous[4], (0xff00fab3));

    let black_analogous: Vec<i64> = TemperatureCache::new(Hct::from_int(0xff000000))
        .analogous(None, None)
        .iter()
        .map(|e| e.to_int())
        .collect();

    assert_eq!(black_analogous[0], (0xff000000));
    assert_eq!(black_analogous[1], (0xff000000));
    assert_eq!(black_analogous[2], (0xff000000));
    assert_eq!(black_analogous[3], (0xff000000));
    assert_eq!(black_analogous[4], (0xff000000));

    let white_analogous: Vec<i64> = TemperatureCache::new(Hct::from_int(0xffffffff))
        .analogous(None, None)
        .iter()
        .map(|e| e.to_int())
        .collect();

    assert_eq!(white_analogous[0], (0xffffffff));
    assert_eq!(white_analogous[1], (0xffffffff));
    assert_eq!(white_analogous[2], (0xffffffff));
    assert_eq!(white_analogous[3], (0xffffffff));
    assert_eq!(white_analogous[4], (0xffffffff));
}
