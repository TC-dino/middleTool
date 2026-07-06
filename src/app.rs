use iced::keyboard;
use iced::widget::{column, container, row, text};
use iced::{Element, Fill, Subscription, Task};

use crate::config::AppConfig;
use crate::db::connection::{self, DbConnection};
use crate::db::QueryResult;
use crate::views::connection_dialog::{ConnectionDialogState, DialogMessage};
use crate::views::editor_panel::{EditorPanelMessage, EditorState};
use crate::views::results_panel::{ResultsPanelMessage, ResultsState};
use crate::views::sidebar::{self, SidebarMessage, SidebarState};

pub enum DbxApp {
    Loading,
    Loaded(State),
}

impl DbxApp {
    pub fn new() -> (Self, Task<Message>) {
        (
            DbxApp::Loaded(State::default()),
            Task::none(),
        )
    }
}

pub struct State {
    pub config: AppConfig,
    pub active_connection: Option<DbConnection>,
    pub active_connection_index: Option<usize>,
    pub sidebar: SidebarState,
    pub editor: EditorState,
    pub results: ResultsState,
    pub dialog: ConnectionDialogState,
    pub error_message: Option<String>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            config: AppConfig::load(),
            active_connection: None,
            active_connection_index: None,
            sidebar: SidebarState::default(),
            editor: EditorState::default(),
            results: ResultsState::default(),
            dialog: ConnectionDialogState::default(),
            error_message: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    // Sidebar
    Sidebar(SidebarMessage),

    // Editor
    Editor(EditorPanelMessage),

    // Results
    Results(ResultsPanelMessage),

    // Connection dialog
    Dialog(DialogMessage),

    // DB operations
    Connected(Result<DbConnection, String>, usize),
    QueryExecuted(Result<QueryResult, String>),
    TablesLoaded(Result<Vec<String>, String>),

    // Keyboard
    KeyPressed(keyboard::Key, keyboard::Modifiers),

    // File dialog for SQLite
    FileDialogResult(Option<std::path::PathBuf>),
}

impl DbxApp {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match self {
            DbxApp::Loading => Task::none(),
            DbxApp::Loaded(state) => state.update(message),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        match self {
            DbxApp::Loading => container(
                text("Loading...").size(24),
            )
            .center_x(Fill)
            .center_y(Fill)
            .into(),
            DbxApp::Loaded(state) => state.view(),
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        keyboard::listen().filter_map(|event| {
            if let keyboard::Event::KeyPressed { key, modifiers, .. } = event {
                Some(Message::KeyPressed(key, modifiers))
            } else {
                None
            }
        })
    }
}

impl State {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Sidebar(msg) => {
                match msg {
                    SidebarMessage::Connect(index) => {
                        if let Some(config) = self.config.connections.get(index).cloned() {
                            let idx = index;
                            return Task::perform(
                                async move {
                                    let result = connection::connect(&config).await;
                                    (result, idx)
                                },
                                |(result, idx)| Message::Connected(result, idx),
                            );
                        }
                    }
                    SidebarMessage::Disconnect => {
                        if let Some(conn) = &self.active_connection {
                            conn.close();
                        }
                        self.active_connection = None;
                        self.active_connection_index = None;
                        self.sidebar.tables.clear();
                        self.sidebar.selected_table = None;
                        self.results.clear();
                    }
                    SidebarMessage::NewConnection => {
                        self.dialog.show_new();
                    }
                    SidebarMessage::TreeMessage(tree_msg) => {
                        match tree_msg {
                            crate::widgets::db_tree::TreeMessage::TableClicked(name) => {
                                let query = format!("SELECT * FROM \"{}\" LIMIT 100;", name);
                                self.editor.set_text(&query);
                                self.sidebar.selected_table = Some(name);
                                return self.execute_query(&query);
                            }
                            crate::widgets::db_tree::TreeMessage::Refresh => {
                                return self.load_tables();
                            }
                        }
                    }
                    _ => {}
                }
                Task::none()
            }

            Message::Editor(msg) => {
                match msg {
                    EditorPanelMessage::Editor(editor_msg) => {
                        match &editor_msg {
                            crate::widgets::sql_editor::EditorMessage::Execute => {
                                if let Some(query) = self.editor.update(editor_msg) {
                                    return self.execute_query(&query);
                                }
                            }
                            _ => {
                                self.editor.update(editor_msg);
                            }
                        }
                    }
                }
                Task::none()
            }

            Message::Results(_) => Task::none(),

            Message::Dialog(msg) => {
                match &msg {
                    DialogMessage::SelectFile => {
                        return Task::perform(
                            async {
                                let file = rfd::AsyncFileDialog::new()
                                    .set_title("Select SQLite Database")
                                    .add_filter("SQLite", &["db", "sqlite", "sqlite3"])
                                    .add_filter("All Files", &["*"])
                                    .pick_file()
                                    .await;
                                file.map(|f| f.path().to_path_buf())
                            },
                            Message::FileDialogResult,
                        );
                    }
                    _ => {}
                }

                if let Some(config) = self.dialog.update(&msg) {
                    self.config.add_connection(config);
                }
                Task::none()
            }

