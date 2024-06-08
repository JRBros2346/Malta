use iced::Application;

mod app;
mod db;

fn main() -> iced::Result {
    app::App::run(iced::Settings::with_flags(()))
}
