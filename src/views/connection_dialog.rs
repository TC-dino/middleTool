use iced::widget::{button, column, container, row, text, text_input, pick_list};
use iced::{Element, Fill};

use crate::db::ConnectionConfig;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DbType {
    SQLite,
    MySQL,
    Postgres,
    Redis,
}

impl std::fmt::Display for DbType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbType::SQLite => write!(f, "SQLite"),
            DbType::MySQL => write!(f, "MySQL"),
            DbType::Postgres => write!(f, "PostgreSQL"),
            DbType::Redis => write!(f, "Redis"),
        }
    }
}

const ALL_DB_TYPES: &[DbType] = &[DbType::SQLite, DbType::MySQL, DbType::Postgres, DbType::Redis];

#[derive(Debug, Clone)]
pub enum DialogMessage {
    NameChanged(String),
    DbTypeChanged(DbType),
    PathChanged(String),
    HostChanged(String),
    PortChanged(String),
    UserChanged(String),
    PasswordChanged(String),
    DatabaseChanged(String),
    SelectFile,
    Test,
    Confirm,
    Cancel,
}

#[derive(Debug)]
pub struct ConnectionDialogState {
    pub visible: bool,
    pub edit_index: Option<usize>,
    pub name: String,
    pub db_type: DbType,
    pub path: String,
    pub host: String,
    pub port: String,
    pub user: String,
    pub password: String,
    pub database: String,
    pub error: Option<String>,
}

impl Default for ConnectionDialogState {
    fn default() -> Self {
        Self {
            visible: false,
            edit_index: None,
            name: String::new(),
            db_type: DbType::SQLite,
            path: String::new(),
            host: "localhost".to_string(),
            port: "3306".to_string(),
            user: String::new(),
            password: String::new(),
            database: String::new(),
            error: None,
        }
    }
}

impl ConnectionDialogState {
    pub fn show_new(&mut self) {
        *self = Self {
            visible: true,
            ..Default::default()
        };
    }

    pub fn update(&mut self, message: &DialogMessage) -> Option<ConnectionConfig> {
        match message {
            DialogMessage::NameChanged(v) => self.name = v.clone(),
            DialogMessage::DbTypeChanged(t) => {
                self.db_type = t.clone();
                self.port = match self.db_type {
                    DbType::MySQL => "3306".to_string(),
                    DbType::Postgres => "5432".to_string(),
                    DbType::Redis => "6379".to_string(),
                    DbType::SQLite => String::new(),
                };
            }
            DialogMessage::PathChanged(v) => self.path = v.clone(),
            DialogMessage::HostChanged(v) => self.host = v.clone(),
            DialogMessage::PortChanged(v) => self.port = v.clone(),
            DialogMessage::UserChanged(v) => self.user = v.clone(),
            DialogMessage::PasswordChanged(v) => self.password = v.clone(),
            DialogMessage::DatabaseChanged(v) => self.database = v.clone(),
            DialogMessage::SelectFile => {
                // File dialog handled externally
            }
            DialogMessage::Test => {
                self.error = Some("Test connection not yet implemented".to_string());
            }
            DialogMessage::Confirm => {
                if self.name.is_empty() {
                    self.error = Some("Name is required".to_string());
                    return None;
                }

                let config = match self.db_type {
                    DbType::SQLite => {
                        if self.path.is_empty() {
                            self.error = Some("Path is required".to_string());
                            return None;
                        }
                        ConnectionConfig::SQLite {
                            name: self.name.clone(),
                            path: self.path.clone(),
                        }
                    }
                    DbType::MySQL => ConnectionConfig::MySQL {
                        name: self.name.clone(),
                        host: self.host.clone(),
                        port: self.port.parse().unwrap_or(3306),
                        user: self.user.clone(),
                        password: self.password.clone(),
                        database: self.database.clone(),
                    },
                    DbType::Postgres => ConnectionConfig::Postgres {
                        name: self.name.clone(),
                        host: self.host.clone(),
                        port: self.port.parse().unwrap_or(5432),
                        user: self.user.clone(),
                        password: self.password.clone(),
                        database: self.database.clone(),
                    },
                    DbType::Redis => ConnectionConfig::Redis {
                        name: self.name.clone(),
                        host: self.host.clone(),
                        port: self.port.parse().unwrap_or(6379),
                        password: if self.password.is_empty() {
                            None
                        } else {
                            Some(self.password.clone())
                        },
                        db: None,
                    },
                };

                self.visible = false;
                return Some(config);
            }
            DialogMessage::Cancel => {
                self.visible = false;
                self.error = None;
            }
        }
        None
    }

