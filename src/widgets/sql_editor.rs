use iced::widget::{button, column, row, text, text_editor};
use iced::{Element, Length};

#[derive(Debug, Clone)]
pub enum EditorMessage {
    Edit(text_editor::Action),
    Execute,
    Clear,
}

pub struct SqlEditor;

impl SqlEditor {
    pub fn view<'a>(content: &'a text_editor::Content) -> Element<'a, EditorMessage> {
        let editor = text_editor(content)
            .on_action(EditorMessage::Edit)
            .height(Length::Fill)
            .padding(8);

        let toolbar = row![
            button(
                text("Execute").size(14)
            )
            .on_press(EditorMessage::Execute)
            .padding([4, 12]),
            button(
                text("Clear").size(14)
            )
            .on_press(EditorMessage::Clear)
            .padding([4, 12]),
            text("Ctrl+Enter to execute").size(12).style(text::secondary),
        ]
        .spacing(8)
        .align_y(iced::Alignment::Center);

        column![toolbar, editor]
            .spacing(4)
            .into()
    }
}
