/// *****************************************
/// Color conversion functions
/// *****************************************

/// Converts RGB values to HSV
///
/// # Arguments
/// * `r` - Red component (0-255)
/// * `g` - Green component (0-255)
/// * `b` - Blue component (0-255)
///
/// # Returns
/// Tuple of (hue, saturation, value) where:
/// * hue is in degrees (0-360)
/// * saturation is normalized (0-1)
/// * value is normalized (0-1)
pub fn rgb_to_hsv(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / delta) % 6.0)
    } else if max == g {
        60.0 * (((b - r) / delta) + 2.0)
    } else {
        60.0 * (((r - g) / delta) + 4.0)
    };

    let h = if h < 0.0 { h + 360.0 } else { h };
    let s = if max == 0.0 { 0.0 } else { delta / max };
    let v = max;

    (h, s, v)
}

/// Converts HSV values to RGB
///
/// # Arguments
/// * `h` - Hue in degrees (0-360)
/// * `s` - Saturation normalized (0-1)
/// * `v` - Value normalized (0-1)
///
/// # Returns
/// Tuple of (red, green, blue) where each component is 0-255
pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let c = v * s; // chroma
    let h_prime = h / 60.0;
    let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());

    let (r1, g1, b1) = match h_prime as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        5 => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0),
    };

    let m = v - c;
    let r = ((r1 + m) * 255.0).round().clamp(0.0, 255.0) as u8;
    let g = ((g1 + m) * 255.0).round().clamp(0.0, 255.0) as u8;
    let b = ((b1 + m) * 255.0).round().clamp(0.0, 255.0) as u8;

    (r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Values can be confirmed from: https://www.rapidtables.com/convert/color/rgb-to-hsv.html
    #[test]
    fn test_rgb_to_hsv() {
        let (h, s, v) = rgb_to_hsv(51, 100, 222);
        assert!((h - 222.807).abs() < 0.01, "Hue should be approx 223");
        assert!((s - 0.77).abs() < 0.01, "Saturation should be approx 0.77");
        assert!((v - 0.871).abs() < 0.01, "Value should be approx 0.871");
    }

    #[test]
    fn test_hsv_to_rgb() {
        let (r, g, b) = hsv_to_rgb(222.807, 0.77, 0.871);
        assert_eq!(r, 51);
        assert_eq!(g, 100);
        assert_eq!(b, 222);
    }
}
