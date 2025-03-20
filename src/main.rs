use welcome::Welcome;
use exam_tt::ExamTT;
use invigilator_tt::InvigilatorTT;
use exam_analysis::ExamAnalysis;

use iced::{Element, Length, Task};

fn main() -> iced::Result {
    iced::run("Ridge Examination Tool", update, view)
}

#[derive(Default)]
struct State {
    screen: Screen,
}

enum Screen {
    WelcomeInterface(Welcome),
    InvigilatorTTInterface(InvigilatorTT),
    ExamTTInterface(ExamTT),
    ExamAnalysisInterface(ExamAnalysis),
}

impl Default for Screen {
    fn default() -> Self { 
        Screen::WelcomeInterface(Welcome) 
    }
}

#[derive(Clone, Debug)]
enum Message {
    GoToWelcomeInterface(welcome::Message),
    GoToInvigilatorTTInterface(invigilator_tt::Message),
    GoToExamTTInterface(exam_tt::Message),
    GoToExamAnalysisInterface(exam_analysis::Message),
}

pub fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::GoToWelcomeInterface(message) => {
            match message {
                welcome::Message::ExamTTButtonPressed => {
                    Task::none()
                }
                welcome::Message::InvigilatorTTButtonPressed => {
                    state.screen = Screen::InvigilatorTTInterface(InvigilatorTT);
                    Task::none()
                }
                welcome::Message::ExamAnalysisButtonPressed => {
                    Task::none()
                }
            }
        }
        Message::GoToInvigilatorTTInterface(message) => {
            todo!()
        }
        Message::GoToExamTTInterface(message) => {
            todo!()
        }
        Message::GoToExamAnalysisInterface(message) => {
            todo!()
        }
    }
}

pub fn view(state: &State) -> Element<Message> {
    match &state.screen {
        Screen::WelcomeInterface(welcome) => 
            welcome
                .view()
                .map(Message::GoToWelcomeInterface),
        Screen::InvigilatorTTInterface(invigilator_tt) => 
            invigilator_tt
                .view()
                .map(Message::GoToInvigilatorTTInterface),
        Screen::ExamTTInterface(exam_tt) => 
            exam_tt
                .view()
                .map(Message::GoToExamTTInterface),
        Screen::ExamAnalysisInterface(exam_analysis) =>
            exam_analysis
                .view()
                .map(Message::GoToExamAnalysisInterface),
    }
}

mod welcome {
    use ridge_exam_tool::domain;
    use ridge_exam_tool::widgets::welcome_button;
    use iced::widget::{
        Column, button, row, text, container, column, Button,
        svg, 
    };
    use iced::{
        Element, Fill, Center, Theme, Border, Background, border,
        Color, window, Task, Length,
    };
    use super::{
        State,
        Screen,
        update,
        invigilator_tt,
        exam_tt,
        exam_analysis,
        invigilator_tt::InvigilatorTT, 
        exam_tt::ExamTT,
        exam_analysis::ExamAnalysis,
    };

    #[derive(Debug, Clone)]
    pub enum Message {
        InvigilatorTTButtonPressed,
        ExamTTButtonPressed,
        ExamAnalysisButtonPressed,
    }

    #[derive(Default)]
    pub struct Welcome;

    impl Welcome {
        pub fn update(&self, message: Message
        ) -> Task<Message> {
            match message {
                Message::InvigilatorTTButtonPressed => {
                    Screen::InvigilatorTTInterface(InvigilatorTT);
                    Task::none()
                }
                Message::ExamTTButtonPressed => {
                    Task::none()
                }
                Message::ExamAnalysisButtonPressed => {
                    Task::none()
                }
            }
        }

        pub fn view(&self) -> Element<'_, Message> {
            let exam_tt_svg = 
                svg(svg::Handle::from_path(
                        format!("{}/assets/proctor-schedule.svg", 
                        env!("CARGO_MANIFEST_DIR"))
                        )
                    )
                .height(60);
            let invigilation_tt_svg = 
                svg(svg::Handle::from_path(
                        format!("{}/assets/exam-schedule.svg", 
                        env!("CARGO_MANIFEST_DIR"))
                        )
                    )
                .height(60);
            let exam_analysis_svg = 
                svg(svg::Handle::from_path(
                        format!("{}/assets/exam-analysis.svg", 
                        env!("CARGO_MANIFEST_DIR"))
                        )
                    )
                .height(60);

            let welcome_text = text!("Welcome!");
            let welcome_label: Element<_> = Column::new()
                .push(welcome_text.size(30))
                .spacing(10)
                .align_x(Center)
                .into();

