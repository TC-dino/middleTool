use iced::widget::{container, row, text};
use iced::{Element, Fill, Length, Theme};

use crate::theme::DbxPalette;

#[derive(Debug, Clone)]
pub enum SplitterMessage {
    DragStarted,
    DragMoved(f32),
    DragEnded,
}

#[derive(Debug, Clone)]
pub struct SplitterState {
    pub position: f32,
    pub dragging: bool,
    pub min: f32,
    pub max: f32,
}

impl Default for SplitterState {
    fn default() -> Self {
        Self {
            position: 250.0,
            dragging: false,
            min: 150.0,
            max: 500.0,
        }
    }
}

impl SplitterState {
    pub fn update(&mut self, message: SplitterMessage) {
        match message {
            SplitterMessage::DragStarted => {
                self.dragging = true;
            }
            SplitterMessage::DragMoved(delta) => {
                if self.dragging {
                    self.position = (self.position + delta).clamp(self.min, self.max);
                }
            }
            SplitterMessage::DragEnded => {
                self.dragging = false;
            }
        }
    }
}

pub struct Splitter;

impl Splitter {
    pub fn view<'a, Message: Clone + 'a>(
        palette: &'a DbxPalette,
        state: &SplitterState,
        left: Element<'a, Message>,
        right: Element<'a, Message>,
        _on_drag: impl Fn(SplitterMessage) -> Message + 'a,
    ) -> Element<'a, Message> {
        let handle = container(text("").size(1))
            .width(4.0)
            .height(Fill)
            .style(move |_theme: &Theme| container::Style {
                background: Some(palette.border.into()),
                ..Default::default()
            });

        row![
            container(left)
                .width(Length::Fixed(state.position))
                .height(Fill),
            handle,
            container(right)
                .width(Fill)
                .height(Fill),
        ]
        .spacing(0)
        .into()
    }
}
