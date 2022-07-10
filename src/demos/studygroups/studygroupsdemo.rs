use super::{Group, InterestArea, Student};
use crate::demos::console;
use crate::demos::menu::{display_state, simple_option, Menu, MenuResult};
use std::fmt::Display;

const MAX_STUDENTS: usize = 50;

pub struct DemoState {
    current_student_id: Option<usize>,
    students: Vec<Student>,
    groups: Vec<Group>,
}

impl DemoState {
    pub fn new() -> Self {
        Self {
            current_student_id: None,
            students: Vec::new(),
            groups: Vec::new(),
        }
    }

    pub fn current_student(&self) -> Option<&Student> {
        self.current_student_id.map(|id| &self.students[id])
    }
}

impl Display for DemoState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "==== Student Demo (UnionFind) ====")?;
        writeln!(f)?;
        if let Some(current_student_id) = self.current_student_id {
            writeln!(f, "Current student: {}", self.students[current_student_id])?;
        } else {
            writeln!(f, "Current student: None")?;
        }
        writeln!(
            f,
            "Student count: {} / {}",
            self.students.len(),
            MAX_STUDENTS
        )?;
        writeln!(f, "Group count: {}", self.groups.len())?;
        Ok(())
    }
}

pub fn study_groups_demo() -> MenuResult {
    let mut menu = Menu::new(
        display_state(),
        DemoState::new(),
        vec![
            simple_option("Deselect student", deselect_student),
            simple_option("Select student", select_student),
            simple_option("Add student", add_student),
            simple_option("Add group", add_group),
            simple_option("Add to group", add_to_group),
            simple_option("Join grop", join_group),
            simple_option("Show students", show_students),
            simple_option("Show groups", show_groups),
            simple_option("Show students in group", show_students_in_group),
        ],
    );
    menu.show()
}

pub fn deselect_student(demo_state: &mut DemoState) {
    demo_state.current_student_id = None;
}

pub fn select_student(demo_state: &mut DemoState) {
    println!("Student id:");
    let student_id = console::parse_line();
    if student_id.is_err() || *student_id.as_ref().unwrap() >= demo_state.students.len() {
        println!("Invalid student id");
        return;
    }
    demo_state.current_student_id = Some(student_id.unwrap());
}

pub fn add_student(demo_state: &mut DemoState) {
    if demo_state.students.len() >= MAX_STUDENTS {
        println!("Max count of students is {}", MAX_STUDENTS);
        return;
    }
    println!("Name:");
    let name = console::read_line();
    if name.is_empty() {
        println!("Name should not be empty");
        return;
    }
    println!("Areas (separated by space):");
    show_interest_areas();
    let mut interest_areas = Vec::new();
    for interest_area in console::read_line().split(' ') {
        let interest_area = interest_area
            .parse::<usize>()
            .map(|i| InterestArea::VALUES.get(i));
        if let Ok(Some(interest_area)) = interest_area {
            interest_areas.push(*interest_area);
        } else {
            println!("Invalid interest area");
            return;
        }
    }
    interest_areas.sort();
    interest_areas.dedup();
    let id = demo_state.students.len();
    demo_state.students.push(Student {
        id,
        name,
        interest_areas,
    });
    demo_state.current_student_id = Some(id);
    println!("Student added");
}

pub fn add_group(demo_state: &mut DemoState) {
    if demo_state.current_student_id.is_none() {
        println!("Select a student");
        return;
    }
    let current_student = demo_state.current_student().unwrap();
    println!("Name:");
    let name = console::read_line();
    if name.is_empty() {
        println!("Name should not be empty");
        return;
    }
    if demo_state.groups.iter().any(|group| group.name() == name) {
        println!("The name should be unique");
        return;
    }
    println!("Interest area:");
    show_interest_areas();
    let interest_area = console::parse_line();
    if interest_area.is_err() {
        println!("Invalid interest area");
        return;
    }
    let interest_area = interest_area.unwrap();
    if !current_student.interest_areas.contains(&interest_area) {
        println!("Only student interest areas are allowed");
        return;
    }
    demo_state.groups.push(Group::new(
        demo_state.current_student_id.unwrap(),
        name,
        interest_area,
        MAX_STUDENTS,
    ));
    println!("Group added");
}

pub fn add_to_group(demo_state: &mut DemoState) {
    if demo_state.current_student().is_none() {
        println!("Select a student");
        return;
    }
    let current_student_id = demo_state.current_student_id.unwrap();
    println!("Group name:");
    let name = console::read_line();
    let group_index = demo_state
        .groups
        .iter()
        .position(|group| group.name() == name);
    if group_index.is_none() {
        println!("Group not found");
        return;
    }
    let group_index = group_index.unwrap();
    let group = &demo_state.groups[group_index];
    if group.representative_id() != current_student_id {
        println!("Only group representative can add students");
        return;
    }
    println!("Student id:");
    let student_id = console::parse_line::<usize>();
    if student_id.is_err() {
        println!("Invalid student id");
        return;
    }
    let student_id = student_id.unwrap();
    let student_interests = &demo_state.students[student_id].interest_areas;
    if student_interests.contains(group.interest_area()) {
        demo_state.groups[group_index].add(student_id);
    }
    println!("Student added to group");
}

pub fn join_group(demo_state: &mut DemoState) {
    if demo_state.current_student().is_none() {
        println!("Select a student");
        return;
    }
    println!("Group name:");
    let name = console::read_line();
    let group_index = demo_state
        .groups
        .iter()
        .position(|group| group.name() == name);
    if group_index.is_none() {
        println!("Group not found");
        return;
    }
    let group_index = group_index.unwrap();
    let current_interest_areas = &demo_state.current_student().unwrap().interest_areas;
    if !current_interest_areas.contains(demo_state.groups[group_index].interest_area()) {
        println!("Incompatible interest areas");
        return;
    }
    println!("Joined group");
    demo_state.groups[group_index].add(demo_state.current_student_id.unwrap());
}

pub fn show_students(demo_state: &mut DemoState) {
    for student in demo_state.students.iter() {
        println!("{}", student)
    }
}

pub fn show_groups(demo_state: &mut DemoState) {
    for group in demo_state.groups.iter() {
        println!(
            "Name: {}, Representative: {}, Size: {}, Interest area: {}",
            group.name(),
            demo_state.students[group.representative_id()].name,
            group.len(),
            group.interest_area(),
        );
    }
}

pub fn show_students_in_group(demo_state: &mut DemoState) {
    println!("Group name:");
    let name = console::read_line();
    let group = demo_state.groups.iter().find(|group| group.name() == name);
    if group.is_none() {
        println!("Group not found");
        return;
    }
    let group = group.unwrap();
    let students_in_group = demo_state
        .students
        .iter()
        .filter(|student| group.contains(student.id));
    for student in students_in_group {
        println!("{}", student);
    }
}

pub fn show_interest_areas() {
    for (index, interest_area) in InterestArea::VALUES.iter().enumerate() {
        println!("{}. {}", index, interest_area);
    }
}
