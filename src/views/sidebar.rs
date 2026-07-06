use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Element, Fill, Theme};

use crate::config::AppConfig;
use crate::db::ConnectionConfig;

#[derive(Debug, Clone)]
pub enum SidebarMessage {
    ConnectionSelected(usize),
    Connect(usize),
    Disconnect,
    NewConnection,
    TreeMessage(crate::widgets::db_tree::TreeMessage),
}

#[derive(Debug)]
pub struct SidebarState {
    pub tables: Vec<String>,
    pub selected_table: Option<String>,
    pub expanded: Vec<usize>,
}

impl Default for SidebarState {
    fn default() -> Self {
        Self {
            tables: vec![],
            selected_table: None,
            expanded: vec![],
        }
    }
}

pub fn view_sidebar<'a>(
    config: &AppConfig,
    connected: bool,
    active_index: Option<usize>,
    state: &SidebarState,
) -> Element<'a, SidebarMessage> {
    let mut content = column![].spacing(2);

    // Header
    content = content.push(
        container(
            row![
                text("Connections")
                    .font(iced::Font {
                        weight: iced::font::Weight::Bold,
                        ..iced::Font::DEFAULT
                    })
                    .size(14)
                    .width(Fill),
                button(text("+").size(16))
                    .on_press(SidebarMessage::NewConnection)
                    .padding([0, 6])
                    .style(button::text),
            ]
            .align_y(iced::Alignment::Center),
        )
        .padding([6, 8]),
    );

    // Connection list
    for (i, conn) in config.connections.iter().enumerate() {
        let is_active = active_index == Some(i);
        let name = conn.name();
        let db_type = conn.db_type();

        let icon = match conn {
            ConnectionConfig::SQLite { .. } => "📦",
            ConnectionConfig::MySQL { .. } => "🐬",
            ConnectionConfig::Postgres { .. } => "🐘",
            ConnectionConfig::Redis { .. } => "🔴",
        };

        let label = format!("{} {} [{}]", icon, name, db_type);

        let conn_btn = button(
            text(label)
                .size(13)
                .width(Fill),
        )
        .on_press(if is_active && connected {
            SidebarMessage::Disconnect
        } else {
            SidebarMessage::Connect(i)
        })
        .width(Fill)
        .padding([4, 8])
        .style(if is_active && connected {
            button::primary
        } else {
            button::text
        });

        content = content.push(conn_btn);

        // Show tables under active connection
        if is_active && connected {
            let tree = crate::widgets::db_tree::DbTree::view(
                &state.tables,
                connected,
                state.selected_table.as_deref(),
            )
            .map(SidebarMessage::TreeMessage);

            content = content.push(
                container(tree)
                    .padding(12)
                    .width(Fill),
            );
        }
    }

    if config.connections.is_empty() {
        content = content.push(
            container(
                text("No connections configured")
                    .size(12)
                    .style(|_theme: &Theme| text::Style {
                        color: Some(iced::Color::from_rgb(0.5, 0.5, 0.5)),
                    }),
            )
            .padding(12),
        );
    }

    container(scrollable(content))
        .width(Fill)
        .height(Fill)
        .style(|_theme: &Theme| container::Style {
            background: Some(iced::Color::from_rgb(0.12, 0.12, 0.14).into()),
            ..Default::default()
        })
        .into()
}
