use iced::{
    Element, Fill, Center, Theme, Border, Background, border,
    Color, window, Task, Length, Rectangle, Size,
    Shadow,
};
use iced::widget::{
    Column, button, row, text, container, column, Button,
    svg, 
};

pub fn welcome_button<'a, Message: 'a>(icon_svg: svg::Svg<'a>, text: &'a str) -> Button<'a, Message> {
    let text_slice: &str = &text[..];
    let icon = column! [
        icon_svg,
        text_slice
    ]
    .align_x(Center);

    Button::new(icon)
        .width(250)
        .padding(styles::WELCOME_BTN_PADDING)
        .style(styles::welcome_btn_style)
}


mod styles {
    use super::*;
    pub const WELCOME_BTN_PADDING: f32 = 20.0;

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
// pub struct WelcomeButton<'a> {
//     icon: svg::Svg<'a>,
//     text: String,
// }
// 
// impl WelcomeButton<'_> {
//     pub fn new(icon: svg::Handle, text: String) -> Self {
//         Self { icon, text }
//     }
// }
// 
// pub fn welcome_btn(icon: svg::Handle, text: String) -> WelcomeButton<'static> {
//     WelcomeButton::new(icon, text)
// }
// impl<Message, Renderer> Widget<Message, Theme, Renderer> for WelcomeButton<'_>
// where 
//     Renderer: iced::advanced::Renderer,
// {
//     fn size(&self) -> Size<Length> {
//         Size {
//             width: Length::Shrink,
//             height: Length::Shrink,
//         }
//     }
// 
//     fn layout(
//         &self,
//         _tree: &mut Tree,
//         _renderer: &Renderer,
//         _limits: &layout::Limits,
//     ) -> layout::Node {
//         layout::Node::new(Size::new(100.0, 300.0))
//     }
// 
//     fn draw(
//         &self,
//         _state: &Tree,
//         renderer: &mut Renderer,
//         _theme: &Theme,
//         _style: &renderer::Style,
//         layout: Layout<'_>,
//         _cursor: mouse::Cursor,
//         _viewport: &Rectangle,
//     ) {
//         renderer.fill_quad(
//             Quad {
//                 bounds: layout.bounds(),
//                 border: Border {
//                     color: Color::BLACK,
//                     width: 1.0,
//                     radius: 10.0.into(),
//                 },
//                 shadow: Shadow::default(),
//             },
//             Color::TRANSPARENT,
//         );
//     }
// }
// 
// impl<'a, Message, Renderer> From<WelcomeButton<'_>> for Element<'a, Message, Theme, Renderer>
// where
//     Renderer: iced::advanced::Renderer,
// {
//     fn from (widget: WelcomeButton) -> Self {
//         Self::new(widget)
//     }
// }

