use iced::widget::{Column, button, row, text, container, column};
use iced::{
    Element, Fill, Center, Task,
};

fn main() -> iced::Result {
    iced::run("Ridge Examination Tool", Welcome::update, Welcome::view)
}

// DOMAIN MODELS
// Exam and invigilation timetables
struct PersonalName {
    first_name: String,
    other_names: String,
    last_name: String,
}

struct Invigilator {
    name: PersonalName,
}

struct ExamDate {
    day: ExamDay,
    date: String,
}

enum Student {
    LowerSecondaryStudent,
    UpperSecondaryStudent,
}

struct StudentPersonalInfo {
    name: PersonalName,
    age: i32,
    gender: Gender,
}

enum Gender {
    Male,
    Female,
}

struct LowerSecondaryStudent {
    student: StudentPersonalInfo,
    student_class: LowerSecondaryStudentClass,
}

struct UpperSecondaryStudent {
    student: StudentPersonalInfo,
    student_class: UpperSecondaryStudentClass,
}

enum ExamDay {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

enum UpperSecondaryStudentClass {
    IG1,
    IG2,
    ASLevel,
    ALevel,
    WASSCE1,
    WASSCE2,
    WASSCE3,
}

enum LowerSecondaryStudentClass {
    Year8,
    Year9,
}

// Analysis of results
struct OverallSummary {
    overall_average: f32,
    best_subject: Subject,
    worst_subject: Subject,
    subject_specific_details: Vec<SubjectSpecificSummary>,
}

struct SubjectSpecificSummary {
    subject: Subject,
    overall_subject_score: f32,
    component_specific_details: ComponentSpecificSummary,
}

struct ComponentSpecificSummary {
    component_list: Vec<SubjectComponentSummary>,
}

struct SubjectComponentSummary {
    component_identifier: String,
    specific_score: f32,
    class_average: f32,
}

enum Subject {
    WassceSubject,
    ALevelSubject,
    IGCSESubject,
}

enum WassceSubject {
    English,
    CoreMathematics,
    IntegratedScience,
    SocialStudies,
    ElectiveMathematics,
    Physics,
    Chemistry,
    Biology,
    Government,
    Economics,
    Literature,
}

enum ALevelSubject {
    Physics,
    ProbabilityStatistics,
    PureMathematics,
    Mechanics,
    Economics,
    French,
    IT,
}

enum IGCSESubject {
    BusinessStudies,
    AdditionalMathematics,
    ExtendedMathematics,
    French,
    Chemistry,
    Biology,
    Geography,
}

// GUI MODELS
#[derive(Debug, Clone)]
enum RidgeExamToolMessage {
    OpenExamTTInterface(ExamTTInterface),
    OpenInvigilationTTInterface(InvigilationTTInterface),
    OpenExamAnalysisInterface(ExamAnalysisInterface),
    Exit,
}

#[derive(Debug, Clone)]
struct ExamAnalysisInterface {
}

#[derive(Debug, Clone)]
struct InvigilationTTInterface {
}

#[derive(Debug, Clone)]
struct ExamTTInterface {
}

#[derive(Default)]
pub struct Welcome; 

impl Welcome {
    pub fn new() -> Self {
        Welcome
    }

    pub fn update(&mut self, message: RidgeExamToolMessage) {
        match message {
            RidgeExamToolMessage::OpenExamTTInterface(_) => {
                todo!();
            }
            RidgeExamToolMessage::OpenInvigilationTTInterface(_) => {
                // call a function which takes this message as one of its arguments here
                // Task::perform(fn_name(arg))
                todo!();
            }
            RidgeExamToolMessage::OpenExamAnalysisInterface(_) => {
                todo!();
            }
            RidgeExamToolMessage::Exit => {
                todo!();
            }
        }
    }

    pub fn view(&self) -> Element<RidgeExamToolMessage> {
        let create_invigilation_tt = button("Create Invigilation Timetable");
        let create_exam_tt = button("Create Exam Timetable");
        let create_analysis_data_btn = button("Perform Analysis");
        // let create_invigilation_tt = button("Create Invigilation Timetable").on_press(RidgeExamToolMessage::OpenExamTTInterface(_));
        // let create_exam_tt = button("Create Exam Timetable").on_press(RidgeExamToolMessage::OpenInvigilationTTInterface(_));
        // let create_analysis_data_btn = button("Perform Analysis").on_press(RidgeExamToolMessage::OpenExamAnalysisInterface(_));
        let welcome_text = text!("Welcome!");
        // let welcome_label = welcome_text.size(30);
        let welcome_label: Element<_> = Column::new()
            .push(welcome_text.size(30))
            .spacing(10)
            .align_x(Center)
            .into();

        let btn_row = row![
            create_invigilation_tt, create_exam_tt, 
            create_analysis_data_btn
        ]
        .spacing(10);

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



