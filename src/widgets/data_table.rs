use iced::widget::{column, container, row, scrollable, text};
use iced::{Element, Fill, Font, Theme};

use crate::db::QueryResult;

#[derive(Debug, Clone)]
pub enum DataTableMessage {
    ScrollX(f32),
    ScrollY(f32),
}

pub struct DataTable;

impl DataTable {
    pub fn view<'a>(result: &'a QueryResult) -> Element<'a, DataTableMessage> {
        if result.columns.is_empty() {
            if let Some(msg) = &result.message {
                return container(
                    text(msg.clone())
                        .size(14)
                        .style(|_theme: &Theme| text::Style {
                            color: Some(iced::Color::from_rgb(0.5, 0.5, 0.5)),
                        }),
                )
                .padding(20)
                .center_x(Fill)
                .into();
            }
            return container(text("")).into();
        }

        let header_cells: Vec<Element<'_, DataTableMessage>> = result
            .columns
            .iter()
            .map(|col| {
                container(
                    text(col.name.as_str())
                        .font(Font {
                            weight: iced::font::Weight::Bold,
                            ..Font::DEFAULT
                        })
                        .size(13),
                )
                .padding([4, 8])
                .width(150)
                .into()
            })
            .collect();

        let header = row(header_cells).spacing(1);

        let mut rows_widget = column![].spacing(1);

        for row_data in &result.rows {
            let cells: Vec<Element<'_, DataTableMessage>> = row_data
                .iter()
                .map(|val| {
                    let display = match val {
                        crate::db::Value::Null => {
                            text("NULL")
                                .size(12)
                                .style(|_theme: &Theme| text::Style {
                                    color: Some(iced::Color::from_rgb(0.6, 0.6, 0.6)),
                                })
                        }
                        _ => text(val.to_string()).size(12),
                    };
                    container(display)
                        .padding([3, 8])
                        .width(150)
                        .into()
                })
                .collect();

            rows_widget = rows_widget.push(row(cells).spacing(1));
        }

        let content = column![header, rows_widget].spacing(0);

        container(scrollable(content).direction(
            scrollable::Direction::Both {
                vertical: scrollable::Scrollbar::default(),
                horizontal: scrollable::Scrollbar::default(),
            },
        ))
        .padding(4)
        .width(Fill)
        .height(Fill)
        .into()
    }
}
