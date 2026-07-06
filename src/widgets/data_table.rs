use iced::widget::{column, container, row, scrollable, text};
use iced::{Element, Fill, Font, Length, Theme};

use crate::db::QueryResult;
use crate::theme::DbxPalette;

#[derive(Debug, Clone)]
pub enum DataTableMessage {
    ScrollX(f32),
    ScrollY(f32),
}

pub struct DataTable;

impl DataTable {
    pub fn view<'a>(palette: &'a DbxPalette, result: &'a QueryResult) -> Element<'a, DataTableMessage> {
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

        // Calculate dynamic column widths based on content
        let col_widths: Vec<f32> = result.columns.iter().enumerate().map(|(i, col)| {
            let header_width = col.name.len() as f32 * 8.0 + 16.0;
            let max_data_width = result.rows.iter()
                .take(100) // Sample first 100 rows for performance
                .filter_map(|row| row.get(i))
                .map(|val| {
                    let display_len = match val {
                        crate::db::Value::Null => 4,
                        _ => val.to_string().len(),
                    };
                    display_len as f32 * 7.5 + 16.0
                })
                .fold(0.0f32, f32::max);
            header_width.max(max_data_width).max(60.0).min(400.0)
        }).collect();

        let header_cells: Vec<Element<'_, DataTableMessage>> = result
            .columns
            .iter()
            .zip(col_widths.iter())
            .map(|(col, &width)| {
                container(
                    text(col.name.as_str())
                        .font(Font {
                            weight: iced::font::Weight::Bold,
                            ..Font::DEFAULT
                        })
                        .size(13),
                )
                .padding([4, 8])
                .width(Length::Fixed(width))
                .into()
            })
            .collect();

        let header = container(row(header_cells).spacing(1))
            .style(move |_theme: &Theme| container::Style {
                background: Some(palette.table_header_bg.into()),
                ..Default::default()
            });

        let mut rows_widget = column![].spacing(0);

        for (row_index, row_data) in result.rows.iter().enumerate() {
            let is_odd = row_index % 2 == 1;
            let cells: Vec<Element<'_, DataTableMessage>> = row_data
                .iter()
                .zip(col_widths.iter())
                .map(|(val, &width)| {
                    let display = match val {
                        crate::db::Value::Null => {
                            text("NULL")
                                .size(12)
                                .style(|_theme: &Theme| text::Style {
                                    color: Some(palette.table_null_color),
                                })
                        }
                        _ => text(val.to_string()).size(12),
                    };
                    container(display)
                        .padding([3, 8])
                        .width(Length::Fixed(width))
                        .into()
                })
                .collect();

            let row_bg = if is_odd {
                palette.table_row_odd
            } else {
                palette.table_row_even
            };

            rows_widget = rows_widget.push(
                container(row(cells).spacing(1))
                    .style(move |_theme: &Theme| container::Style {
                        background: Some(row_bg.into()),
                        ..Default::default()
                    })
            );
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
