use std::collections::VecDeque;

use dialoguer::{Completion, History};

use babibapp_api::types::*;

pub struct BabicliHistory {
    max: usize,
    history: VecDeque<String>,
}

impl Default for BabicliHistory {
    fn default() -> Self {
        Self {
            max: 16,
            history: VecDeque::new(),
        }
    }
}

impl<T: ToString> History<T> for BabicliHistory {
    fn read(&self, pos: usize) -> Option<String> {
        self.history.get(pos).cloned()
    }

    fn write(&mut self, val: &T) {
        if self.history.len() == self.max {
            self.history.pop_back();
        }
        self.history.push_front(val.to_string());
    }
}

pub struct BabicliCompletion {
    options: Vec<String>,
}

impl BabicliCompletion {
    pub fn new(options: &[String]) -> Self {
        Self {
            options: options.to_vec(),
        }
    }
}

impl Completion for BabicliCompletion {
    fn get(&self, input: &str) -> Option<String> {
        let matches = self
            .options
            .iter()
            .filter(|opt| opt.starts_with(input))
            .collect::<Vec<_>>();

        if matches.len() == 1 {
            Some(matches[0].to_string())
        } else {
            None
        }
    }
}

pub fn view_student(student: &StudentView) {
    match student {
        StudentView::Limited(student) => {
            println!("{} {}", student.first_name, student.last_name);
            println!("----------------");
            println!("id: {}", student.id);
        }
        StudentView::Full(student) => {
            match student.admin {
                true => println!("{} {} (admin)", student.first_name, student.last_name),
                false => println!("{} {}", student.first_name, student.last_name),
            }
            println!("----------------");
            println!("id: {}", student.id);
            println!("Email: {}", student.email);
            println!("Password hash: {}", student.password_hash);
        }
    }
}

pub fn view_teacher(teacher: &Teacher) {
    println!("{} {}", teacher.prefix, teacher.name);
    println!("----------------");
    println!("id: {}", teacher.id);
}
