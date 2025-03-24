use iced::{Element, Length, Task, widget};

use screen::{summaryboard, login, invigilator_tt, exam_tt, exam_analysis};

fn main() -> iced::Result {
    iced::application("Ridge Examination Tool", RidgeExamTool::update, RidgeExamTool::view) 
        .centered()
        .run()
}

#[derive(Default, Debug)]
struct RidgeExamTool {
    screen: Screen,
}

#[derive(Debug)]
enum Screen {
    LoginInterface(login::Login),
    SummaryboardInterface(summaryboard::Summaryboard),
    // InvigilatorTTInterface(invigilator_tt::InvigilatorTT),
    // ExamTTInterface(exam_tt::ExamTT),
    // ExamAnalysisInterface(exam_analysis::ExamAnalysis),
}

impl Default for Screen {
    fn default() -> Self {
        Screen::LoginInterface(login::Login {
            username: String::new(),
            password: String::new(),
        })
    }
}

#[derive(Clone, Debug)]
enum Message {
    GoToLoginInterface(login::Message),
    GoToSummaryboardInterface(summaryboard::Message),
    // GoToInvigilatorTTInterface(invigilator_tt::Message),
    // GoToExamTTInterface(exam_tt::Message),
    // GoToExamAnalysisInterface(exam_analysis::Message),
}


impl RidgeExamTool {
    pub fn new() -> (RidgeExamTool, Task<Message>) {
        (
            RidgeExamTool {
                screen: Screen::LoginInterface(login::Login {
                    username: String::new(), 
                    password: String::new(),
                })
            },
            Task::none()
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::GoToLoginInterface(message) => {
                self.screen = Screen::SummaryboardInterface(summaryboard::Summaryboard);
                Task::none()
            },
            Message::GoToSummaryboardInterface(message) => match message {
                summaryboard::Message::ExamTTButtonPressed => Task::none(),
                summaryboard::Message::InvigilatorTTButtonPressed => {
                    let Screen::InvigilatorTTInterface(invigilator_tt) = &mut self.screen else {
                        return Task::none();
                    };
                    Task::none()
                }
                summaryboard::Message::ExamAnalysisButtonPressed => Task::none(),
            },
            // Message::GoToInvigilatorTTInterface(message) => Task::none(),
            // Message::GoToExamTTInterface(message) => Task::none(),
            // Message::GoToExamAnalysisInterface(message) => Task::none(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        match &self.screen {
            Screen::LoginInterface(login) => login.view().map(Message::GoToLoginInterface),
            Screen::SummaryboardInterface(summaryboard) => 
                summaryboard.view().map(Message::GoToSummaryboardInterface),
            // Screen::InvigilatorTTInterface(invigilator_tt) => 
            //     invigilator_tt.view().map(Message::GoToInvigilatorTTInterface),
            // Screen::ExamTTInterface(exam_tt) => exam_tt.view().map(Message::GoToExamTTInterface),
            // Screen::ExamAnalysisInterface(exam_analysis) =>
            //     exam_analysis.view().map(Message::GoToExamAnalysisInterface)
        }
    }
}

mod screen {
    pub mod sidebar {
        #[derive(Debug, Clone)]
        pub enum Message {
            OpenDashboard,
            CreateExamTTProject,
            CreateInvigilationTTProject,
            CreateExamAnalysisProject,
        }

        #[derive(Default)]
        pub struct Sidebar {
            pub minimised: bool,
        }

        impl Sidebar {
            pub fn new() -> Self {
                minimised: false,
            }
        }

        pub fn toggle_visibility(&mut self) {
            self.minimised = !self.minimised
        }
        
        pub fn update(&mut self, message: Message) -> Task<Message> {
            Task::none()
        }

        fn side_menu<'a>(&self) -> Element<'a, Message> {
            let menu = Menu::list();
        }

        pub fn view(&self) -> Element<Message> {
        }

