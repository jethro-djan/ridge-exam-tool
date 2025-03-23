use iced::widget::{
    Button, Column, button, column, container, row, svg, text,
    Container, text_input
};
use iced::{
    Background, Border, Center, Color, Element, Fill, Length, Rectangle, 
    Shadow, Size, Task, Theme, border, window,
};

pub fn welcome_button<'a, Message: 'a>(
    icon_svg: svg::Svg<'a>,
    text: &'a str,
) -> Button<'a, Message> {
    let text_slice: &str = &text[..];
    let icon = column![icon_svg, text_slice].align_x(Center);

    Button::new(icon)
        .width(250)
        .padding(styles::WELCOME_BTN_PADDING)
        .style(styles::welcome_btn_style)
}

pub fn login_container<'a, Message: 'a>(content: Element<'a, Message>) -> Container<'a, Message> {
    let form_container = Container::new(content)
        .width(Length::Fixed(400.0))
        .padding(50)
        .style(styles::login_box_style);

    container(form_container)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
}

mod styles {
    use super::*;
    pub const WELCOME_BTN_PADDING: f32 = 20.0;

    pub fn login_box_style(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();

        container::Style {
            background: Some(Background::Color(Color::from_rgb(0.95, 0.95, 0.95))),
            border: Border {
                color: Color::from_rgb(0.95, 0.95, 0.95),
                width: 1.0,
                radius: border::Radius::new(8.0),
            },
            ..container::Style::default()
        }
    }

    pub fn welcome_btn_style(theme: &Theme, status: button::Status) -> button::Style {
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
                border: Border {
                    color: Color::BLACK,
                    width: 2.0,
                    radius: border::Radius::new(5.0),
                },
                ..Default::default()
            },
            button::Status::Pressed => button::Style {
                background: Some(Background::Color(Color::from_rgb(207.0, 236.0, 247.0))),
                text_color: Color::BLACK,
                border: Border {
                    color: Color::BLACK,
                    width: 2.0,
                    radius: border::Radius::new(5.0),
                },
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
