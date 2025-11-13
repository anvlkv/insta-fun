use plotters::style::RGBColor;

pub(crate) const OUTPUT_CHANNEL_COLORS: &[&str] = &[
    "#4285F4", "#EA4335", "#FBBC04", "#34A853", "#FF6D00", "#AB47BC", "#00ACC1", "#7CB342",
    "#9C27B0", "#3F51B5", "#009688", "#8BC34A", "#FFEB3B", "#FF9800", "#795548", "#607D8B",
    "#E91E63", "#673AB7", "#2196F3", "#00BCD4", "#4CAF50", "#CDDC39", "#FFC107", "#FF5722",
    "#9E9E9E", "#03A9F4", "#8D6E63", "#78909C", "#880E4F", "#4A148C", "#0D47A1", "#004D40",
];

pub(crate) const INPUT_CHANNEL_COLORS: &[&str] = &[
    "#B39DDB", "#FFAB91", "#FFF59D", "#A5D6A7", "#FFCC80", "#CE93D8", "#80DEEA", "#C5E1A5",
    "#BA68C8", "#9FA8DA", "#80CBC4", "#DCE775", "#FFF176", "#FFB74D", "#BCAAA4", "#B0BEC5",
    "#F48FB1", "#B39DDB", "#90CAF9", "#80DEEA", "#A5D6A7", "#E6EE9C", "#FFD54F", "#FF8A65",
    "#BDBDBD", "#81D4FA", "#A1887F", "#90A4AE", "#C2185B", "#7B1FA2", "#1976D2", "#00796B",
];

// Helper function to parse hex color strings
pub(crate) fn parse_hex_color(hex: &str) -> RGBColor {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    RGBColor(r, g, b)
}

// Helper function to get a contrasting color for text
pub(crate) fn get_contrasting_color(bg: &RGBColor) -> RGBColor {
    // Calculate relative luminance using WCAG formula
    let r = bg.0 as f32 / 255.0;
    let g = bg.1 as f32 / 255.0;
    let b = bg.2 as f32 / 255.0;

    let r_linear = if r <= 0.03928 {
        r / 12.92
    } else {
        ((r + 0.055) / 1.055).powf(2.4)
    };
    let g_linear = if g <= 0.03928 {
        g / 12.92
    } else {
        ((g + 0.055) / 1.055).powf(2.4)
    };
    let b_linear = if b <= 0.03928 {
        b / 12.92
    } else {
        ((b + 0.055) / 1.055).powf(2.4)
    };

    let luminance = 0.2126 * r_linear + 0.7152 * g_linear + 0.0722 * b_linear;

    // Use white for dark backgrounds, dark gray for light backgrounds
    if luminance < 0.5 {
        RGBColor(255, 255, 255) // White
    } else {
        RGBColor(32, 32, 32) // Dark gray
    }
}

pub(crate) fn time_formatter(sample: usize, sample_rate: f64) -> String {
    let sr = sample_rate;
    if !sr.is_finite() || sr <= 0.0 {
        return "0".to_string();
    }

    let total_micros = ((sample as f64) * 1_000_000.0 / sr).round() as u64;

    let hours = total_micros / 3_600_000_000;
    let minutes = (total_micros % 3_600_000_000) / 60_000_000;
    let seconds = (total_micros % 60_000_000) / 1_000_000;
    let micros = total_micros % 1_000_000;

    let mut frac = if micros > 0 {
        format!(".{:06}", micros)
    } else {
        String::new()
    };
    if !frac.is_empty() {
        while frac.ends_with('0') {
            frac.pop();
        }
        if frac.ends_with('.') {
            frac.pop();
        }
    }

    if hours > 0 {
        if frac.is_empty() {
            format!("{}h {}m {}s", hours, minutes, seconds)
        } else {
            format!("{}h {}m {}{}s", hours, minutes, seconds, frac)
        }
    } else if minutes > 0 {
        if frac.is_empty() {
            format!("{}m {}s", minutes, seconds)
        } else {
            format!("{}m {}{}s", minutes, seconds, frac)
        }
    } else if frac.is_empty() {
        format!("{}s", seconds)
    } else {
        format!("{}{}s", seconds, frac)
    }
}

pub(crate) fn num_x_labels(data_len: usize, sample_rate: f64) -> usize {
    let sr = sample_rate;
    if !sr.is_finite() || sr <= 0.0 || data_len == 0 {
        return 2;
    }
    let duration = (data_len as f64) / sr;
    if !duration.is_finite() || duration <= 0.0 {
        return 2;
    }

    // Aim for about 8 labels using a "nice" 1-2-5 step
    let target_labels = 8usize;
    let rough_step = duration / (target_labels.saturating_sub(1) as f64);
    if !rough_step.is_finite() || rough_step <= 0.0 {
        return 2;
    }

    // Round step to a nice value: 1, 2, or 5 times a power of 10
    let exp = rough_step.log10().floor() as i32;
    let base = 10f64.powi(exp);
    let scaled = rough_step / base;
    let mult = if scaled <= 1.0 {
        1.0
    } else if scaled <= 2.0 {
        2.0
    } else if scaled <= 5.0 {
        5.0
    } else {
        10.0
    };
    let step = base * mult;
    if !step.is_finite() || step <= 0.0 {
        return 2;
    }

    let labels = (duration / step).floor() as usize + 1;

    labels.max(2)
}
