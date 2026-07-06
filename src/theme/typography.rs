use iced::Font;

pub struct Typography;

impl Typography {
    pub const MONOSPACE: Font = Font {
        family: iced::font::Family::Monospace,
        ..Font::DEFAULT
    };

    pub const DEFAULT: Font = Font::DEFAULT;

    pub const BOLD: Font = Font {
        weight: iced::font::Weight::Bold,
        ..Font::DEFAULT
    };

    pub const SEMIBOLD: Font = Font {
        weight: iced::font::Weight::Semibold,
        ..Font::DEFAULT
    };
}