        pub enum Menu {
            Dashboard,
            ExamTTProject,
            InvigilationTTProject,
            ExamAnalysisProject,

        }

        impl Menu {
            fn list() -> Vec<&Self> {
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
            Column, button, column, container, 
            image, row, text, text_input, center,
        };
        use iced::{Center, Element, Fill, Length};

        use ridge_exam_tool::widgets::login_container;
        use super::summaryboard;
        use crate::Screen;
        use crate::RidgeExamTool;

        pub const RIDGE_SCHOOL: &str = "Ridge SHS";

        #[derive(Default, Debug, Clone)]
        pub struct Login {
            pub username: String,
            pub password: String,
        }

        #[derive(Debug, Clone)]
        pub enum Message {
            LoginButtonPressed,
            UsernameInputFilled(String),
            PasswordInputFilled(String),
        }

        fn login_screen<'a>(state: &Login) -> Element<'a, Message> {
            let ridge_logo = image(image::Handle::from_path(format!(
                "{}/assets/images/Ridge_School_Kumasi_Logo.png",
                env!("CARGO_MANIFEST_DIR")
            )))
            .height(60);
            let school_info = column![
                ridge_logo, 
                text!("{}", RIDGE_SCHOOL).size(35)
            ]
            .align_x(Center)
            .spacing(5);

            let username_field = text_input("Enter Username", &state.username)
                .width(400)
                .line_height(text::LineHeight::Relative(2.0))
                .size(15)
                .on_input(Message::UsernameInputFilled);
            let password_field = text_input("Enter Password", &state.password)
                .width(400)
                .line_height(text::LineHeight::Relative(2.0))
                .size(15)
                .on_input(Message::PasswordInputFilled);
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

            pub fn update(state: &mut Login, message: Message) -> Task<Message> {
                match message {
                    Message::UsernameInputFilled(username) => {
                        state.username = username;
                        Task::none()
                    }
                    Message::PasswordInputFilled(password) => {
                        state.password = password;
                        Task::none()
                    }
                    Message::LoginButtonPressed => {
                        let (mut main_state, command) = RidgeExamTool::new();
                        main_state.screen = Screen::SummaryboardInterface(summaryboard::Summaryboard);
                        Task::none()
                    }
                }
            }

            pub fn view(&self) -> Element<'_, Message> {
                login_screen(self)
            }
        }
    }

    pub mod exam_analysis {
        use iced::widget::{Button, Column, button, column, container, row, svg, text};
        use iced::{Background, Border, Center, Color, Element, Fill, Task, Theme, border, window};
        use ridge_exam_tool::domain;
        use ridge_exam_tool::widgets::welcome_button;
        // use super::welcome::Model;

        #[derive(Debug, Clone)]
        pub struct ExamAnalysis;

        impl ExamAnalysis {
            pub fn view(&self) -> Element<'_, Message> {
                text!("Exam Analysis").into()
            }
        }

