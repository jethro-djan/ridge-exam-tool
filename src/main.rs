use ridge_exam_tool::models;
use ridge_exam_tool::styles;
use iced::widget::{
    Column, button, row, text, container, column, Button,
    svg, 
};
use iced::{
    Element, Fill, Center, Theme, Border, Background, border,
    Color, window, Task,
};

fn main() -> iced::Result {
    iced::run("Ridge Examination Tool", update, view)
}

#[derive(Default)]
struct RidgeExamTool {
    state: Model,
}

enum Model {
    WelcomeInterface(welcome::Welcome),
    InvigilatorTTInterface(invigilator_tt::InvigilatorTT),
    ExamTTInterface(exam_tt::ExamTT),
    ExamAnalysisInterface(exam_analysis::ExamAnalysis),
}

impl Default for Model {
    fn default() -> Self { Model::WelcomeInterface(welcome::Welcome) }
}
#[derive(Debug)]
enum Message {
    GoToWelcomeMsg(welcome::Message),
    GoToInvigilatorTTMsg(invigilator_tt::Message),
    GoToExamTTMsg(exam_tt::Message),
    GoToExamAnalysisMsg(exam_analysis::Message),
}

mod welcome {
    use ridge_exam_tool::models;
    use ridge_exam_tool::styles;
    use iced::widget::{
        Column, button, row, text, container, column, Button,
        svg, 
    };
    use iced::{
        Element, Fill, Center, Theme, Border, Background, border,
        Color, window, Task,
    };
    
    #[derive(Default)]
    pub struct Welcome;

    impl Welcome {
        pub fn view(&self) -> Element<'_, Message> {
            let exam_tt_svg = 
                svg(svg::Handle::from_path(format!("{}/assets/proctor-schedule.svg", env!("CARGO_MANIFEST_DIR"))))
                .height(60);
            let exam_tt_icon = column![
                exam_tt_svg,
                "Create Invigilation Timetable"
            ]
            .align_x(Center);
            let invigilation_tt_svg = 
                svg(svg::Handle::from_path(format!("{}/assets/exam-schedule.svg", env!("CARGO_MANIFEST_DIR"))))
                .height(60);
            let invigilation_tt_icon = column![
                invigilation_tt_svg,
                "Create Examination Timetable"
            ]
            .align_x(Center);
            let exam_analysis_svg = 
                svg(svg::Handle::from_path(format!("{}/assets/exam-analysis.svg", env!("CARGO_MANIFEST_DIR"))))
                .height(60);
            let exam_analysis_icon = column![
                exam_analysis_svg,
                "Perform Examination Analysis"
            ]
            .align_x(Center);
            let create_invigilation_tt = 
                button(invigilation_tt_icon)
                .padding(styles::WelcomeButtonStyle::PADDING)
                .width(250)
                .style(|theme: &Theme, status| {
                    styles::WelcomeButtonStyle::style(theme, status)
                });
                // .on_press(RidgeExamToolMessage::OpenWelcomeInterface(MainMessage));
            let create_exam_tt = 
                Button::new(exam_tt_icon)
                .width(250)
                .padding(styles::WelcomeButtonStyle::PADDING)
                .style(|theme: &Theme, status| {
                    styles::WelcomeButtonStyle::style(theme, status)
                });
                // .on_press(RidgeExamToolMessage::OpenWelcomeInterface(MainMessage::OpenExamTTInterface));
            let perform_exam_analysis = 
                button(exam_analysis_icon)
                .padding(styles::WelcomeButtonStyle::PADDING)
                .width(250)
                .style(|theme: &Theme, status| {
                    styles::WelcomeButtonStyle::style(theme, status)
                });
                // .on_press(RidgeExamToolMessage::OpenWelcomeInterface(MainMessage::OpenExamAnalysisInterface));
            let welcome_text = text!("Welcome!");
            let welcome_label: Element<_> = Column::new()
                .push(welcome_text.size(30))
                .spacing(10)
                .align_x(Center)
                .into();

