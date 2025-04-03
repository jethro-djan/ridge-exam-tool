use iced::{Element, Task};

use sqlx::postgres::PgPool;
use std::sync::Arc;
use std::env;
use dotenv::dotenv;

use screen::login;
use screen::main_app;

fn main() -> iced::Result {
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let handle = rt.handle().clone();

    iced::application(
        RidgeExamTool::title,
        RidgeExamTool::update,
        RidgeExamTool::view,
    )
    .centered()
    .antialiasing(true)
    .run_with(|| (RidgeExamTool::new(handle)))
}

struct RidgeExamTool {
    screen: Screen,
    db_pool: Option<Arc<PgPool>>,
    error: Option<Error>,
    runtime_handle: tokio::runtime::Handle,
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
    OpenDatabase(Result<PgPool, Error>),
}

impl RidgeExamTool {
    pub fn new(handle: tokio::runtime::Handle) -> (Self, Task<Message>) {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:mysecretpassword@localhost:5432/examtool-db");
        (
            Self {
                screen: Screen::LoginView(login::Login::new()),
                db_pool: None,
                error: None,
                runtime_handle: handle.clone(),
            },
            Task::perform(
                async move {
                    handle.block_on(db::connect(&database_url))
                },
                Message::OpenDatabase,
            ),
        )
    }

    fn title(&self) -> String {
        String::from("Ridge Examination Tool")
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OpenLoginView(message) => {
                let mut login = login::Login::new();
                self.screen = Screen::LoginView(login.clone());

                let action = login.update(message);

                match action {
                    login::Action::None => Task::none(),
                    login::Action::Login => {
                        let (main_app, task) = main_app::MainApp::new();

                        self.screen = Screen::MainAppView(main_app);

                        task.map(Message::OpenMainAppView)
                    }
                }
            }
            Message::OpenMainAppView(message) => {
                let (main_app, task) = main_app::MainApp::new();

                self.screen = Screen::MainAppView(main_app);

                Task::none()
            }
            Message::OpenDatabase(Ok(pool)) => {
                self.db_pool = Some(pool.into());

                Task::none()
            } 
            Message::OpenDatabase(Err(_)) => Task::none(),
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
        use iced::widget::text;
        use iced::{Element, Task};

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
        use iced::widget::{Column, Container, button, column, container, text, text_input};
        use iced::{Background, Border, Center, Color, Element, Length, Theme, border};

        use crate::icon::ridge;

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

        pub enum Action {
            None,
            Login,
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
            // let ridge_logo = image(image::Handle::from_path(format!(
            //     "{}/assets/images/Ridge_School_Kumasi_Logo.png",
            //     env!("CARGO_MANIFEST_DIR")
            // )))
            // .height(60);
            let school_info = column![ridge(), text!("{}", RIDGE_SCHOOL).size(35)]
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

            pub fn update(&mut self, message: Message) -> Action {
                match message {
                    Message::UsernameInputChanged(username) => {
                        self.username = username;

                        Action::None
                    }
                    Message::PasswordInputChanged(password) => {
                        self.password = password;

                        Action::None
                    }
                    Message::LoginButtonPressed => Action::Login,
                }
            }

            pub fn view(&self) -> Element<'_, Message> {
                login_screen(self)
            }
        }
    }

    pub mod main_app {
        use iced::widget::text;
        use iced::{Element, Task};

        #[derive(Debug, Clone)]
        pub struct MainApp {}

        #[derive(Debug, Clone)]
        pub enum Message {}

        impl MainApp {
            pub fn new() -> (Self, Task<Message>) {
                (Self {}, Task::none())
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
        .height(80)
    }
}

pub mod db {
    use tokio::runtime::Runtime;
    use sqlx::postgres::PgPoolOptions;
    use sqlx::PgPool;

    use std::sync::Arc;
    use std::env;

    use crate::Error;

    pub async fn connect(connection_str: &str) -> Result<PgPool, Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(connection_str)
            .await
            .map_err(|error| Error::from(error))?;

        Ok(pool)

    }
}

#[derive(Debug, Clone)]
pub enum Error {
    DbError,
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Error {
        dbg!(error);

        Error::DbError
    }
}
