use super::color_utils::ColorUtils;

pub struct StringUtils {}
impl StringUtils {
    fn _map_bool_string(value: bool, if_true: String, if_false: String) -> String {
        if value {
            return if_true;
        } else {
            return if_false;
        }
    }

    pub fn hex_from_argb(argb: i64, leading_hash_sign: Option<bool>) -> String {
        let leading_hash_sign = leading_hash_sign.unwrap_or(true);

        let red = ColorUtils::red_from_argb(argb);
        let green = ColorUtils::green_from_argb(argb);
        let blue = ColorUtils::blue_from_argb(argb);

        return format!(
            "{}{}",
            Self::_map_bool_string(leading_hash_sign, "#".to_string(), "".to_string()),
            format!("{:02x}{:02x}{:02x}", red, green, blue).to_uppercase()
        );
    }

    pub fn argb_from_hex(hex: String) -> Option<i64> {
        return i64::from_str_radix(hex.trim_start_matches("#"), 16).ok();
    }
}
