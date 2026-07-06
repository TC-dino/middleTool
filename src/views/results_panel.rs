use iced::Element;

use crate::db::QueryResult;
use crate::theme::DbxPalette;
use crate::widgets::data_table::{self, DataTableMessage};

#[derive(Debug, Clone)]
pub enum ResultsPanelMessage {
    Table(DataTableMessage),
}

#[derive(Debug, Default)]
pub struct ResultsState {
    pub result: Option<QueryResult>,
    pub history: Vec<QueryResult>,
}

impl ResultsState {
    pub fn set_result(&mut self, result: QueryResult) {
        self.result = Some(result.clone());
        self.history.push(result);
    }

    pub fn clear(&mut self) {
        self.result = None;
    }

    pub fn view<'a>(&'a self, palette: &'a DbxPalette) -> Element<'a, ResultsPanelMessage> {
        match &self.result {
            Some(result) => data_table::DataTable::view(palette, result).map(ResultsPanelMessage::Table),
            None => iced::widget::text("Execute a query to see results")
                .size(14)
                .style(|_theme: &iced::Theme| iced::widget::text::Style {
                    color: Some(iced::Color::from_rgb(0.5, 0.5, 0.5)),
                })
                .into(),
        }
    }
}
