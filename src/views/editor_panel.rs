use iced::widget::text_editor;
use iced::Element;

use crate::editor::statement;
use crate::theme::DbxPalette;
use crate::widgets::sql_editor::{self, EditorMessage};

#[derive(Debug, Clone)]
pub enum EditorPanelMessage {
    Editor(EditorMessage),
}

#[derive(Debug)]
pub struct EditorState {
    pub content: text_editor::Content,
    pub query_text: String,
    pub cursor_offset: usize,
    pub selected_text: Option<String>,
}

impl Default for EditorState {
    fn default() -> Self {
        Self {
            content: text_editor::Content::with_text("SELECT 1;"),
            query_text: "SELECT 1;".to_string(),
            cursor_offset: 0,
            selected_text: None,
        }
    }
}

impl EditorState {
    pub fn update(&mut self, message: EditorMessage) -> Option<String> {
        match message {
            EditorMessage::Edit(action) => {
                self.content.perform(action);
                self.query_text = self.content.text();
                // Note: iced's text_editor doesn't expose cursor position directly
                // We'll need to track this manually or use a workaround
                None
            }
            EditorMessage::Execute => {
                let query = self.get_executable_query();
                if query.is_empty() {
                    None
                } else {
                    Some(query)
                }
            }
            EditorMessage::Clear => {
                self.content = text_editor::Content::new();
                self.query_text = String::new();
                self.cursor_offset = 0;
                self.selected_text = None;
                None
            }
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.content = text_editor::Content::with_text(text);
        self.query_text = text.to_string();
    }

    /// Get the query to execute based on selection or cursor position
    pub fn get_executable_query(&self) -> String {
        statement::detect_statement_at_cursor(
            &self.query_text,
            self.cursor_offset,
            self.selected_text.as_deref(),
        )
    }

    pub fn view(&self, palette: &DbxPalette) -> Element<'_, EditorPanelMessage> {
        sql_editor::SqlEditor::view(palette, &self.content).map(EditorPanelMessage::Editor)
    }
}
