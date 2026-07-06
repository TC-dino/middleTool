use iced::widget::{container, row, text};
use iced::{Element, Fill, Theme};

use crate::app::Message;
use crate::theme::DbxPalette;

pub struct StatusBar;

impl StatusBar {
    pub fn view(
        palette: &DbxPalette,
        connected: bool,
        connection_name: String,
        message: Option<String>,
        execution_time: Option<std::time::Duration>,
    ) -> Element<'static, Message> {
        let status_icon = if connected { "●" } else { "○" };
        let status_color = if connected {
            palette.success
        } else {
            palette.error
        };

        let status = row![
            text(status_icon.to_string()).style(move |_theme: &Theme| text::Style {
                color: Some(status_color),
            }),
            text(connection_name).size(12),
        ]
        .spacing(4);

        let msg_text = message.unwrap_or_default();
        let time_text = execution_time
            .map(|d| format!(" | {:.2}ms", d.as_secs_f64() * 1000.0))
            .unwrap_or_default();

        let info = row![
            text(msg_text).size(12),
            text(time_text).size(12),
        ]
        .spacing(4);

        container(row![status, info].spacing(16).align_y(iced::Alignment::Center))
            .padding([2, 8])
            .width(Fill)
            .style(|_theme: &Theme| container::Style {
                background: Some(iced::Color::from_rgb(0.15, 0.15, 0.18).into()),
                ..Default::default()
            })
            .into()
    }
}
