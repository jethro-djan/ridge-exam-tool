use iced::{Element, Length};

fn main() -> iced::Result {
    iced::run("Ridge Examination Tool", update, view)
}

pub enum Model {
    WelcomeInterface(welcome::Welcome),
    InvigilatorTTInterface(invigilator_tt::InvigilatorTT),
    ExamTTInterface(exam_tt::ExamTT),
    ExamAnalysisInterface(exam_analysis::ExamAnalysis),
}

impl Default for Model {
    fn default() -> Self { Model::WelcomeInterface(welcome::Welcome) }
}
#[derive(Debug, Clone)]
pub enum Message {
    GoToWelcomeMsg(welcome::Message),
    GoToInvigilatorTTMsg(invigilator_tt::Message),
    GoToExamTTMsg(exam_tt::Message),
    GoToExamAnalysisMsg(exam_analysis::Message),
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
    use super::Message as MainMessage;
    
    #[derive(Default)]
    pub struct Welcome;

    impl Welcome {
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

            let invigilator_tt_btn = 
                welcome_button(
                    invigilation_tt_svg, 
                    "Create Invigilation Timetable"
                )
                .on_press(Message::GoToInvigilatorTTMsg);
            let exam_tt_btn = 
                welcome_button(
                    exam_tt_svg, 
                    "Create Examination Timetable"
                )
                .on_press(Message::GoToExamTTMsg);
            let exam_analysis_btn = 
                welcome_button(
                    exam_analysis_svg, 
                    "Perform Examination Analysis"
                )
                .on_press(Message::GoToExamAnalysisMsg);
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

    #[derive(Debug, Clone)]
    pub enum Message {
        GoToInvigilatorTTMsg,
        GoToExamTTMsg,
        GoToExamAnalysisMsg,
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

    pub struct InvigilatorTT;

    impl InvigilatorTT {
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

    // Exam analysis model
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

fn update(state: &mut Model, message: Message) {
    match message {
        Message::GoToWelcomeMsg(_) => {
            todo!();
        }
        Message::GoToInvigilatorTTMsg(_) => {
            todo!()
        }
        Message::GoToExamTTMsg(_) => {
            todo!()
        }
        Message::GoToExamAnalysisMsg(_) => {
            todo!()
        }
    }
}

pub fn view(state: &Model) -> Element<'_, Message> {
    match &state {
        Model::WelcomeInterface(welcome) => welcome.view().map(Message::GoToWelcomeMsg),
        Model::InvigilatorTTInterface(invigilator_tt) => invigilator_tt.view().map(Message::GoToInvigilatorTTMsg),
        Model::ExamTTInterface(exam_tt) => exam_tt.view().map(Message::GoToExamTTMsg),
        Model::ExamAnalysisInterface(exam_analysis) => exam_analysis.view().map(Message::GoToExamAnalysisMsg),
    }
}
