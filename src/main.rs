mod app;
mod config;
mod db;
mod editor;
mod theme;
mod views;
mod widgets;

use app::DbxApp;

pub fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    iced::application(DbxApp::new, DbxApp::update, DbxApp::view)
        .subscription(DbxApp::subscription)
        .window_size(iced::Size::new(1200.0, 800.0))
        .run()
}
