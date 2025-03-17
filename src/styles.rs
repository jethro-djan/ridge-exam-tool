use iced::{
    Element, Fill, Center, Theme, Border, Background, border,
    Color, window, Task,
};
use iced::widget::{
    Column, button, row, text, container, column, Button,
    svg, 
};

pub struct WelcomeButtonStyle; 

impl WelcomeButtonStyle {
    pub const PADDING: f32 = 20.0;

    pub fn style(theme: &Theme, status: button::Status) -> button::Style {
        let palette = theme.extended_palette();

        match status {
            button::Status::Active => button::Style {
                background: Some(Background::Color(Color::TRANSPARENT)),
                text_color: Color::BLACK,
                ..Default::default()
            },
            button::Status::Hovered => button::Style {
                background: Some(Background::Color(Color::TRANSPARENT)),
                text_color: Color::BLACK,
                border: Border { color: Color::BLACK, width: 2.0, radius: border::Radius::new(5.0) },
                ..Default::default()
            },
            button::Status::Pressed => button::Style {
                background: Some(Background::Color(Color::from_rgb(207.0, 236.0, 247.0))),
                text_color: Color::BLACK,
                border: Border { color: Color::BLACK, width: 2.0, radius: border::Radius::new(5.0) },
                ..Default::default()
            },
            _ => button::Style {
                background: Some(Background::Color(Color::TRANSPARENT)),
                text_color: Color::BLACK,
                ..Default::default()
            },
        }
    }
}
