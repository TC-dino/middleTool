use iced::Color;

#[derive(Debug, Clone)]
pub struct DbxPalette {
    // Backgrounds (darkest to lightest)
    pub bg_base: Color,        // #0D0D10 — main background
    pub bg_surface: Color,     // #141418 — panels, sidebar
    pub bg_elevated: Color,    // #1A1A20 — cards, popups, dialogs
    pub bg_hover: Color,       // #22222A — hover states
    pub bg_selected: Color,    // #2A2A35 — selected items

    // Text
    pub text_primary: Color,   // #E0E0E8
    pub text_secondary: Color, // #8888A0
    pub text_muted: Color,     // #555568

    // Accent
    pub accent: Color,         // #4A90D9 — primary accent (blue)
    pub accent_hover: Color,   // #5AA0E9

    // Semantic
    pub success: Color,        // #4CAF50
    pub warning: Color,        // #FF9800
    pub error: Color,          // #EF5350

    // Borders
    pub border: Color,         // #2A2A32
    pub border_focus: Color,   // #4A90D9

    // Data table
    pub table_header_bg: Color,   // #18181E
    pub table_row_even: Color,    // transparent / bg_base
    pub table_row_odd: Color,     // #121216 (subtle zebra)
    pub table_null_color: Color,  // #555568
}

impl Default for DbxPalette {
    fn default() -> Self {
        Self {
            // Backgrounds
            bg_base: Color::from_rgb(0.05, 0.05, 0.06),      // #0D0D10
            bg_surface: Color::from_rgb(0.08, 0.08, 0.09),    // #141418
            bg_elevated: Color::from_rgb(0.10, 0.10, 0.13),   // #1A1A20
            bg_hover: Color::from_rgb(0.13, 0.13, 0.16),      // #22222A
            bg_selected: Color::from_rgb(0.16, 0.16, 0.21),   // #2A2A35

            // Text
            text_primary: Color::from_rgb(0.88, 0.88, 0.91),  // #E0E0E8
            text_secondary: Color::from_rgb(0.53, 0.53, 0.63), // #8888A0
            text_muted: Color::from_rgb(0.33, 0.33, 0.41),    // #555568

            // Accent
            accent: Color::from_rgb(0.29, 0.56, 0.85),        // #4A90D9
            accent_hover: Color::from_rgb(0.35, 0.63, 0.91),  // #5AA0E9

            // Semantic
            success: Color::from_rgb(0.30, 0.69, 0.31),       // #4CAF50
            warning: Color::from_rgb(1.00, 0.60, 0.00),       // #FF9800
            error: Color::from_rgb(0.94, 0.33, 0.31),         // #EF5350

            // Borders
            border: Color::from_rgb(0.16, 0.16, 0.20),        // #2A2A32
            border_focus: Color::from_rgb(0.29, 0.56, 0.85),  // #4A90D9

            // Data table
            table_header_bg: Color::from_rgb(0.09, 0.09, 0.12), // #18181E
            table_row_even: Color::from_rgb(0.05, 0.05, 0.06),  // bg_base
            table_row_odd: Color::from_rgb(0.07, 0.07, 0.09),   // #121216
            table_null_color: Color::from_rgb(0.33, 0.33, 0.41), // #555568
        }
    }
}

impl DbxPalette {
    pub fn dark() -> Self {
        Self::default()
    }
}
