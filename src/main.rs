#[macro_use] extern crate prettytable;

use prettytable::{Table, Row, Cell};

pub mod task;
pub use crate::task::task::Task;
use std::io;
use chrono::prelude::Local;

fn main() {
    println!("Welcome to Rusty Task Manager!\n");

    let mut tasks: Vec<Task> = Vec::new(); // Making tasks mutable

    loop {
        println!("Commands:");
        println!("- add <title> <description> <due_date>");
        println!("- view");
        println!("- complete <task_index>");
        println!("- filter <completed | upcoming>");
        println!("- clear");
        println!("- export");
        println!("- exit");
        println!();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        match parts[0] {
            "add" => {
                if parts.len() == 4 {
                    let id = tasks.len() as u32 + 1;
                    let title = String::from(parts[1]);
                    let description = String::from(parts[2]);
                    let due_date = String::from(parts[3]);

                    tasks.push(Task::new(id, title, description, due_date));
                } else {
                    println!("Invalid input. Use: add <title> <description> <due_date>");
                }
            }
            "view" => {
                let mut table = Table::new();
                table.add_row(row!["ID", "Title", "Description", "Due Date", "Status"]);

                for task in &tasks {
                    let status = if task.completed { "Complete" } else { "Incomplete" };
                    table.add_row(Row::new(vec![
                        Cell::new(&task.id.to_string()),
                        Cell::new(&task.title),
                        Cell::new(&task.description),
                        Cell::new(&task.due_date),
                        Cell::new(status)

                        // task.id,
                        // &task.title,
                        // &task.description,
                        // &task.due_date,
                        // status
                    ]));
                }

                table.printstd();
            }
            "complete" => {
                if parts.len() == 2 {
                    let index: usize = parts[1].parse().expect("Invalid task index");
                    if index > 0 && index <= tasks.len() {
                        tasks[index - 1].complete();
                        println!("Task {} marked as complete.", index);
                    } else {
                        println!("Invalid task index");
                    }
                } else {
                    println!("Invalid input. Use: complete <task_index>");
                }
            }
            "filter" => {
                if parts.len() == 2 {
                    match parts[1] {
                        "completed" => {
                            for (index, task) in tasks.iter().enumerate().filter(|t| t.1.completed) {
                                println!("{}. {} [Complete]", index + 1, task.title);
                            }
                        }
                        "upcoming" => {
                            let today = Local::now().naive_local();
                            for (index, task) in tasks.iter().enumerate().filter(|t| t.1.due_date >= today.to_string()) {
                                let status = if task.completed { "[Complete]" } else { "[Incomplete]" };
                                println!("{}. {} {} (Due: {})", index + 1, task.title, status, task.due_date);
                            }
                        }
                        _ => println!("Invalid filter option."),
                    }
                } else {
                    println!("Invalid input. Use: filter <completed | upcoming>");
                }
            }

            "clear" => {
                tasks.clear(); // Clears all tasks
                println!("All tasks cleared.");
            }
            "exit" => {
                println!("Exiting Rusty Task Manager. Goodbye!");
                break;  // Exit the loop and end the program
            }
            _ => println!("Invalid command."),
        }
    }
}
