use std::{env, process};

use dialoguer::theme::ColorfulTheme;

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
    println!("Use `exit` to quit the program or ask for `help`.");
    println!("Use the Up/Down arrows to scroll through history.");
    println!("Use the Right arrow or Tab to complete your command.");

    let mut history = BabicliHistory::default();
    let completion = BabicliCompletion::new(&vec![
        "validate_token".to_string(),
        "get_student".to_string(),
        "get_self".to_string(),
        "get_all_students".to_string(),
        "clear".to_string(),
        "help".to_string(),
        "exit".to_string(),
        "quit".to_string(),
    ]);

    loop {
        println!();

        let cmd_theme = ColorfulTheme::default();
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
                    if let Some(id) = args.next() {
                        if let Ok(id) = id.parse::<i32>() {
                            let student = match babibapp.get_student(id).await {
                                Ok(student) => student,
                                Err(_) => {
                                    eprintln!("Failed to get student");
                                    continue;
                                }
                            };

                            babicli::view_student(&student);
                        } else {
                            eprintln!("Invalid student id");
                        }
                    } else {
                        eprintln!("Please pass a student id");
                    }
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
