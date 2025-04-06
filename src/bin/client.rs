use iced::{Element, Task};

use dotenv::dotenv;
use sqlx::PgPool;
use std::env;
use std::sync::Arc;

use screen::dashboard;
use screen::login;

fn main() -> iced::Result {
    iced::application(
        RidgeExamTool::title,
        RidgeExamTool::update,
        RidgeExamTool::view,
    )
    .centered()
    .antialiasing(true)
    .run_with(move || RidgeExamTool::new())
}

struct RidgeExamTool {
    screen: Screen,
    db_pool: Option<Arc<PgPool>>,
    display_message: String,
    current_user: Option<db::User>
}

pub struct LoginView(login::Login);

#[derive(Debug)]
pub enum Screen {
    LoginView(login::Login),
    DashboardView(dashboard::Dashboard),
}
#[derive(Clone, Debug)]
pub enum Message {
    OpenDashboard(dashboard::Message),
    Login(login::Message),
    DatabaseConnected(Result<Arc<PgPool>, db::Error>),
    CreateUsersTable(Result<(), db::Error>),
}

impl RidgeExamTool {
    pub fn new() -> (Self, Task<Message>) {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
            String::from("postgres://postgres:mysecretpassword@localhost:5432/examtool-db")
        });
        (
            Self {
                screen: Screen::LoginView(login::Login::new(None)),
                db_pool: None,
                display_message: String::new(),
                current_user: None,
            },
            Task::perform(
                async move { db::connect(&database_url).await },
                Message::DatabaseConnected,
            ),
        )
    }

    fn title(&self) -> String {
        String::from("Ridge Examination Tool")
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Login(message) => {
                let Screen::LoginView(login) = &mut self.screen else {
                    return Task::none();
                };
                let action = login.update(message);

                match action {
                    login::Action::NoAction => Task::none(),
                    login::Action::Login(Ok(user)) => {
                        self.current_user = Some(user);
                        let (dashboard, task) = dashboard::Dashboard::new();
                        self.screen = Screen::DashboardView(dashboard);

                        task.map(Message::OpenDashboard)
                    }
                    login::Action::Login(Err(_)) => {
                        if let Screen::LoginView(login) = &mut self.screen {
                            self.display_message =
                                String::from("Username or password incorrect. Please try again.");
                            login.inject_display_message(self.display_message.clone());

                            return Task::none();
                        } else {
                            return Task::none();
                        };
                    }
                    login::Action::AsyncTask(_) => Task::none(),
                }
            }
            Message::DatabaseConnected(Ok(pool)) => {
                self.db_pool = Some(pool.clone().into());
                let Screen::LoginView(login) = &mut self.screen else {
                    return Task::none();
                };
                login.db_pool = Some(pool.clone());
                
                Task::perform( 
                    db::create_users_table(pool),
                    Message::CreateUsersTable
                )
            }
            Message::DatabaseConnected(Err(_)) => {
                self.display_message =
                    String::from("Something went wrong. Contact the administrator for help.");
                let Screen::LoginView(login) = &mut self.screen else {
                    return Task::none()
                };
                login.inject_display_message(self.display_message.clone());

                Task::none()
            }
            Message::OpenDashboard(message) => {
                let (dashboard, _) = dashboard::Dashboard::new();
                self.screen = Screen::DashboardView(dashboard);

                Task::none()
            }
            Message::CreateUsersTable(Ok(_)) => Task::none(),
            Message::CreateUsersTable(Err(_)) => {
                if let Screen::LoginView(login) = &mut self.screen {
                    self.display_message =
                        String::from("Something went wrong. Contact the administrator for help.");
                    login.inject_display_message(self.display_message.clone());

                    return Task::none();
                } else {
                    return Task::none();
                };
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        match &self.screen {
            Screen::LoginView(login) => login.view().map(Message::Login),
            Screen::DashboardView(dashboard) => dashboard.view().map(Message::OpenDashboard),
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
        use iced::widget::{Column, Container, button, column, container, row, text, text_input};
        use iced::{
            Background, Border, Center, Color, Element, Length, Theme, border,
            Task,
        };

        use sqlx::postgres::PgPool;
        use std::sync::Arc;

        use crate::db;
        use crate::icon::ridge;

        pub const RIDGE_SCHOOL: &str = "Ridge SHS";

        #[derive(Debug, Clone)]
        pub struct Login {
            pub current_user: Option<db::User>,
            pub username: String,
            pub password: String,
            pub ui_error_message: String,
            pub db_pool: Option<Arc<PgPool>>,
            pub error: Option<Error>,
        }

        #[derive(Debug, Clone)]
        pub enum Message {
            UsernameInputChanged(String),
            PasswordInputChanged(String),
            DisplayMessageChanged(String),
            LoginButtonPressed,
        }

        pub enum Action {
            NoAction,
            Login(Result<db::User, Error>),
            AsyncTask(Task<Action>),
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

        pub async fn handle_login(
            username: String,
            password: String,
            pool: Arc<PgPool>,
        ) -> Result<db::User, Error> {
            if username.is_empty() || password.is_empty() {
                return Err(Error::InvalidCredentials);
            }

            db::login_user(&pool, &username, &password)
                .await
                .map_err(|_| Error::InvalidCredentials)
        }

        impl Login {
            pub fn new(pool: Option<Arc<PgPool>>) -> Self {
                Self {
                    current_user: None,
                    username: String::new(),
                    password: String::new(),
                    ui_error_message: String::new(),
                    db_pool: pool,
                    error: None,
                }
            }

            pub fn inject_display_message(&mut self, message: String) {
                self.ui_error_message = message
            }

            pub fn update(&mut self, message: Message) -> Action {
                match message {
                    Message::UsernameInputChanged(username) => {
                        self.username = username;

                        Action::NoAction
                    }
                    Message::PasswordInputChanged(password) => {
                        self.password = password;

                        Action::NoAction
                    }
                    Message::DisplayMessageChanged(display_msg) => {
                        self.ui_error_message = display_msg;

                        Action::NoAction
                    }
                    Message::LoginButtonPressed => {
                        let username = self.username.clone();
                        let password = self.password.clone();
                        let pool = self.db_pool.clone().unwrap();

                        Action::AsyncTask(Task::perform(
                            async move { 
                                handle_login(username, password, pool).await 
                            }, 
                            Action::Login
                        ))
                    }
                }
            }

            pub fn view(&self) -> Element<'_, Message> {
                let school_info = column![ridge(), text!("{}", RIDGE_SCHOOL).size(35)]
                    .align_x(Center)
                    .spacing(5);

                let username_field = text_input("Enter Username", &self.username)
                    .width(400)
                    .line_height(text::LineHeight::Relative(2.0))
                    .size(15)
                    .on_input(Message::UsernameInputChanged);
                let password_field = text_input("Enter Password", &self.password)
                    .width(400)
                    .line_height(text::LineHeight::Relative(2.0))
                    .size(15)
                    .on_input(Message::PasswordInputChanged);
                let login_button = button(text("Log in").size(15).center())
                    .width(250)
                    .padding(4)
                    .on_press(Message::LoginButtonPressed);

                let login_fields = column![username_field, password_field,].spacing(15);
                let display_message = row![text!("{}", &self.ui_error_message),];

                let fields_with_button = column![login_fields, login_button, display_message,]
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
        }

        #[derive(Clone, Debug)]
        pub enum Error {
            DbError,
            InvalidCredentials,
            PasswordError,
        }
    }

    pub mod dashboard {
        use iced::widget::text;
        use iced::{Element, Task};

        #[derive(Debug, Clone)]
        pub struct Dashboard {}

        #[derive(Debug, Clone)]
        pub enum Message {}

        impl Dashboard {
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

        pub enum Error {}
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
    use argon2::Argon2;
    use argon2::PasswordHash;
    use argon2::PasswordVerifier;
    use sqlx::PgPool;
    use sqlx::postgres::PgPoolOptions;

    use std::sync::Arc;

    #[derive(sqlx::FromRow, Debug, Clone)]
    pub struct User {
        pub id: i32,
        pub username: String,
        pub password_hash: String,
    }

    pub async fn connect(connection_str: &str) -> Result<Arc<PgPool>, Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(connection_str)
            .await
            .map_err(|error| Error::from(error))?;

        Ok(Arc::new(pool))
    }

    pub async fn create_users_table(pool: Arc<PgPool>) -> Result<(), Error> {
        let query = "
            CREATE TABLE IF NOT EXISTS users (
                id SERIAL PRIMARY KEY,
                username VARCHAR(100) NOT NULL UNIQUE,
                password_hash VARCHAR(255) NOT NULL
            )
        ";

        sqlx::query(query)
            .execute(&*pool)
            .await
            .map_err(|_| Error::TableNotCreated)?;

        Ok(())
    }

    pub async fn login_user(pool: &PgPool, username: &str, password: &str) -> Result<User, Error> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, username, password_hash FROM users where username = $1",
        )
            .bind(username)
            .fetch_one(pool)
            .await
            .map_err(|_| Error::InvalidCredentials)?;

        if verify_password(password, &user.password_hash)? {
            Ok(user)
        } else {
            Err(Error::InvalidCredentials)
        }
    }

    fn verify_password(password: &str, hash: &str) -> Result<bool, Error> {
        let parsed_hash = PasswordHash::new(hash).map_err(|_| Error::PasswordError)?;

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    #[derive(Debug, Clone)]
    pub enum Error {
        DbConnectionError,
        PasswordError,
        InvalidCredentials,
        TableNotCreated,
    }

    impl From<sqlx::Error> for Error {
        fn from(error: sqlx::Error) -> Error {
            dbg!(error);

            Error::DbConnectionError
        }
    }
}

// TODO Introduce error codes later
// fn display_message(error_msgs: Error) -> String {
//     match error_msgs {
//         Error::LoginError(db_error) => {
//             match db_error {
//                 db::Error::DbConnectionError =>
//                     String::from("Cannot connect to database. Contact administrator for assistance."),
//                 db::Error::PasswordError => String::from("Error hashing password"),
//                 db::Error::InvalidCredentials => String::from("Username or password incorrect. Please try again."),
//
//             }
//         }
//         Error::DashboardError(dash_error) => {
//         }
//     }
// }

// #[derive(Debug, Clone)]
// pub enum Error {
//     DbError(db::Error),
//     LoginError(login::Error),
// }
