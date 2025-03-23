// DOMAIN MODELS
// Exam and invigilation timetables
struct PersonalName {
    first_name: String,
    other_names: String,
    last_name: String,
}

struct Teacher {
    teacher: TeacherPersonalInfo,
    subjects_taught: Vec<Subject>,
}

struct TeacherPersonalInfo {
    name: PersonalName,
    gender: Gender,
}

pub struct Invigilator {
    name: PersonalName,
    level: StudentClass,
}

struct ExamDate {
    day: ExamDay,
    date: String,
}

struct Student {
    student: StudentPersonalInfo,
    class: StudentClass,
    subjects_read: Vec<Subject>,
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

enum StudentClass {
    LowerSecondaryStudentClass,
    UpperSecondaryStudentClass,
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

pub struct StudentPerformanceSummary {
    student: Student,
    overall_average: f32,
    best_subject: Subject,
    worst_subject: Subject,
    subject_specific_details: Vec<SubjectSpecificSummary>,
}

pub struct ClassPerformanceSummary {
    class: StudentClass,
    class_overall_average: f32,
    class_subject_performance_summary: Vec<ClassSubjectSummary>,
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
struct ClassSubjectSummary {
    subject: Subject,
    subject_average: f32,
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

struct ExamPair {
    exam_date: ExamDate,
    subject_list: Vec<Subject>,
}

pub struct ExamTTObject {
    student_class: StudentClass,
    exam_pair: ExamPair,
}