        #[derive(Debug, Clone)]
        pub enum Message {
            SaveStudentPerformanceSummary,
        }
    }

    pub mod exam_tt {
        use iced::widget::{Button, Column, button, column, container, row, svg, text};
        use iced::{Background, Border, Center, Color, Element, Fill, Task, Theme, border, window};
        use ridge_exam_tool::domain;
        use ridge_exam_tool::widgets::welcome_button;

        #[derive(Debug, Clone)]
        pub struct ExamTT;

        impl ExamTT {
            pub fn view(&self) -> Element<'_, Message> {
                text!("Exam TT").into()
            }
        }

        #[derive(Debug, Clone)]
        pub enum Message {
            CreateStudentCombination,
        }
    }

    pub mod summaryboard {
        use crate::{
            Screen, exam_analysis, exam_analysis::ExamAnalysis, exam_tt, exam_tt::ExamTT,
            invigilator_tt, invigilator_tt::InvigilatorTT,
        };
        use iced::widget::{Button, Column, button, column, container, row, svg, text};
        use iced::{
            Background, Border, Center, Color, Element, Fill, Length, Task, Theme, border, window,
        };
        use ridge_exam_tool::domain;
        use ridge_exam_tool::widgets::welcome_button;

        #[derive(Debug, Clone)]
        pub enum Message {
            InvigilatorTTButtonPressed,
            ExamTTButtonPressed,
            ExamAnalysisButtonPressed,
        }

        #[derive(Debug)]
        pub struct Summaryboard {
            side_menu: Sidebar,
        }

        impl Summaryboard {
            pub fn update(&self, message: Message) -> Task<Message> {
                match message {
                    Message::InvigilatorTTButtonPressed => {
                        Screen::InvigilatorTTInterface(InvigilatorTT);
                        Task::none()
                    }
                    Message::ExamTTButtonPressed => Task::none(),
                    Message::ExamAnalysisButtonPressed => Task::none(),
                }
            }

            pub fn view(&self) -> Element<'_, Message> {
                let exam_tt_svg = svg(svg::Handle::from_path(format!(
                    "{}/assets/images/proctor-schedule.svg",
                    env!("CARGO_MANIFEST_DIR")
                )))
                .height(60);
                let invigilation_tt_svg = svg(svg::Handle::from_path(format!(
                    "{}/assets/images/exam-schedule.svg",
                    env!("CARGO_MANIFEST_DIR")
                )))
                .height(60);
                let exam_analysis_svg = svg(svg::Handle::from_path(format!(
                    "{}/assets/images/exam-analysis.svg",
                    env!("CARGO_MANIFEST_DIR")
                )))
                .height(60);

                let welcome_text = text!("Welcome!");
                let welcome_label: Element<_> = Column::new()
                    .push(welcome_text.size(30))
                    .spacing(10)
                    .align_x(Center)
                    .into();

                let invigilator_tt_btn =
                    welcome_button(invigilation_tt_svg, "Create Invigilation Timetable")
                        .on_press(Message::InvigilatorTTButtonPressed);
                let exam_tt_btn = welcome_button(exam_tt_svg, "Create Examination Timetable")
                    .on_press(Message::ExamTTButtonPressed);
                let exam_analysis_btn =
                    welcome_button(exam_analysis_svg, "Perform Examination Analysis")
                        .on_press(Message::ExamAnalysisButtonPressed);

                let btn_row = row![invigilator_tt_btn, exam_tt_btn, exam_analysis_btn,].spacing(40);

                let welcome_screen_elements: Element<_> = Column::new()
                    .padding(20)
                    .spacing(100)
                    .align_x(Center)
                    .push(welcome_label)
                    .push(btn_row)
                    .into();

                container(welcome_screen_elements)
                    .padding(20)
                    .center_x(Fill)
                    .center_y(Fill)
                    .into()
            }
        }
    }

    pub mod invigilator_tt {
        use iced::widget::{Button, Column, button, column, container, row, svg, text};
        use iced::{Background, Border, Center, Color, Element, Fill, Task, Theme, border, window};
        use ridge_exam_tool::domain;
        use ridge_exam_tool::widgets::welcome_button;

        #[derive(Debug, Clone)]
        pub struct InvigilatorTT;

        impl InvigilatorTT {
            pub fn view(&self) -> Element<'_, Message> {
                text!("Hi, it's me").size(30).align_x(Center).into()
            }
        }

        #[derive(Debug, Clone)]
        pub enum Message {
            CreateInvigilatorList,
        }
    }
}

pub mod icon {
    use iced::svg::Svg;
    use iced::image::Image;

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
    
    pub fn ridge<'a>() -> Image<'a> {
        image(image::Handle::from_path(format!(
                    "{}/assets/images/Ridge_School_Kumasi_Logo.png",
                    env!("CARGO_MANIFEST_DIR")
        )))
            .height(60)
    }
}
