use std::collections::VecDeque;

use chrono::offset::Local;
use chrono::DateTime;
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
    options: Vec<&'static str>,
}

impl BabicliCompletion {
    pub fn new(options: &[&'static str]) -> Self {
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

        if matches.is_empty() {
            return None;
        } else if matches.len() == 1 {
            return Some(matches[0].to_string());
        }

        let mut completion = String::new();
        'outer: for (i, c) in matches[0].chars().enumerate() {
            for word in &matches {
                let wc = match word.chars().nth(i) {
                    Some(wc) => wc,
                    None => break 'outer,
                };

                if wc != c {
                    break 'outer;
                }
            }
            completion.push(c);
        }

        if completion.is_empty() {
            return None;
        }

        Some(completion)
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

pub fn view_student_comment_limited(
    comment: &LimitedViewStudentComment,
    recv: &StudentView,
    vote: i64,
) {
    let recv_name = match recv {
        StudentView::Limited(student) => (&student.first_name, &student.last_name),
        StudentView::Full(student) => (&student.first_name, &student.last_name),
    };

    let published: DateTime<Local> = comment.published.into();

    println!("-> {} {} [{}]", recv_name.0, recv_name.1, comment.id,);
    println!("----------------");
    println!("{}", comment.body);
    println!("----------------");
    println!(
        "Vote: {}, published: {}",
        vote,
        published.format("%d.%m.%Y %T")
    );
}

pub fn view_student_comment_full(
    comment: &StudentComment,
    recv: &StudentView,
    author: &StudentView,
    vote: i64,
) {
    let recv_name = match recv {
        StudentView::Limited(student) => (&student.first_name, &student.last_name),
        StudentView::Full(student) => (&student.first_name, &student.last_name),
    };

    let author_name = match author {
        StudentView::Limited(student) => (&student.first_name, &student.last_name),
        StudentView::Full(student) => (&student.first_name, &student.last_name),
    };

    let published: DateTime<Local> = comment.published.into();

    println!(
        "{} {} -> {} {} [{}]",
        author_name.0, author_name.1, recv_name.0, recv_name.1, comment.id,
    );
    println!("----------------");
    println!("{}", comment.body);
    println!("----------------");
    println!(
        "Vote: {}, published: {}",
        vote,
        published.format("%d.%m.%Y %T")
    );
}

pub fn view_teacher_comment_limited(
    comment: &LimitedViewTeacherComment,
    recv: &Teacher,
    vote: i64,
) {
    let published: DateTime<Local> = comment.published.into();

    println!("-> {} {} [{}]", recv.prefix, recv.name, comment.id,);
    println!("----------------");
    println!("{}", comment.body);
    println!("----------------");
    println!(
        "Vote: {}, published: {}",
        vote,
        published.format("%d.%m.%Y %T")
    );
}

pub fn view_teacher_comment_full(
    comment: &TeacherComment,
    recv: &Teacher,
    author: &StudentView,
    vote: i64,
) {
    let author_name = match author {
        StudentView::Limited(teacher) => (&teacher.first_name, &teacher.last_name),
        StudentView::Full(teacher) => (&teacher.first_name, &teacher.last_name),
    };

    let published: DateTime<Local> = comment.published.into();

    println!(
        "{} {} -> {} {} [{}]",
        author_name.0, author_name.1, recv.prefix, recv.name, comment.id,
    );
    println!("----------------");
    println!("{}", comment.body);
    println!("----------------");
    println!(
        "Vote: {}, published: {}",
        vote,
        published.format("%d.%m.%Y %T")
    );
}
