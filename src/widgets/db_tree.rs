use iced::widget::{button, column, container, scrollable, text};
use iced::{Element, Fill, Theme};

#[derive(Debug, Clone)]
pub enum TreeMessage {
    TableClicked(String),
    Refresh,
}

pub struct DbTree;

impl DbTree {
    pub fn view<'a>(
        tables: &[String],
        connected: bool,
        selected_table: Option<&str>,
    ) -> Element<'a, TreeMessage> {
        let mut content = column![].spacing(2);

        if !connected {
            content = content.push(
                container(
                    text("No connection")
                        .size(13)
                        .style(|_theme: &Theme| text::Style {
                            color: Some(iced::Color::from_rgb(0.5, 0.5, 0.5)),
                        }),
                )
                .padding(8),
            );
        } else if tables.is_empty() {
            content = content.push(
                container(
                    text("No tables found")
                        .size(13)
                        .style(|_theme: &Theme| text::Style {
                            color: Some(iced::Color::from_rgb(0.5, 0.5, 0.5)),
                        }),
                )
                .padding(8),
            );
        } else {
            let header = container(
                text("Tables")
                    .font(iced::Font {
                        weight: iced::font::Weight::Bold,
                        ..iced::Font::DEFAULT
                    })
                    .size(13),
            )
            .padding([4, 8]);

            content = content.push(header);

            for table in tables {
                let is_selected = Some(table.as_str()) == selected_table;
                let label = if is_selected {
                    text(format!("  {}", table))
                        .size(13)
                        .style(|_theme: &Theme| text::Style {
                            color: Some(iced::Color::from_rgb(0.2, 0.6, 1.0)),
                        })
                } else {
                    text(format!("  {}", table)).size(13)
                };

                let btn = button(label)
                    .on_press(TreeMessage::TableClicked(table.clone()))
                    .padding([2, 4])
                    .width(Fill)
                    .style(if is_selected {
                        button::secondary
                    } else {
                        button::text
                    });

                content = content.push(btn);
            }
        }

        let refresh_btn = button(text("Refresh").size(12))
            .on_press(TreeMessage::Refresh)
            .padding([4, 8]);

        column![scrollable(content).height(Fill), refresh_btn]
            .spacing(4)
            .into()
    }
}
