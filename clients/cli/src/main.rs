use std::{env, process};

use dialoguer::theme::{ColorfulTheme, SimpleTheme};

use babibapp_api::types::*;
use babibapp_api::BabibappClient;
use babicli::{BabicliCompletion, BabicliHistory};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args().skip(1);
    let base_url = args.next().unwrap_or_else(|| {
        eprintln!("Please pass the base url to the babibapp server API");
        process::exit(1);
    });

    // get credentials
    let cred_theme = ColorfulTheme::default();

    let email: String = dialoguer::Input::with_theme(&cred_theme)
        .with_prompt("Your email")
        .validate_with({
            let mut force = None;
            move |input: &String| -> Result<(), &str> {
                if input.contains('@') || force.as_ref().map_or(false, |old| old == input) {
                    Ok(())
                } else {
                    force = Some(input.clone());
                    Err("This is not a mail address; type the same value again to force use")
                }
            }
        })
        .interact_text()
        .unwrap();

    let password = dialoguer::Password::with_theme(&cred_theme)
        .with_prompt("Password")
        .interact()
        .unwrap();

    // init client
    let babibapp = BabibappClient::login(&base_url, &email, &password)
        .await
        .unwrap_or_else(|_| {
            eprintln!("Failed to login");
            process::exit(1);
        });

    println!();
    println!("Successfully connected to {}!", base_url);
    println!();
    println!("Use `exit` to quit the program or ask for `help`.");
    println!("Use the Up/Down arrows to scroll through history.");
    println!("Use the Right arrow or Tab to complete your command.");

    let mut history = BabicliHistory::default();
    let completion = BabicliCompletion::new(&vec![
        "validate_token".to_string(),
        "get_student".to_string(),
        "get_self".to_string(),
        "get_all_students".to_string(),
        "register_student".to_string(),
        "reset_student".to_string(),
        "delete_student".to_string(),
        "make_student_admin".to_string(),
        "get_teacher".to_string(),
        "get_all_teachers".to_string(),
        "clear".to_string(),
        "help".to_string(),
        "exit".to_string(),
        "quit".to_string(),
    ]);

    loop {
        println!();

        let cmd_theme = ColorfulTheme::default();
        let info_theme = SimpleTheme;

        if let Ok(cmd) = dialoguer::Input::<String>::with_theme(&cmd_theme)
            .with_prompt("babicli")
            .history_with(&mut history)
            .completion_with(&completion)
            .interact_text()
        {
            let mut args = cmd.split_whitespace();
            match args.next() {
                Some("validate_token") => {
                    let valid = match babibapp.validate_token().await {
                        Ok(res) => res,
                        Err(_) => {
                            eprintln!("Failed to validate token");
                            continue;
                        }
                    };

                    if valid {
                        println!("The token is valid!");
                    } else {
                        println!("The token is invalid!");
                    }
                }

                Some("get_student") => {
                    let id = if let Some(id) = args.next() {
                        if let Ok(id) = id.parse::<i32>() {
                            id
                        } else {
                            eprintln!("Invalid student id");
                            continue;
                        }
                    } else {
                        if let Ok(id) = dialoguer::Input::<i32>::with_theme(&info_theme)
                            .with_prompt("id")
                            .interact_text()
                        {
                            id
                        } else {
                            eprintln!("Invalid student id");
                            continue;
                        }
                    };

                    let student = match babibapp.get_student(id).await {
                        Ok(student) => student,
                        Err(_) => {
                            eprintln!("Failed to get student");
                            continue;
                        }
                    };

                    babicli::view_student(&student);
                }

                Some("get_self") => {
                    let me = match babibapp.get_self().await {
                        Ok(me) => me,
                        Err(_) => {
                            eprintln!("Failed to get self");
                            continue;
                        }
                    };

                    babicli::view_student(&StudentView::Full(me));
                }

                Some("get_all_students") => {
                    let students = match babibapp.get_all_students().await {
                        Ok(students) => students,
                        Err(_) => {
                            eprintln!("Failed to get all students");
                            continue;
                        }
                    };

                    if students.is_empty() {
                        println!("No students found");
                        continue;
                    }

                    for student in &students {
                        babicli::view_student(student);
                        println!();
                    }
                }

                Some("register_student") => {
                    let email: String = match dialoguer::Input::with_theme(&info_theme)
                        .with_prompt("Email")
                        .validate_with({
                            let mut force = None;
                            move |input: &String| -> Result<(), &str> {
                                if input.contains('@') || force.as_ref().map_or(false, |old| old == input) {
                                    Ok(())
                                } else {
                                    force = Some(input.clone());
                                    Err("Is this a valid email address? Type the same value again to force use")
                                }
                            }
                        })
                        .interact_text() {
                        Ok(email) => email,
                        Err(_) => {
                            eprintln!("Failed to read email");
                            continue;
                        }
                    };

                    let first_name: String = match dialoguer::Input::with_theme(&info_theme)
                        .with_prompt("First name")
                        .interact_text()
                    {
                        Ok(first_name) => first_name,
                        Err(_) => {
                            eprintln!("Failed to read first name");
                            continue;
                        }
                    };

                    let last_name: String = match dialoguer::Input::with_theme(&info_theme)
                        .with_prompt("Last name")
                        .interact_text()
                    {
                        Ok(last_name) => last_name,
                        Err(_) => {
                            eprintln!("Failed to read first name");
                            continue;
                        }
                    };

                    let password: String = match dialoguer::Password::with_theme(&info_theme)
                        .with_prompt("Password")
                        .with_confirmation("Repeat password", "The passwords do not match")
                        .interact()
                    {
                        Ok(password) => password,
                        Err(_) => {
                            eprintln!("Failed to read password");
                            continue;
                        }
                    };

                    let admin = match dialoguer::Confirm::with_theme(&info_theme)
                        .with_prompt(format!("Is {} {} an admin?", first_name, last_name))
                        .default(false)
                        .interact()
                    {
                        Ok(admin) => admin,
                        Err(_) => {
                            eprintln!("Failed to read admin");
                            continue;
                        }
                    };

                    let student = match babibapp
                        .register_student(&email, &first_name, &last_name, &password, Some(admin))
                        .await
                    {
                        Ok(student) => student,
                        Err(_) => {
                            eprintln!("Failed to register student");
                            continue;
                        }
                    };

                    println!("\nStudent successfully registered!");
                    babicli::view_student(&StudentView::Full(student));
                }

                Some("reset_student") => {
                    let id = if let Some(id) = args.next() {
                        if let Ok(id) = id.parse::<i32>() {
                            id
                        } else {
                            eprintln!("Invalid student id");
                            continue;
                        }
                    } else {
                        if let Ok(id) = dialoguer::Input::<i32>::with_theme(&info_theme)
                            .with_prompt("id")
                            .interact_text()
                        {
                            id
                        } else {
                            eprintln!("Invalid student id");
                            continue;
                        }
                    };

                    let reset_options = &["Email", "Password", "Name", "Full"];

                    let student = match dialoguer::Select::with_theme(&info_theme)
                        .items(reset_options)
                        .interact()
                    {
                        Ok(idx) => match reset_options[idx] {
                            "Email" => {
                                let email: String = match dialoguer::Input::with_theme(&info_theme)
                                    .with_prompt("Email")
                                    .validate_with({
                                        let mut force = None;
                                        move |input: &String| -> Result<(), &str> {
                                            if input.contains('@') || force.as_ref().map_or(false, |old| old == input) {
                                                Ok(())
                                            } else {
                                                force = Some(input.clone());
                                                Err("Is this a valid email address? Type the same value again to force use")
                                            }
                                        }
                                    })
                                    .interact_text() {
                                    Ok(email) => email,
                                    Err(_) => {
                                        eprintln!("Failed to read new email");
                                        continue;
                                    }
                                };

                                let student = match babibapp.reset_student_email(id, &email).await {
                                    Ok(student) => student,
                                    Err(_) => {
                                        eprintln!("Failed to reset student email");
                                        continue;
                                    }
                                };

                                student
                            }
                            "Password" => {
                                let password: String =
                                    match dialoguer::Password::with_theme(&info_theme)
                                        .with_prompt("Password")
                                        .with_confirmation(
                                            "Repeat password",
                                            "The passwords do not match",
                                        )
                                        .interact()
                                    {
                                        Ok(password) => password,
                                        Err(_) => {
                                            eprintln!("Failed to read new password");
                                            continue;
                                        }
                                    };

                                let student =
                                    match babibapp.reset_student_password(id, &password).await {
                                        Ok(student) => student,
                                        Err(_) => {
                                            eprintln!("Failed to reset student password");
                                            continue;
                                        }
                                    };

                                student
                            }
                            "Name" => {
                                let first_name: String =
                                    match dialoguer::Input::with_theme(&info_theme)
                                        .with_prompt("First name")
                                        .interact_text()
                                    {
                                        Ok(first_name) => first_name,
                                        Err(_) => {
                                            eprintln!("Failed to read new first name");
                                            continue;
                                        }
                                    };

                                let last_name: String =
                                    match dialoguer::Input::with_theme(&info_theme)
                                        .with_prompt("Last name")
                                        .interact_text()
                                    {
                                        Ok(last_name) => last_name,
                                        Err(_) => {
                                            eprintln!("Failed to read new last name");
                                            continue;
                                        }
                                    };

                                let student = match babibapp
                                    .reset_student_name(id, &first_name, &last_name)
                                    .await
                                {
                                    Ok(student) => student,
                                    Err(_) => {
                                        eprintln!("Failed to reset student name");
                                        continue;
                                    }
                                };

                                student
                            }
                            "Full" => {
                                let email: String = match dialoguer::Input::with_theme(&info_theme)
                                    .with_prompt("Email")
                                    .validate_with({
                                        let mut force = None;
                                        move |input: &String| -> Result<(), &str> {
                                            if input.contains('@') || force.as_ref().map_or(false, |old| old == input) {
                                                Ok(())
                                            } else {
                                                force = Some(input.clone());
                                                Err("Is this a valid email address? Type the same value again to force use")
                                            }
                                        }
                                    })
                                    .interact_text() {
                                    Ok(email) => email,
                                    Err(_) => {
                                        eprintln!("Failed to read new email");
                                        continue;
                                    }
                                };

                                let first_name: String =
                                    match dialoguer::Input::with_theme(&info_theme)
                                        .with_prompt("First name")
                                        .interact_text()
                                    {
                                        Ok(first_name) => first_name,
                                        Err(_) => {
                                            eprintln!("Failed to read new first name");
                                            continue;
                                        }
                                    };

                                let last_name: String =
                                    match dialoguer::Input::with_theme(&info_theme)
                                        .with_prompt("Last name")
                                        .interact_text()
                                    {
                                        Ok(last_name) => last_name,
                                        Err(_) => {
                                            eprintln!("Failed to read new last name");
                                            continue;
                                        }
                                    };

                                let password: String =
                                    match dialoguer::Password::with_theme(&info_theme)
                                        .with_prompt("Password")
                                        .with_confirmation(
                                            "Repeat password",
                                            "The passwords do not match",
                                        )
                                        .interact()
                                    {
                                        Ok(password) => password,
                                        Err(_) => {
                                            eprintln!("Failed to read new password");
                                            continue;
                                        }
                                    };

                                let admin = match dialoguer::Confirm::with_theme(&info_theme)
                                    .with_prompt(format!(
                                        "Is {} {} an admin?",
                                        first_name, last_name
                                    ))
                                    .default(false)
                                    .interact()
                                {
                                    Ok(admin) => admin,
                                    Err(_) => {
                                        eprintln!("Failed to read admin");
                                        continue;
                                    }
                                };

                                let student = match babibapp
                                    .reset_student_full(
                                        id,
                                        &email,
                                        &first_name,
                                        &last_name,
                                        &password,
                                        Some(admin),
                                    )
                                    .await
                                {
                                    Ok(student) => student,
                                    Err(_) => {
                                        eprintln!("Failed to reset student");
                                        continue;
                                    }
                                };

                                student
                            }
                            _ => {
                                eprintln!("Failed to read valid reset option");
                                continue;
                            }
                        },
                        Err(_) => {
                            eprintln!("Failed to read reset selection");
                            continue;
                        }
                    };

                    println!("Student successfully reset!");
                    babicli::view_student(&StudentView::Full(student));
                }

                Some("make_student_admin") => {
                    let id = if let Some(id) = args.next() {
                        if let Ok(id) = id.parse::<i32>() {
                            id
                        } else {
                            eprintln!("Invalid student id");
                            continue;
                        }
                    } else {
                        if let Ok(id) = dialoguer::Input::<i32>::with_theme(&info_theme)
                            .with_prompt("id")
                            .interact_text()
                        {
                            id
                        } else {
                            eprintln!("Invalid student id");
                            continue;
                        }
                    };

                    let student = match babibapp.make_student_admin(id).await {
                        Ok(student) => student,
                        Err(_) => {
                            eprintln!("Failed to make student admin");
                            continue;
                        }
                    };

                    println!("Student successfully made admin!");
                    babicli::view_student(&StudentView::Full(student));
                }

                Some("delete_student") => {
                    let id = if let Some(id) = args.next() {
                        if let Ok(id) = id.parse::<i32>() {
                            id
                        } else {
                            eprintln!("Invalid student id");
                            continue;
                        }
                    } else {
                        if let Ok(id) = dialoguer::Input::<i32>::with_theme(&info_theme)
                            .with_prompt("id")
                            .interact_text()
                        {
                            id
                        } else {
                            eprintln!("Invalid student id");
                            continue;
                        }
                    };

                    let student = match babibapp.delete_student(id).await {
                        Ok(student) => student,
                        Err(_) => {
                            eprintln!("Failed to delete student");
                            continue;
                        }
                    };

                    println!("Student successfully deleted!");
                    babicli::view_student(&StudentView::Full(student));
                }

                Some("get_teacher") => {
                    let id = if let Some(id) = args.next() {
                        if let Ok(id) = id.parse::<i32>() {
                            id
                        } else {
                            eprintln!("Invalid teacher id");
                            continue;
                        }
                    } else {
                        if let Ok(id) = dialoguer::Input::<i32>::with_theme(&info_theme)
                            .with_prompt("id")
                            .interact_text()
                        {
                            id
                        } else {
                            eprintln!("Invalid teacher id");
                            continue;
                        }
                    };

                    let teacher = match babibapp.get_teacher(id).await {
                        Ok(teacher) => teacher,
                        Err(_) => {
                            eprintln!("Failed to get teacher");
                            continue;
                        }
                    };

                    babicli::view_teacher(&teacher);
                }

                Some("get_all_teachers") => {
                    let teachers = match babibapp.get_all_teachers().await {
                        Ok(teachers) => teachers,
                        Err(_) => {
                            eprintln!("Failed to get all teachers");
                            continue;
                        }
                    };

                    if teachers.is_empty() {
                        println!("No teachers found");
                        continue;
                    }

                    for teacher in &teachers {
                        babicli::view_teacher(teacher);
                        println!();
                    }
                }

                Some("clear") => print!("\x1B[2J\x1B[1;1H"),

                Some("help") => println!("`help` is not yet implemented"),

                Some("exit") | Some("quit") => {
                    println!("Exit.");
                    process::exit(0);
                }

                Some(unknown) => eprintln!("Unknown command: {}", unknown),

                None => eprintln!("Type `exit` to quit or `help` to get help"),
            }
        };
    }
}
