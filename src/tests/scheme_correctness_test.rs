use crate::{
    contrast::contrast::Contrast,
    dynamiccolor::{
        dynamic_color::DynamicColor,
        dynamic_scheme::DynamicScheme,
        material_dynamic_colors::MaterialDynamicColors,
        src::{contrast_curve::ContrastCurve, tone_delta_pair::TonePolarity},
        variant::Variant,
    },
    hct::hct::Hct,
    scheme::{
        scheme_content::SchemeContent, scheme_expressive::SchemeExpressive,
        scheme_fidelity::SchemeFidelity, scheme_fruit_salad::SchemeFruitSalad,
        scheme_monochrome::SchemeMonochrome, scheme_neutral::SchemeNeutral,
        scheme_rainbow::SchemeRainbow, scheme_tonal_spot::SchemeTonalSpot,
        scheme_vibrant::SchemeVibrant,
    },
};

trait _Constraint {
    /// Tests this constraint against [scheme], failing if constraint not met.
    fn test_against(&mut self, scheme: &DynamicScheme, tag: &String) -> ();
}

struct _ContrastConstraint {
    pub foreground: DynamicColor,
    pub background: DynamicColor,
    pub contrast_curve: ContrastCurve,
}
impl _ContrastConstraint {
    const _CONTRAST_TOLERANCE: f64 = 0.05;
    fn _new(
        foreground: DynamicColor,
        background: DynamicColor,
        contrast_curve: ContrastCurve,
    ) -> Self {
        Self {
            foreground,
            background,
            contrast_curve,
        }
    }
}
impl _Constraint for _ContrastConstraint {
    fn test_against(&mut self, scheme: &DynamicScheme, tag: &String) -> () {
        let foreground_color = self.foreground.get_hct(scheme);
        let background_color = self.background.get_hct(scheme);
        let actual_contrast =
            Contrast::ratio_of_tones(foreground_color.get_tone(), background_color.get_tone());
        let desired_contrast = self.contrast_curve.get(scheme.contrast_level);

        if desired_contrast <= 4.5 {
            // A requirement of <= 4.5 must be met (with tolerance)
            if actual_contrast < desired_contrast - Self::_CONTRAST_TOLERANCE {
                panic!(
                    r#"Dynamic scheme $scheme fails contrast constraint:\n'
                '${} should have contrast at least ${} '
                'against ${}, but has ${}\n\n'
                'Foreground: ${}\n'
                'Background: ${}\n'
                'Scheme parameters:\n'
                '  Variant: ${:?}\n'
                '  Source color: ${}\n'
                '  Brightness: ${}\n'
                '  Contrast level: ${}\n'
                'Desired contrast: ${}\n'
                'Actual contrast: ${}"#,
                    self.foreground.name,
                    desired_contrast,
                    self.background.name,
                    actual_contrast,
                    self.foreground.name,
                    self.background.name,
                    scheme.variant,
                    scheme.source_color_hct.to_string(),
                    if scheme.is_dark { "dark" } else { "light" },
                    scheme.contrast_level,
                    desired_contrast,
                    actual_contrast
                );
            }
        } else {
            if actual_contrast < 4.5 - Self::_CONTRAST_TOLERANCE {
                panic!(
                    r#"Dynamic scheme $scheme fails contrast constraint:\n'
                '${} should have contrast at least 4.5 '
                'against ${}, but has ${}\n\n'
                'Foreground: ${}\n'
                'Background: ${}\n'
                'Scheme parameters:\n'
                '  Variant: ${:?}\n'
                '  Source color: ${}\n'
                '  Brightness: ${}\n'
                '  Contrast level: ${}\n'
                'Desired contrast: ${}\n'
                'Actual contrast: ${}"#,
                    self.foreground.name,
                    self.background.name,
                    actual_contrast,
                    self.foreground.name,
                    self.background.name,
                    scheme.variant,
                    scheme.source_color_hct.to_string(),
                    if scheme.is_dark { "dark" } else { "light" },
                    scheme.contrast_level,
                    desired_contrast,
                    actual_contrast
                );
            } else if actual_contrast < desired_contrast - Self::_CONTRAST_TOLERANCE
                && foreground_color.get_tone() != 100.0
                && foreground_color.get_tone() != 0.0
            {
                panic!(
                    r#"tag {}
                    'Dynamic scheme $scheme fails contrast constraint:\n'
                '${} should have contrast at least ${} '
                'against ${}, but has ${}, and no color '
                'has a tone of 0 or 100\n\n'
                'Foreground: ${}\n'
                'Background: ${}\n'
                'Scheme parameters:\n'
                '  Variant: ${:?}\n'
                '  Source color: ${}\n'
                '  Brightness: ${}\n'
                '  Contrast level: ${}\n'
                'Desired contrast: ${}\n'
                'Actual contrast: ${}"#,
                    tag,
                    self.foreground.name,
                    desired_contrast,
                    self.background.name,
                    actual_contrast,
                    self.foreground.name,
                    self.background.name,
                    scheme.variant,
                    scheme.source_color_hct.to_string(),
                    if scheme.is_dark { "dark" } else { "light" },
                    scheme.contrast_level,
                    desired_contrast,
                    actual_contrast
                )
            }
        }
    }
}
struct _DeltaConstraint {
    pub role_a: DynamicColor,
    pub role_b: DynamicColor,
    pub delta: f64,
    pub polarity: TonePolarity,
}
impl _DeltaConstraint {
    const _DELTA_TOLERANCE: f64 = 0.5;

