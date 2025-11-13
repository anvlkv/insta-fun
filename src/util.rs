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