            let btn_row = row![
                create_invigilation_tt, 
                create_exam_tt, 
                perform_exam_analysis,
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
    }

}

mod invigilator_tt {
    use ridge_exam_tool::models;
    use ridge_exam_tool::styles;
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
            let exam_tt_icon = column![
                exam_tt_svg,
                "Create Invigilation Timetable"
            ]
            .align_x(Center);
            let invigilation_tt_svg = 
                svg(svg::Handle::from_path(format!("{}/assets/exam-schedule.svg", env!("CARGO_MANIFEST_DIR"))))
                .height(60);
            let invigilation_tt_icon = column![
                invigilation_tt_svg,
                "Create Examination Timetable"
            ]
            .align_x(Center);
            let exam_analysis_svg = 
                svg(svg::Handle::from_path(format!("{}/assets/exam-analysis.svg", env!("CARGO_MANIFEST_DIR"))))
                .height(60);
            let exam_analysis_icon = column![
                exam_analysis_svg,
                "Perform Examination Analysis"
            ]
            .align_x(Center);
            let create_invigilation_tt = 
                button(invigilation_tt_icon)
                .padding(styles::WelcomeButtonStyle::PADDING)
                .width(250)
                .style(|theme: &Theme, status| {
                    styles::WelcomeButtonStyle::style(theme, status)
                });
                // .on_press(RidgeExamToolMessage::OpenWelcomeInterface(MainMessage));
            let create_exam_tt = 
                Button::new(exam_tt_icon)
                .width(250)
                .padding(styles::WelcomeButtonStyle::PADDING)
                .style(|theme: &Theme, status| {
                    styles::WelcomeButtonStyle::style(theme, status)
                });
                // .on_press(RidgeExamToolMessage::OpenWelcomeInterface(MainMessage::OpenExamTTInterface));
            let perform_exam_analysis = 
                button(exam_analysis_icon)
                .padding(styles::WelcomeButtonStyle::PADDING)
                .width(250)
                .style(|theme: &Theme, status| {
                    styles::WelcomeButtonStyle::style(theme, status)
                });
                // .on_press(RidgeExamToolMessage::OpenWelcomeInterface(MainMessage::OpenExamAnalysisInterface));
            let welcome_text = text!("Welcome!");
            let welcome_label: Element<_> = Column::new()
                .push(welcome_text.size(30))
                .spacing(10)
                .align_x(Center)
                .into();

            let btn_row = row![
                create_invigilation_tt, 
                create_exam_tt, 
                perform_exam_analysis,
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
    }

}

mod exam_tt {
    use ridge_exam_tool::models;
    use ridge_exam_tool::styles;
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
            let exam_tt_icon = column![
                exam_tt_svg,
                "Create Invigilation Timetable"
            ]
            .align_x(Center);
            let invigilation_tt_svg = 
                svg(svg::Handle::from_path(format!("{}/assets/exam-schedule.svg", env!("CARGO_MANIFEST_DIR"))))
                .height(60);
            let invigilation_tt_icon = column![
                invigilation_tt_svg,
                "Create Examination Timetable"
            ]
            .align_x(Center);
            let exam_analysis_svg = 
                svg(svg::Handle::from_path(format!("{}/assets/exam-analysis.svg", env!("CARGO_MANIFEST_DIR"))))
                .height(60);
            let exam_analysis_icon = column![
                exam_analysis_svg,
                "Perform Examination Analysis"
            ]
            .align_x(Center);
            let create_invigilation_tt = 
                button(invigilation_tt_icon)
                .padding(styles::WelcomeButtonStyle::PADDING)
                .width(250)
                .style(|theme: &Theme, status| {
                    styles::WelcomeButtonStyle::style(theme, status)
                });
                // .on_press(RidgeExamToolMessage::OpenWelcomeInterface(MainMessage));
            let create_exam_tt = 
                Button::new(exam_tt_icon)
                .width(250)
                .padding(styles::WelcomeButtonStyle::PADDING)
                .style(|theme: &Theme, status| {
                    styles::WelcomeButtonStyle::style(theme, status)
                });
                // .on_press(RidgeExamToolMessage::OpenWelcomeInterface(MainMessage::OpenExamTTInterface));
            let perform_exam_analysis = 
                button(exam_analysis_icon)
                .padding(styles::WelcomeButtonStyle::PADDING)
                .width(250)
                .style(|theme: &Theme, status| {
                    styles::WelcomeButtonStyle::style(theme, status)
                });
                // .on_press(RidgeExamToolMessage::OpenWelcomeInterface(MainMessage::OpenExamAnalysisInterface));
            let welcome_text = text!("Welcome!");
            let welcome_label: Element<_> = Column::new()
                .push(welcome_text.size(30))
                .spacing(10)
                .align_x(Center)
                .into();

