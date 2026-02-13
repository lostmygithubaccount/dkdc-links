//! Graphical interface for dkdc-links

use iced::widget::{center, column, container, text};
use iced::{Element, Theme};

mod colors {
    use iced::Color;

    pub const BG_DARK: Color = Color::from_rgb(0.10, 0.10, 0.16);
    pub const PURPLE_BRIGHT: Color = Color::from_rgb(0.85, 0.20, 1.0);
    pub const TEXT_MUTED: Color = Color::from_rgb(0.55, 0.55, 0.65);
}

struct Links;

#[derive(Debug, Clone, Copy)]
enum Message {}

impl Links {
    fn new() -> (Self, iced::Task<Message>) {
        (Self, iced::Task::none())
    }

    fn update(&mut self, _message: Message) -> iced::Task<Message> {
        iced::Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let title = text("dkdc-links").size(36).color(colors::PURPLE_BRIGHT);
        let subtitle = text("Coming soon...").size(16).color(colors::TEXT_MUTED);

        let content = column![title, subtitle]
            .spacing(12)
            .align_x(iced::Alignment::Center);

        center(container(content).padding(40).style(|_| container::Style {
            background: Some(iced::Background::Color(colors::BG_DARK)),
            ..Default::default()
        }))
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn title(&self) -> String {
        "dkdc-links".into()
    }
}

pub fn run() -> iced::Result {
    iced::application(Links::new, Links::update, Links::view)
        .title(Links::title)
        .theme(Links::theme)
        .antialiasing(true)
        .run()
}