    fn _new(
        role_a: DynamicColor,
        role_b: DynamicColor,
        delta: f64,
        polarity: TonePolarity,
    ) -> Self {
        Self {
            role_a,
            role_b,
            delta,
            polarity,
        }
    }
}
impl _Constraint for _DeltaConstraint {
    fn test_against(&mut self, scheme: &DynamicScheme, tag: &String) -> () {
        let role_a_color = self.role_a.get_hct(scheme);
        let role_b_color = self.role_b.get_hct(scheme);
        let role_a_should_be_lighter = (self.polarity == TonePolarity::Lighter)
            || (self.polarity == TonePolarity::Nearer && !scheme.is_dark)
            || (self.polarity == TonePolarity::Farther && scheme.is_dark);
        /* let _lighterOrDarker = if roleAShouldBeLighter {
            "lighter"
        } else {
            "darker"
        }; */

        let actual_delta = if role_a_should_be_lighter {
            role_a_color.get_tone() - role_b_color.get_tone()
        } else {
            role_b_color.get_tone() - role_a_color.get_tone()
        };
        if actual_delta < self.delta - Self::_DELTA_TOLERANCE {
            panic!(
                r#"tag {}
                'Dynamic scheme $scheme fails delta constraint:\n'
              '${} should be $delta $lighterOrDarker than ${}, '
              'but they have tones ${} and ${}, respectively\n\n'
              'Role A: ${}\n'
              'Role B: ${}\n'
              'Scheme parameters:\n'
              '  Variant: ${:?}\n'
              '  Source color: ${}\n'
              '  Brightness: ${}\n'
              '  Contrast level: ${}\n'
              'Desired delta: ${}\n'
              'Actual delta: ${}"#,
                tag,
                self.role_a.name,
                self.role_b.name,
                role_a_color.get_tone(),
                role_b_color.get_tone(),
                self.role_a.name,
                self.role_b.name,
                scheme.variant,
                scheme.source_color_hct.to_string(),
                if scheme.is_dark { "dark" } else { "light" },
                scheme.contrast_level,
                self.delta,
                actual_delta
            );
        }
    }
}