            let btn_row = row![
                create_invigilation_tt, 
                create_exam_tt, 
                perform_exam_analysis,
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
    }



}

mod exam_analysis {
    use ridge_exam_tool::models;
    use ridge_exam_tool::styles;
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
            let exam_tt_icon = column![
                exam_tt_svg,
                "Create Invigilation Timetable"
            ]
            .align_x(Center);
            let invigilation_tt_svg = 
                svg(svg::Handle::from_path(format!("{}/assets/exam-schedule.svg", env!("CARGO_MANIFEST_DIR"))))
                .height(60);
            let invigilation_tt_icon = column![
                invigilation_tt_svg,
                "Create Examination Timetable"
            ]
            .align_x(Center);
            let exam_analysis_svg = 
                svg(svg::Handle::from_path(format!("{}/assets/exam-analysis.svg", env!("CARGO_MANIFEST_DIR"))))
                .height(60);
            let exam_analysis_icon = column![
                exam_analysis_svg,
                "Perform Examination Analysis"
            ]
            .align_x(Center);
            let create_invigilation_tt = 
                button(invigilation_tt_icon)
                .padding(styles::WelcomeButtonStyle::PADDING)
                .width(250)
                .style(|theme: &Theme, status| {
                    styles::WelcomeButtonStyle::style(theme, status)
                });
                // .on_press(RidgeExamToolMessage::OpenWelcomeInterface(MainMessage));
            let create_exam_tt = 
                Button::new(exam_tt_icon)
                .width(250)
                .padding(styles::WelcomeButtonStyle::PADDING)
                .style(|theme: &Theme, status| {
                    styles::WelcomeButtonStyle::style(theme, status)
                });
                // .on_press(RidgeExamToolMessage::OpenWelcomeInterface(MainMessage::OpenExamTTInterface));
            let perform_exam_analysis = 
                button(exam_analysis_icon)
                .padding(styles::WelcomeButtonStyle::PADDING)
                .width(250)
                .style(|theme: &Theme, status| {
                    styles::WelcomeButtonStyle::style(theme, status)
                });
                // .on_press(RidgeExamToolMessage::OpenWelcomeInterface(MainMessage::OpenExamAnalysisInterface));
            let welcome_text = text!("Welcome!");
            let welcome_label: Element<_> = Column::new()
                .push(welcome_text.size(30))
                .spacing(10)
                .align_x(Center)
                .into();

            let btn_row = row![
                create_invigilation_tt, 
                create_exam_tt, 
                perform_exam_analysis,
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
    }

}

fn update(state: &mut RidgeExamTool, message: Message) {
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

pub fn view(state: &RidgeExamTool) -> Element<'_, Message> {
    match &state.state {
        Model::WelcomeInterface(welcome) => welcome.view().map(Message::GoToWelcomeMsg),
        Model::InvigilatorTTInterface(invigilator_tt) => invigilator_tt.view().map(Message::GoToInvigilatorTTMsg),
        Model::ExamTTInterface(exam_tt) => exam_tt.view().map(Message::GoToExamTTMsg),
        Model::ExamAnalysisInterface(exam_analysis) => exam_analysis.view().map(Message::GoToExamAnalysisMsg),
    }
}