            Message::Connected(result, index) => {
                match result {
                    Ok(conn) => {
                        self.active_connection = Some(conn);
                        self.active_connection_index = Some(index);
                        self.error_message = None;
                        return self.load_tables();
                    }
                    Err(err) => {
                        self.error_message = Some(err);
                    }
                }
                Task::none()
            }

            Message::QueryExecuted(result) => {
                match result {
                    Ok(query_result) => {
                        self.results.set_result(query_result);
                        self.error_message = None;
                    }
                    Err(err) => {
                        self.error_message = Some(err);
                    }
                }
                Task::none()
            }

            Message::TablesLoaded(result) => {
                match result {
                    Ok(tables) => {
                        self.sidebar.tables = tables;
                        self.error_message = None;
                    }
                    Err(err) => {
                        self.error_message = Some(err);
                    }
                }
                Task::none()
            }

            Message::KeyPressed(key, modifiers) => {
                use keyboard::key::{Key, Named};

                // Ctrl+Enter -> Execute query
                if modifiers.command() {
                    if let Key::Named(Named::Enter) = key {
                        if let Some(query) = self.editor.update(
                            crate::widgets::sql_editor::EditorMessage::Execute,
                        ) {
                            return self.execute_query(&query);
                        }
                    }
                }

                Task::none()
            }

            Message::FileDialogResult(path) => {
                if let Some(path) = path {
                    self.dialog.update(&DialogMessage::PathChanged(
                        path.to_string_lossy().to_string(),
                    ));
                }
                Task::none()
            }
        }
    }

    fn execute_query(&self, query: &str) -> Task<Message> {
        if let Some(conn) = &self.active_connection {
            let conn = conn.clone();
            let query = query.to_string();
            Task::perform(
                async move { Message::QueryExecuted(conn.execute(&query).await) },
                |msg| msg,
            )
        } else {
            Task::none()
        }
    }

    fn load_tables(&self) -> Task<Message> {
        if let Some(conn) = &self.active_connection {
            let conn = conn.clone();
            Task::perform(
                async move { Message::TablesLoaded(conn.list_tables().await) },
                |msg| msg,
            )
        } else {
            Task::none()
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let connection_name = self
            .active_connection_index
            .and_then(|i| self.config.connections.get(i))
            .map(|c| format!("{} [{}]", c.name(), c.db_type()))
            .unwrap_or_else(|| "Not connected".to_string());

        // Dialog overlay
        if self.dialog.visible {
            let dialog = self.dialog.view().map(Message::Dialog);
            return container(dialog)
                .center_x(Fill)
                .center_y(Fill)
                .into();
        }

        // Sidebar
        let sidebar_view = sidebar::view_sidebar(
            &self.config,
            self.active_connection.is_some(),
            self.active_connection_index,
            &self.sidebar,
        )
        .map(Message::Sidebar);

        // Editor
        let editor_view = self.editor.view().map(Message::Editor);

        // Results
        let results_view = self.results.view().map(Message::Results);

        // Right panel: editor + results split
        let right_panel = column![
            container(editor_view)
                .height(iced::Length::FillPortion(2))
                .width(Fill),
            container(text("").size(1))
                .height(2)
                .style(|_theme: &iced::Theme| container::Style {
                    background: Some(iced::Color::from_rgb(0.3, 0.3, 0.35).into()),
                    ..Default::default()
                }),
            container(
                column![
                    container(
                        text("Results")
                            .size(13)
                            .font(iced::Font {
                                weight: iced::font::Weight::Bold,
                                ..iced::Font::DEFAULT
                            })
                    )
                    .padding([4, 8]),
                    results_view,
                ]
            )
            .height(iced::Length::FillPortion(3))
            .width(Fill),
        ]
        .spacing(0);

        // Error bar
        let error_bar = if let Some(err) = &self.error_message {
            container(
                text(err.as_str())
                    .size(12)
                    .style(|_theme: &iced::Theme| text::Style {
                        color: Some(iced::Color::from_rgb(1.0, 0.3, 0.3)),
                    }),
            )
            .padding([2, 8])
            .width(Fill)
        } else {
            container(text(""))
        };

        // Status bar
        let status = crate::widgets::status_bar::StatusBar::view(
            self.active_connection.is_some(),
            connection_name.clone(),
            self.results.result.as_ref().and_then(|r| r.message.clone()),
            self.results.result.as_ref().map(|r| r.execution_time),
        );

        // Main layout
        let main_content = row![
            container(sidebar_view)
                .width(250)
                .height(Fill),
            right_panel,
        ]
        .spacing(0);

        column![
            container(main_content).height(Fill),
            error_bar,
            status,
        ]
        .into()
    }
}