struct _BackgroundConstraint {
    pub background: DynamicColor,
}
impl _BackgroundConstraint {
    fn _new(background: DynamicColor) -> Self {
        Self { background }
    }
}
impl _Constraint for _BackgroundConstraint {
    fn test_against(&mut self, scheme: &DynamicScheme, tag: &String) -> () {
        let color = self.background.get_hct(scheme);
        if color.get_tone() >= 50.5 && color.get_tone() < 59.5 {
            panic!(
                r#"tag {}\n
                'Dynamic scheme $scheme fails background constraint:\n'
              '${} has tone ${} which is in the '
              'forbidden zone 50.5 <= tone < 59.5\n\n'
              'Background: ${}\n'
              'Scheme parameters:\n'
              '  Variant: ${:?}\n'
              '  Source color: ${}\n'
              '  Brightness: ${}\n'
              '  Contrast level: ${}\n'
              'Actual tone: ${}"#,
                tag,
                self.background.name,
                color.get_tone(),
                self.background.name,
                scheme.variant,
                scheme.source_color_hct.to_string(),
                if scheme.is_dark { "dark" } else { "light" },
                scheme.contrast_level,
                color.get_tone()
            );
        }
    }
}

#[test]
fn scheme_correctness_test() {
    let mut contrast_constraints = [
        _ContrastConstraint::_new(
            MaterialDynamicColors::on_surface(),
            MaterialDynamicColors::surface(),
            ContrastCurve::new(4.5, 7.0, 11.0, 21.0),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::on_surface(),
            MaterialDynamicColors::surface_bright(),
            ContrastCurve::new(4.5, 7.0, 11.0, 21.0),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::primary(),
            MaterialDynamicColors::surface_dim(),
            ContrastCurve::new(3.0, 4.5, 7.0, 7.0),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::primary(),
            MaterialDynamicColors::surface_bright(),
            ContrastCurve::new(3.0, 4.5, 7.0, 7.0),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::secondary(),
            MaterialDynamicColors::surface_dim(),
            ContrastCurve::new(3.0, 4.5, 7.0, 7.0),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::secondary(),
            MaterialDynamicColors::surface_bright(),
            ContrastCurve::new(3.0, 4.5, 7.0, 7.0),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::tertiary(),
            MaterialDynamicColors::surface_dim(),
            ContrastCurve::new(3.0, 4.5, 7.0, 7.0),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::tertiary(),
            MaterialDynamicColors::surface_bright(),
            ContrastCurve::new(3.0, 4.5, 7.0, 7.0),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::error(),
            MaterialDynamicColors::surface_dim(),
            ContrastCurve::new(3.0, 4.5, 7.0, 7.0),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::error(),
            MaterialDynamicColors::surface_bright(),
            ContrastCurve::new(3.0, 4.5, 7.0, 7.0),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::on_surface_variant(),
            MaterialDynamicColors::surface_dim(),
            ContrastCurve::new(3.0, 4.5, 7.0, 11.0),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::on_surface_variant(),
            MaterialDynamicColors::surface_bright(),
            ContrastCurve::new(3.0, 4.5, 7.0, 11.0),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::outline(),
            MaterialDynamicColors::surface_dim(),
            ContrastCurve::new(1.5, 3.0, 4.5, 7.0),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::outline(),
            MaterialDynamicColors::surface_bright(),
            ContrastCurve::new(1.5, 3.0, 4.5, 7.0),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::primary_container(),
            MaterialDynamicColors::surface_dim(),
            ContrastCurve::new(0.0, 0.0, 3.0, 4.5),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::primary_container(),
            MaterialDynamicColors::surface_bright(),
            ContrastCurve::new(0.0, 0.0, 3.0, 4.5),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::primary_fixed(),
            MaterialDynamicColors::surface_dim(),
            ContrastCurve::new(0.0, 0.0, 3.0, 4.5),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::primary_fixed(),
            MaterialDynamicColors::surface_bright(),
            ContrastCurve::new(0.0, 0.0, 3.0, 4.5),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::primary_fixed_dim(),
            MaterialDynamicColors::surface_dim(),
            ContrastCurve::new(0.0, 0.0, 3.0, 4.5),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::primary_fixed_dim(),
            MaterialDynamicColors::surface_bright(),
            ContrastCurve::new(0.0, 0.0, 3.0, 4.5),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::secondary_container(),
            MaterialDynamicColors::surface_dim(),
            ContrastCurve::new(0.0, 0.0, 3.0, 4.5),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::secondary_container(),
            MaterialDynamicColors::surface_bright(),
            ContrastCurve::new(0.0, 0.0, 3.0, 4.5),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::secondary_fixed(),
            MaterialDynamicColors::surface_dim(),
            ContrastCurve::new(0.0, 0.0, 3.0, 4.5),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::secondary_container(),
            MaterialDynamicColors::surface_bright(),
            ContrastCurve::new(0.0, 0.0, 3.0, 4.5),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::secondary_fixed_dim(),
            MaterialDynamicColors::surface_dim(),
            ContrastCurve::new(0.0, 0.0, 3.0, 4.5),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::secondary_fixed_dim(),
            MaterialDynamicColors::surface_bright(),
            ContrastCurve::new(0.0, 0.0, 3.0, 4.5),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::tertiary_container(),
            MaterialDynamicColors::surface_dim(),
            ContrastCurve::new(0.0, 0.0, 3.0, 4.5),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::tertiary_container(),
            MaterialDynamicColors::surface_bright(),
            ContrastCurve::new(0.0, 0.0, 3.0, 4.5),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::tertiary_fixed(),
            MaterialDynamicColors::surface_dim(),
            ContrastCurve::new(0.0, 0.0, 3.0, 4.5),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::tertiary_fixed(),
            MaterialDynamicColors::surface_bright(),
            ContrastCurve::new(0.0, 0.0, 3.0, 4.5),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::tertiary_fixed_dim(),
            MaterialDynamicColors::surface_dim(),
            ContrastCurve::new(0.0, 0.0, 3.0, 4.5),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::tertiary_fixed_dim(),
            MaterialDynamicColors::surface_bright(),
            ContrastCurve::new(0.0, 0.0, 3.0, 4.5),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::error_container(),
            MaterialDynamicColors::surface_dim(),
            ContrastCurve::new(0.0, 0.0, 3.0, 4.5),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::error_container(),
            MaterialDynamicColors::surface_bright(),
            ContrastCurve::new(0.0, 0.0, 3.0, 4.5),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::outline_variant(),
            MaterialDynamicColors::surface_dim(),
            ContrastCurve::new(0.0, 0.0, 3.0, 4.5),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::outline_variant(),
            MaterialDynamicColors::surface_bright(),
            ContrastCurve::new(0.0, 0.0, 3.0, 4.5),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::inverse_on_surface(),
            MaterialDynamicColors::inverse_surface(),
            ContrastCurve::new(4.5, 7.0, 11.0, 21.0),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::inverse_primary(),
            MaterialDynamicColors::inverse_surface(),
            ContrastCurve::new(3.0, 4.5, 7.0, 7.0),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::on_primary(),
            MaterialDynamicColors::primary(),
            ContrastCurve::new(4.5, 7.0, 11.0, 21.0),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::on_secondary(),
            MaterialDynamicColors::secondary(),
            ContrastCurve::new(4.5, 7.0, 11.0, 21.0),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::on_tertiary(),
            MaterialDynamicColors::tertiary(),
            ContrastCurve::new(4.5, 7.0, 11.0, 21.0),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::on_error(),
            MaterialDynamicColors::error(),
            ContrastCurve::new(4.5, 7.0, 11.0, 21.0),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::on_primary_container(),
            MaterialDynamicColors::primary_container(),
            ContrastCurve::new(4.5, 7.0, 11.0, 21.0),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::on_secondary_container(),
            MaterialDynamicColors::secondary_container(),
            ContrastCurve::new(4.5, 7.0, 11.0, 21.0),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::on_tertiary_container(),
            MaterialDynamicColors::tertiary_container(),
            ContrastCurve::new(4.5, 7.0, 11.0, 21.0),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::on_error_container(),
            MaterialDynamicColors::error_container(),
            ContrastCurve::new(4.5, 7.0, 11.0, 21.0),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::on_primary_fixed(),
            MaterialDynamicColors::primary_fixed(),
            ContrastCurve::new(4.5, 7.0, 11.0, 21.0),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::on_primary_fixed(),
            MaterialDynamicColors::primary_fixed_dim(),
            ContrastCurve::new(4.5, 7.0, 11.0, 21.0),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::on_secondary_fixed(),
            MaterialDynamicColors::secondary_fixed(),
            ContrastCurve::new(4.5, 7.0, 11.0, 21.0),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::on_secondary_fixed(),
            MaterialDynamicColors::secondary_fixed_dim(),
            ContrastCurve::new(4.5, 7.0, 11.0, 21.0),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::on_tertiary_fixed(),
            MaterialDynamicColors::tertiary_fixed(),
            ContrastCurve::new(4.5, 7.0, 11.0, 21.0),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::on_tertiary_fixed(),
            MaterialDynamicColors::tertiary_fixed_dim(),
            ContrastCurve::new(4.5, 7.0, 11.0, 21.0),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::on_primary_fixed_variant(),
            MaterialDynamicColors::primary_fixed(),
            ContrastCurve::new(3.0, 4.5, 7.0, 11.0),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::on_primary_fixed_variant(),
            MaterialDynamicColors::primary_fixed_dim(),
            ContrastCurve::new(3.0, 4.5, 7.0, 11.0),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::on_secondary_fixed_variant(),
            MaterialDynamicColors::secondary_fixed(),
            ContrastCurve::new(3.0, 4.5, 7.0, 11.0),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::on_secondary_fixed_variant(),
            MaterialDynamicColors::secondary_fixed_dim(),
            ContrastCurve::new(3.0, 4.5, 7.0, 11.0),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::on_tertiary_fixed_variant(),
            MaterialDynamicColors::tertiary_fixed(),
            ContrastCurve::new(3.0, 4.5, 7.0, 11.0),
        ),
        _ContrastConstraint::_new(
            MaterialDynamicColors::on_tertiary_fixed_variant(),
            MaterialDynamicColors::tertiary_fixed_dim(),
            ContrastCurve::new(3.0, 4.5, 7.0, 11.0),
        ),
    ];
    let mut delta_constraints = [
        _DeltaConstraint::_new(
            MaterialDynamicColors::primary(),
            MaterialDynamicColors::primary_container(),
            10.0,
            TonePolarity::Farther,
        ),
        _DeltaConstraint::_new(
            MaterialDynamicColors::secondary(),
            MaterialDynamicColors::secondary_container(),
            10.0,
            TonePolarity::Farther,
        ),
        _DeltaConstraint::_new(
            MaterialDynamicColors::tertiary(),
            MaterialDynamicColors::tertiary_container(),
            10.0,
            TonePolarity::Farther,
        ),
        _DeltaConstraint::_new(
            MaterialDynamicColors::error(),
            MaterialDynamicColors::error_container(),
            10.0,
            TonePolarity::Farther,
        ),
        _DeltaConstraint::_new(
            MaterialDynamicColors::primary_fixed_dim(),
            MaterialDynamicColors::primary_fixed(),
            10.0,
            TonePolarity::Darker,
        ),
        _DeltaConstraint::_new(
            MaterialDynamicColors::secondary_fixed_dim(),
            MaterialDynamicColors::secondary_fixed(),
            10.0,
            TonePolarity::Darker,
        ),
        _DeltaConstraint::_new(
            MaterialDynamicColors::tertiary_fixed_dim(),
            MaterialDynamicColors::tertiary_fixed(),
            10.0,
            TonePolarity::Darker,
        ),
    ];

    let mut background_constraints = [
        _BackgroundConstraint::_new(MaterialDynamicColors::background()),
        _BackgroundConstraint::_new(MaterialDynamicColors::error()),
        _BackgroundConstraint::_new(MaterialDynamicColors::error_container()),
        _BackgroundConstraint::_new(MaterialDynamicColors::primary()),
        _BackgroundConstraint::_new(MaterialDynamicColors::primary_container()),
        _BackgroundConstraint::_new(MaterialDynamicColors::primary_fixed()),
        _BackgroundConstraint::_new(MaterialDynamicColors::primary_fixed_dim()),
        _BackgroundConstraint::_new(MaterialDynamicColors::secondary()),
        _BackgroundConstraint::_new(MaterialDynamicColors::secondary_container()),
        _BackgroundConstraint::_new(MaterialDynamicColors::secondary_fixed()),
        _BackgroundConstraint::_new(MaterialDynamicColors::secondary_fixed_dim()),
        _BackgroundConstraint::_new(MaterialDynamicColors::surface()),
        _BackgroundConstraint::_new(MaterialDynamicColors::surface_bright()),
        _BackgroundConstraint::_new(MaterialDynamicColors::surface_container()),
        _BackgroundConstraint::_new(MaterialDynamicColors::surface_container_high()),
        _BackgroundConstraint::_new(MaterialDynamicColors::surface_container_highest()),
        _BackgroundConstraint::_new(MaterialDynamicColors::surface_container_low()),
        _BackgroundConstraint::_new(MaterialDynamicColors::surface_container_lowest()),
        _BackgroundConstraint::_new(MaterialDynamicColors::surface_dim()),
        _BackgroundConstraint::_new(MaterialDynamicColors::surface_tint()),
        _BackgroundConstraint::_new(MaterialDynamicColors::surface_variant()),
        _BackgroundConstraint::_new(MaterialDynamicColors::tertiary()),
        _BackgroundConstraint::_new(MaterialDynamicColors::tertiary_container()),
        _BackgroundConstraint::_new(MaterialDynamicColors::tertiary_fixed()),
        _BackgroundConstraint::_new(MaterialDynamicColors::tertiary_fixed_dim()),
    ];
    let variants = [
        Variant::Monochrome,
        Variant::Neutral,
        Variant::TonalSpot,
        Variant::Vibrant,
        Variant::Expressive,
        Variant::Content,
        Variant::Fidelity,
        Variant::Rainbow,
        Variant::FruitSalad,
    ];
    for variant in variants {
        for contrast_level in [-1.0, 0.0, 0.5, 1.0] {
            // For each variant-contrast combination, tests across four
            // seed colors as well as two brightnesses.

            for source_color_argb in [0xFF0000FF, 0xFF00FF00, 0xFFFFFF00, 0xFFFF0000] {
                for is_dark in [false, true] {
                    let s: DynamicScheme = _scheme_from_variant(
                        &variant,
                        &Hct::from_int(source_color_argb),
                        is_dark,
                        contrast_level,
                    );
                    // Ensures all constraints are satisfied.
                    for constraint in &mut contrast_constraints {
                        constraint
                            .test_against(&s, &format!("${:?}, ${}", variant, contrast_level));
                    }
                    for constraint in &mut delta_constraints {
                        constraint
                            .test_against(&s, &format!("${:?}, ${}", variant, contrast_level));
                    }
                    for constraint in &mut background_constraints {
                        constraint
                            .test_against(&s, &format!("${:?}, ${}", variant, contrast_level));
                    }
                }
            }
        }
    }
}