            let invigilator_tt_btn = 
                welcome_button(
                    invigilation_tt_svg, 
                    "Create Invigilation Timetable"
                )
                .on_press(Message::InvigilatorTTButtonPressed);
            let exam_tt_btn = 
                welcome_button(
                    exam_tt_svg, 
                    "Create Examination Timetable"
                )
                .on_press(Message::ExamTTButtonPressed);
            let exam_analysis_btn = 
                welcome_button(
                    exam_analysis_svg, 
                    "Perform Examination Analysis"
                )
                .on_press(Message::ExamAnalysisButtonPressed);

            let btn_row = row![
                invigilator_tt_btn,
                exam_tt_btn,
                exam_analysis_btn,
            ]
            .spacing(40);

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

mod invigilator_tt {
    use ridge_exam_tool::domain;
    use ridge_exam_tool::widgets::welcome_button;
    use iced::widget::{
        Column, button, row, text, container, column, Button,
        svg, 
    };
    use iced::{
        Element, Fill, Center, Theme, Border, Background, border,
        Color, window, Task,
    };

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

mod exam_tt {
    use ridge_exam_tool::domain;
    use ridge_exam_tool::widgets::welcome_button;
    use iced::widget::{
        Column, button, row, text, container, column, Button,
        svg, 
    };
    use iced::{
        Element, Fill, Center, Theme, Border, Background, border,
        Color, window, Task,
    };

    #[derive(Debug, Clone)]
    pub struct ExamTT;

    impl ExamTT {
        pub fn view(&self) -> Element<'_, Message> {
            let exam_tt_svg = 
                svg(svg::Handle::from_path(format!("{}/assets/proctor-schedule.svg", env!("CARGO_MANIFEST_DIR"))))
                .height(60);
            let invigilation_tt_svg = 
                svg(svg::Handle::from_path(format!("{}/assets/exam-schedule.svg", env!("CARGO_MANIFEST_DIR"))))
                .height(60);
            let exam_analysis_svg = 
                svg(svg::Handle::from_path(format!("{}/assets/exam-analysis.svg", env!("CARGO_MANIFEST_DIR"))))
                .height(60);

            let welcome_text = text!("Welcome!");
            let welcome_label: Element<_> = Column::new()
                .push(welcome_text.size(30))
                .spacing(10)
                .align_x(Center)
                .into();

            let btn_row = row![
                welcome_button(invigilation_tt_svg, "Create Invigilation Timetable"),
                welcome_button(exam_tt_svg, "Create Examination Timetable"),
                welcome_button(exam_analysis_svg, "Perform Examination Analysis"),
            ]
            .spacing(40);

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

    #[derive(Debug, Clone)]
    pub enum Message {
        CreateStudentCombination,
    }
}

mod exam_analysis {
    use ridge_exam_tool::domain;
    use ridge_exam_tool::widgets::welcome_button;
    use iced::widget::{
        Column, button, row, text, container, column, Button,
        svg, 
    };
    use iced::{
        Element, Fill, Center, Theme, Border, Background, border,
        Color, window, Task,
    };
    // use super::welcome::Model;

    #[derive(Debug, Clone)]
    pub struct ExamAnalysis;

    impl ExamAnalysis {
        pub fn view(&self) -> Element<'_, Message> {
            let exam_tt_svg = 
                svg(svg::Handle::from_path(format!("{}/assets/proctor-schedule.svg", env!("CARGO_MANIFEST_DIR"))))
                .height(60);
            let invigilation_tt_svg = 
                svg(svg::Handle::from_path(format!("{}/assets/exam-schedule.svg", env!("CARGO_MANIFEST_DIR"))))
                .height(60);
            let exam_analysis_svg = 
                svg(svg::Handle::from_path(format!("{}/assets/exam-analysis.svg", env!("CARGO_MANIFEST_DIR"))))
                .height(60);

            let welcome_text = text!("Welcome!");
            let welcome_label: Element<_> = Column::new()
                .push(welcome_text.size(30))
                .spacing(10)
                .align_x(Center)
                .into();

            let btn_row = row![
                welcome_button(invigilation_tt_svg, "Create Invigilation Timetable"),
                welcome_button(exam_tt_svg, "Create Examination Timetable"),
                welcome_button(exam_analysis_svg, "Perform Examination Analysis"),
            ]
            .spacing(40);

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

    #[derive(Debug, Clone)]
    pub enum Message {
        SaveStudentPerformanceSummary,
    }
}

