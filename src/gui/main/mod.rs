pub mod main;
use egui::Color32;

// Our struct used for theme colours
//
pub struct ThemeColours {
    pub primary: Color32,
    pub secondary: Color32,
    pub background_dark: Color32,
    pub text_muted: Color32,
    pub highlight: Color32,
}

// Define some colours
//
//
// Start off with dark theme
//
pub const DARK_THEME: ThemeColours = ThemeColours {
    // Deep blood red
    primary: Color32::from_rgb(140, 0, 0),
    // Rich purple
    secondary: Color32::from_rgb(83, 53, 74),
    // Darker blood red for backgrounds
    background_dark: Color32::from_rgb(20, 0, 0),
    // Desaturated blood red for text
    text_muted: Color32::from_rgb(171, 103, 103),
    // Brighter blood red for highlights
    highlight: Color32::from_rgb(196, 27, 27),
};
