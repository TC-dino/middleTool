use iced::widget::{button, container, text};
use iced::Theme;

use super::palette::DbxPalette;

pub fn sidebar_container(palette: &DbxPalette) -> impl Fn(&Theme) -> container::Style + '_ {
    move |_theme: &Theme| container::Style {
        background: Some(palette.bg_surface.into()),
        ..Default::default()
    }
}

pub fn dialog_container(palette: &DbxPalette) -> impl Fn(&Theme) -> container::Style + '_ {
    move |_theme: &Theme| container::Style {
        background: Some(palette.bg_elevated.into()),
        border: iced::Border {
            color: palette.border,
            width: 1.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    }
}

pub fn status_bar_container(palette: &DbxPalette) -> impl Fn(&Theme) -> container::Style + '_ {
    move |_theme: &Theme| container::Style {
        background: Some(palette.bg_surface.into()),
        ..Default::default()
    }
}

pub fn table_header_cell(palette: &DbxPalette) -> impl Fn(&Theme) -> container::Style + '_ {
    move |_theme: &Theme| container::Style {
        background: Some(palette.table_header_bg.into()),
        ..Default::default()
    }
}

pub fn table_row_cell(palette: &DbxPalette, odd: bool) -> impl Fn(&Theme) -> container::Style + '_ {
    move |_theme: &Theme| container::Style {
        background: Some(if odd {
            palette.table_row_odd.into()
        } else {
            palette.table_row_even.into()
        }),
        ..Default::default()
    }
}

pub fn primary_button(palette: &DbxPalette) -> impl Fn(&Theme, button::Status) -> button::Style + '_ {
    move |_theme: &Theme, status: button::Status| {
        let bg = match status {
            button::Status::Hovered => palette.accent_hover,
            _ => palette.accent,
        };
        button::Style {
            background: Some(bg.into()),
            text_color: palette.text_primary,
            border: iced::Border {
                radius: 4.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

pub fn text_button(palette: &DbxPalette) -> impl Fn(&Theme, button::Status) -> button::Style + '_ {
    move |_theme: &Theme, status: button::Status| {
        let bg = match status {
            button::Status::Hovered => palette.bg_hover,
            _ => iced::Color::TRANSPARENT,
        };
        button::Style {
            background: Some(bg.into()),
            text_color: palette.text_primary,
            ..Default::default()
        }
    }
}

pub fn error_text(palette: &DbxPalette) -> impl Fn(&Theme) -> text::Style + '_ {
    move |_theme: &Theme| text::Style {
        color: Some(palette.error),
    }
}

pub fn secondary_text(palette: &DbxPalette) -> impl Fn(&Theme) -> text::Style + '_ {
    move |_theme: &Theme| text::Style {
        color: Some(palette.text_secondary),
    }
}

pub fn muted_text(palette: &DbxPalette) -> impl Fn(&Theme) -> text::Style + '_ {
    move |_theme: &Theme| text::Style {
        color: Some(palette.text_muted),
    }
}
