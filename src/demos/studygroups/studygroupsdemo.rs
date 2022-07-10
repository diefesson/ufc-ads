use super::{Group, InterestArea, Student};
use crate::demos::console;
use crate::demos::menu::{display_state, menu_option, Menu, MenuResult};
use crate::demos::studygroups::interestarea;
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
            menu_option("Deselect student", |state| deselect_student(state)),
            menu_option("Select student", |state| select_student(state)),
            menu_option("Add student", |state| add_student(state)),
            menu_option("Add group", |state| add_group(state)),
            menu_option("Show students", |state| show_students(state)),
            menu_option("Show groups", |state| show_groups(state)),
        ],
    );
    menu.show()
}

pub fn deselect_student(demo_state: &mut DemoState) -> MenuResult {
    demo_state.current_student_id = None;
    Ok(())
}

pub fn select_student(demo_state: &mut DemoState) -> MenuResult {
    println!("Student id:");
    let student_id = console::parse_line();
    if student_id.is_err() || *student_id.as_ref().unwrap() >= demo_state.students.len() {
        println!("Invalid student id");
        return Ok(());
    }
    demo_state.current_student_id = Some(student_id.unwrap());
    Ok(())
}

pub fn add_student(demo_state: &mut DemoState) -> MenuResult {
    if demo_state.students.len() >= MAX_STUDENTS {
        println!("Max count of students is {}", MAX_STUDENTS);
        return Ok(());
    }
    println!("Name:");
    let name = console::read_line();
    if name.len() == 0 {
        println!("Name should not be empty");
        return Ok(());
    }
    println!("Areas (separated by space):");
    show_interest_areas();
    let mut interest_areas = Vec::new();
    for interest_area in console::read_line().split(' ') {
        let interest_area = interest_area
            .parse::<usize>()
            .map(|i| InterestArea::VALUES.get(i));
        if let Ok(Some(interest_area)) = interest_area {
            if !interest_areas.contains(interest_area) {
                interest_areas.push(*interest_area)
            }
        } else {
            println!("Invalid interest area")
        }
    }
    let id = demo_state.students.len();
    demo_state.students.push(Student {
        id,
        name,
        interest_areas,
    });
    demo_state.current_student_id = Some(id);
    println!("Student added");
    Ok(())
}

pub fn add_group(demo_state: &mut DemoState) -> MenuResult {
    if demo_state.current_student_id.is_none() {
        println!("Select a student to represent the new group");
        return Ok(());
    }
    let current_student = demo_state.current_student().unwrap();
    println!("Name:");
    let name = console::read_line();
    if name.len() == 0 {
        println!("Name should not be empty");
        return Ok(());
    }
    if demo_state.groups.iter().any(|group| group.name() == name) {
        println!("The name should be unique");
        return Ok(());
    }
    println!("Interest area:");
    show_interest_areas();
    let interest_area = console::parse_line();
    if interest_area.is_err() {
        println!("Invalid interest area");
        return Ok(());
    }
    let interest_area = interest_area.unwrap();
    if !current_student.interest_areas.contains(&interest_area) {
        println!("Only student interest areas are allowed");
        return Ok(());
    }
    demo_state.groups.push(Group::new(
        demo_state.current_student_id.unwrap(),
        name,
        interest_area,
        MAX_STUDENTS,
    ));
    println!("Group added");
    Ok(())
}

pub fn show_students(demo_state: &DemoState) -> MenuResult {
    for student in demo_state.students.iter() {
        println!("{}", student)
    }
    Ok(())
}

pub fn show_groups(demo_state: &DemoState) -> MenuResult {
    for group in demo_state.groups.iter() {
        println!(
            "({}) {} [{}] : {}",
            demo_state.students[group.representative_id()],
            group.name(),
            group.len(),
            group.interest_area(),
        );
        let representative = &demo_state.students[group.representative_id()];
        println!("Represented by: {}", representative);
    }
    Ok(())
}

pub fn show_interest_areas() {
    for (index, interest_area) in InterestArea::VALUES.iter().enumerate() {
        println!("{}. {}", index, interest_area);
    }
}