fn _scheme_from_variant(
    variant: &Variant,
    source_color_hct: &Hct,
    is_dark: bool,
    contrast_level: f64,
) -> DynamicScheme {
    let source_color_hct = source_color_hct.clone();
    match variant {
        Variant::Monochrome => {
            return SchemeMonochrome::new(source_color_hct, is_dark, contrast_level).dynamic_scheme;
        }
        Variant::Neutral => {
            return SchemeNeutral::new(source_color_hct, is_dark, contrast_level).dynamic_scheme;
        }
        Variant::TonalSpot => {
            return SchemeTonalSpot::new(source_color_hct, is_dark, contrast_level).dynamic_scheme;
        }
        Variant::Vibrant => {
            return SchemeVibrant::new(source_color_hct, is_dark, contrast_level).dynamic_scheme;
        }
        Variant::Expressive => {
            return SchemeExpressive::new(source_color_hct, is_dark, contrast_level).dynamic_scheme;
        }
        Variant::Content => {
            return SchemeContent::new(source_color_hct, is_dark, contrast_level).dynamic_scheme;
        }
        Variant::Fidelity => {
            return SchemeFidelity::new(source_color_hct, is_dark, contrast_level).dynamic_scheme;
        }
        Variant::Rainbow => {
            return SchemeRainbow::new(source_color_hct, is_dark, contrast_level).dynamic_scheme;
        }
        Variant::FruitSalad => {
            return SchemeFruitSalad::new(source_color_hct, is_dark, contrast_level).dynamic_scheme;
        }
    };
}