    pub fn view(&self) -> Element<'_, DialogMessage> {
        if !self.visible {
            return text("").into();
        }

        let title = text("New Connection")
            .size(18)
            .font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..iced::Font::DEFAULT
            });

        let name_field = row![
            text("Name:").width(80),
            text_input("My Database", &self.name)
                .on_input(DialogMessage::NameChanged)
                .width(Fill),
        ]
        .spacing(8)
        .align_y(iced::Alignment::Center);

        let db_type_field = row![
            text("Type:").width(80),
            pick_list(ALL_DB_TYPES, Some(self.db_type.clone()), DialogMessage::DbTypeChanged)
                .width(Fill),
        ]
        .spacing(8)
        .align_y(iced::Alignment::Center);

        let mut fields = column![title, name_field, db_type_field].spacing(12);

        match self.db_type {
            DbType::SQLite => {
                let path_field = row![
                    text("Path:").width(80),
                    text_input("/path/to/database.db", &self.path)
                        .on_input(DialogMessage::PathChanged)
                        .width(Fill),
                    button(text("Browse").size(13))
                        .on_press(DialogMessage::SelectFile)
                        .padding([4, 8]),
                ]
                .spacing(8)
                .align_y(iced::Alignment::Center);

                fields = fields.push(path_field);
            }
            _ => {
                let host_field = row![
                    text("Host:").width(80),
                    text_input("localhost", &self.host)
                        .on_input(DialogMessage::HostChanged)
                        .width(Fill),
                ]
                .spacing(8)
                .align_y(iced::Alignment::Center);

                let port_field = row![
                    text("Port:").width(80),
                    text_input(&self.port, &self.port)
                        .on_input(DialogMessage::PortChanged)
                        .width(Fill),
                ]
                .spacing(8)
                .align_y(iced::Alignment::Center);

                let user_field = row![
                    text("User:").width(80),
                    text_input("root", &self.user)
                        .on_input(DialogMessage::UserChanged)
                        .width(Fill),
                ]
                .spacing(8)
                .align_y(iced::Alignment::Center);

                let pass_field = row![
                    text("Password:").width(80),
                    text_input("password", &self.password)
                        .on_input(DialogMessage::PasswordChanged)
                        .secure(true)
                        .width(Fill),
                ]
                .spacing(8)
                .align_y(iced::Alignment::Center);

                let db_field = row![
                    text("Database:").width(80),
                    text_input("database", &self.database)
                        .on_input(DialogMessage::DatabaseChanged)
                        .width(Fill),
                ]
                .spacing(8)
                .align_y(iced::Alignment::Center);

                fields = fields.push(host_field);
                fields = fields.push(port_field);
                fields = fields.push(user_field);
                fields = fields.push(pass_field);
                fields = fields.push(db_field);
            }
        }

        // Error message
        if let Some(err) = &self.error {
            fields = fields.push(
                text(err.as_str())
                    .size(12)
                    .style(|_theme: &iced::Theme| iced::widget::text::Style {
                        color: Some(iced::Color::from_rgb(1.0, 0.3, 0.3)),
                    }),
            );
        }

        // Buttons
        let buttons = row![
            button(text("Cancel").size(14))
                .on_press(DialogMessage::Cancel)
                .padding([6, 16]),
            button(text("Connect").size(14))
                .on_press(DialogMessage::Confirm)
                .padding([6, 16]),
        ]
        .spacing(8);

        fields = fields.push(buttons);

        container(
            container(fields.padding(20).spacing(12).width(450))
                .style(|_theme: &iced::Theme| container::Style {
                    background: Some(iced::Color::from_rgb(0.18, 0.18, 0.22).into()),
                    border: iced::Border {
                        width: 1.0,
                        color: iced::Color::from_rgb(0.3, 0.3, 0.35),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .center_x(450),
        )
        .center_x(Fill)
        .center_y(Fill)
        .into()
    }
}
