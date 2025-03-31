use iced::{Element, Task};

use screen::login;
use screen::main_app;

fn main() -> iced::Result {
    iced::application(
        RidgeExamTool::title,
        RidgeExamTool::update,
        RidgeExamTool::view,
    )
    .centered()
    .run_with(move || RidgeExamTool::new())
}

struct RidgeExamTool {
    screen: Screen,
}

#[derive(Debug)]
pub enum Screen {
    LoginView(login::Login),
    MainAppView(main_app::MainApp),
}

#[derive(Clone, Debug)]
pub enum Message {
    OpenLoginView(login::Message),
    OpenMainAppView(main_app::Message),
}

impl RidgeExamTool {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                screen: Screen::LoginView(login::Login::new()),
            },
            Task::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Ridge Examination Tool")
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OpenLoginView(message) => {
                let login = login::Login::new();
                self.screen = Screen::LoginView(login);

                Task::none()
            }
            Message::OpenMainAppView(message) => {
                let main_app = main_app::MainApp::new();
                self.screen = Screen::MainAppView(main_app);

                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        match &self.screen {
            Screen::LoginView(login) => login.view().map(Message::OpenLoginView),
            Screen::MainAppView(main_app) => main_app.view().map(Message::OpenMainAppView),
        }
    }
}

mod screen {
    pub mod sidebar {
        use iced::{Element, Task};
        use iced::widget::text;

        #[derive(Debug, Clone)]
        pub enum Message {
            OpenDashboard,
            CreateExamTTProject,
            CreateInvigilationTTProject,
            CreateExamAnalysisProject,
        }

        #[derive(Clone, Default, Debug)]
        pub struct Sidebar {
            pub minimised: bool,
        }

        impl Sidebar {
            pub fn new() -> Self {
                Self { minimised: false }
            }

            pub fn toggle_visibility(&mut self) {
                self.minimised = !self.minimised
            }

            pub fn update(&mut self, message: Message) -> Task<Message> {
                Task::none()
            }

            fn side_menu<'a>(&self) -> Element<'a, Message> {
                let menu = Menu::list();

                text("Side menu").into()
            }

            pub fn view(&self) -> Element<Message> {
                text("Sidebar").into()
            }
        }

        pub enum Menu {
            Dashboard,
            ExamTTProject,
            InvigilationTTProject,
            ExamAnalysisProject,
        }

        impl Menu {
            fn list() -> Vec<Self> {
                vec![
                    Menu::Dashboard,
                    Menu::ExamTTProject,
                    Menu::InvigilationTTProject,
                    Menu::ExamAnalysisProject,
                ]
            }
        }
    }

    pub mod login {
        use iced::Task;
        use iced::widget::{
            Column, button, column, container, image, text, text_input,
            Container,
        };
        use iced::{Center, Element, Length, Theme, Background, Color, Border, border};

        use crate::RidgeExamTool;
        use crate::Screen;
        use crate::main_app;

        pub const RIDGE_SCHOOL: &str = "Ridge SHS";

        #[derive(Default, Debug, Clone)]
        pub struct Login {
            pub username: String,
            pub password: String,
        }

        #[derive(Debug, Clone)]
        pub enum Message {
            LoginButtonPressed,
            UsernameInputChanged(String),
            PasswordInputChanged(String),
        }

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

        pub fn login_container<'a, Message: 'a>(
            content: Element<'a, Message>,
        ) -> Container<'a, Message> {
            let form_container = Container::new(content)
                .width(Length::Fixed(400.0))
                .padding(50)
                .style(login_box_style);

            container(form_container)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
        }

        fn login_screen<'a>(state: &Login) -> Element<'a, Message> {
            let ridge_logo = image(image::Handle::from_path(format!(
                "{}/assets/images/Ridge_School_Kumasi_Logo.png",
                env!("CARGO_MANIFEST_DIR")
            )))
            .height(60);
            let school_info = column![ridge_logo, text!("{}", RIDGE_SCHOOL).size(35)]
                .align_x(Center)
                .spacing(5);

            let username_field = text_input("Enter Username", &state.username)
                .width(400)
                .line_height(text::LineHeight::Relative(2.0))
                .size(15)
                .on_input(Message::UsernameInputChanged);
            let password_field = text_input("Enter Password", &state.password)
                .width(400)
                .line_height(text::LineHeight::Relative(2.0))
                .size(15)
                .on_input(Message::PasswordInputChanged);
            let login_button = button(text!("Log in").size(15).center())
                .width(250)
                .padding(4)
                .on_press(Message::LoginButtonPressed);

            let login_fields = column![username_field, password_field,].spacing(15);

            let fields_with_button = column![login_fields, login_button,]
                .align_x(Center)
                .spacing(20);

            let login_info = Column::new()
                .push(school_info)
                .push(fields_with_button)
                .align_x(Center)
                .width(Length::Shrink)
                .spacing(50)
                .into();

            login_container(login_info).into()
        }

        impl Login {
            pub fn new() -> Self {
                Self {
                    username: String::new(),
                    password: String::new(),
                }
            }

            pub fn update(&mut self, message: Message) -> Task<Message> {
                match message {
                    Message::UsernameInputChanged(username) => {
                        self.username = username;
                        Task::none()
                    }
                    Message::PasswordInputChanged(password) => {
                        self.password = password;
                        Task::none()
                    }
                    Message::LoginButtonPressed => {
                        let (mut main_state, command) = RidgeExamTool::new();
                        main_state.screen = Screen::MainAppView(main_app::MainApp {});

                        Task::none()
                    }
                }
            }

            pub fn view(&self) -> Element<'_, Message> {
                login_screen(self)
            }
        }

    }

    pub mod main_app {
        use iced::{Task, Element};
        use iced::widget::{text, };

        #[derive(Debug, Clone)]
        pub struct MainApp {}

        #[derive(Debug, Clone)]
        pub enum Message {}

        impl MainApp {
            pub fn new() -> Self {
                Self {}
            }

            pub fn update(&mut self, message: Message) -> Task<Message> {
                Task::none()
            }

            pub fn view(&self) -> Element<'_, Message> {
                text("Main App").into()
            }
        }
    }
}

pub mod icon {
    use iced::widget::{image, image::Image};
    use iced::widget::{svg, svg::Svg};

    pub fn invigilator<'a>() -> Svg<'a> {
        svg(svg::Handle::from_path(format!(
            "{}/assets/images/proctor-schedule.svg",
            env!("CARGO_MANIFEST_DIR")
        )))
        .height(30)
    }

    pub fn exam<'a>() -> Svg<'a> {
        svg(svg::Handle::from_path(format!(
            "{}/assets/images/exam-schedule.svg",
            env!("CARGO_MANIFEST_DIR")
        )))
        .height(30)
    }

    pub fn analysis<'a>() -> Svg<'a> {
        svg(svg::Handle::from_path(format!(
            "{}/assets/images/exam-analysis.svg",
            env!("CARGO_MANIFEST_DIR")
        )))
        .height(30)
    }

    pub fn ridge() -> Image {
        image(image::Handle::from_path(format!(
            "{}/assets/images/Ridge_School_Kumasi_Logo.png",
            env!("CARGO_MANIFEST_DIR")
        )))
        .height(60)
    }
}
