use iced::widget::text_editor;
use iced::Element;

use crate::widgets::sql_editor::{self, EditorMessage};

#[derive(Debug, Clone)]
pub enum EditorPanelMessage {
    Editor(EditorMessage),
}

#[derive(Debug)]
pub struct EditorState {
    pub content: text_editor::Content,
    pub query_text: String,
}

impl Default for EditorState {
    fn default() -> Self {
        Self {
            content: text_editor::Content::with_text("SELECT 1;"),
            query_text: "SELECT 1;".to_string(),
        }
    }
}

impl EditorState {
    pub fn update(&mut self, message: EditorMessage) -> Option<String> {
        match message {
            EditorMessage::Edit(action) => {
                self.content.perform(action);
                self.query_text = self.content.text();
                None
            }
            EditorMessage::Execute => {
                let query = self.query_text.trim().to_string();
                if query.is_empty() {
                    None
                } else {
                    Some(query)
                }
            }
            EditorMessage::Clear => {
                self.content = text_editor::Content::new();
                self.query_text = String::new();
                None
            }
        }
    }

    pub fn set_text(&mut self, text: &str) {
        self.content = text_editor::Content::with_text(text);
        self.query_text = text.to_string();
    }

    pub fn view(&self) -> Element<'_, EditorPanelMessage> {
        sql_editor::SqlEditor::view(&self.content).map(EditorPanelMessage::Editor)
    }
}
