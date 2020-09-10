use regex::Regex;
use std::fs;
use std::io;
use std::process::Command;

use users::{get_current_uid, get_user_by_uid};

struct Choice {
    name: String,
    mount_and_capacity: String,
}

fn main() {
    println!(
        "\n/!\\ WARNING : THIS PROGRAM WILL MAKE PERMANENT CHANGES TO THE TARGETED DRIVE /!\\\n"
    );
    let output = Command::new("sudo")
        .arg("fdisk")
        .arg("-l")
        .output()
        .expect("failed to execute process");
    let str_out = String::from_utf8_lossy(&output.stdout);
    let choices = get_choices(&str_out);

    match choices {
        Some(c) => show_menu(&c),
        None => println!("Aborting"),
    }
}

fn show_menu(choices: &std::vec::Vec<Choice>) {
    let user = get_user_by_uid(get_current_uid()).unwrap();

    let paths = fs::read_dir(format!("/home/{}/Downloads", user.name().to_string_lossy())).unwrap();

    let mut vpath = Vec::new();

    for path in paths {
        match check_extension(&path.unwrap()) {
            Some(file_path) => vpath.push(file_path),
            None => (),
        }
    }

    for (v_number, v) in vpath.iter().enumerate() {
        println!("{}) {}", v_number, v);
    }

    let mut user_input: String = "".to_string();
    get_string_user_input(&mut user_input, "Pick an iso file");

    match user_input.parse::<usize>() {
        Ok(uz) => {
            user_input = "".to_string();
            for (c_number, choice) in choices.iter().enumerate() {
                println!(
                    "{}) {} |{}",
                    c_number, choice.mount_and_capacity, choice.name
                );
            }
            get_string_user_input(&mut user_input, "Pick a drive");
            match user_input.parse::<usize>() {
                Ok(uz2) => {
                    user_input = "".to_string();
                    let iso_file_name = &vpath[uz];
                    let destination_drive =
                        &clean_path_from_mount_and_capacity(&choices[uz2].mount_and_capacity);
                    get_string_user_input(
                        &mut user_input,
                        format!("Put {} on {} ? y/n", iso_file_name, destination_drive).as_str(),
                    );

                    if user_input == "y" {
                        let output = Command::new("sudo")
                            .arg("dd")
                            .arg("bs=4M")
                            .arg("if=".to_string() + iso_file_name)
                            .arg("of=".to_string() + destination_drive)
                            .output()
                            .expect("failed to execute process");
                        let str_out = String::from_utf8_lossy(&output.stdout);
                        println!("{}", str_out);
                    } else {
                        println!("Aborting");
                    }
                }
                Err(e) => invalid_user_input(e, user_input),
            };
        }
        Err(e) => invalid_user_input(e, user_input),
    };
}

fn invalid_user_input(e: std::num::ParseIntError, user_input: String) {
    println!("Invalid user input : {}", user_input);
    println!("Error : {}", e);
}

fn get_string_user_input(user_input: &mut String, message: &str) {
    println!("{}", message);
    io::stdin()
        .read_line(user_input)
        .expect("error: unable to read user input");
    user_input.pop();
}

fn clean_path_from_mount_and_capacity(mount_and_capacity: &String) -> String {
    let re = Regex::new(r"/.*/.*:").unwrap();
    let mut r_str = "".to_string();
    for r in re.captures_iter(mount_and_capacity) {
        r_str = r[0].to_string();
        r_str.pop();
    }

    r_str
}

fn check_extension(p: &std::fs::DirEntry) -> Option<String> {
    let mut file_path = "".to_string();

    match p.path().extension() {
        Some(c) => {
            if c.to_str().unwrap() == "iso" {
                file_path = p.path().to_str().unwrap().to_string();
            }
        }
        None => (),
    }

    if file_path.is_empty() {
        None
    } else {
        Some(file_path)
    }
}

fn get_choices(str_out: &std::borrow::Cow<str>) -> Option<Vec<Choice>> {
    let mut counter = 1;
    let mut vec = Vec::new();

    for line in str_out.lines() {
        let re = Regex::new(r"Disk .*:").unwrap();
        if re.is_match(line) {
            if counter % 3 != 0 {
                vec.push(line);
            }
            counter += 1;
        }
    }

    let mut mount_and_capacity = Vec::new();
    let mut names = Vec::new();

    for (l_number, l) in vec.iter().enumerate() {
        // This is a line with the mounting point and the capacity of the drive
        if l_number % 2 == 0 {
            mount_and_capacity.push(l.split(",").next().unwrap());
        }
        // This is a line with the name of the drive
        else {
            names.push(l.split(":").last().unwrap());
        }
    }
    // If these two are not the same length, we got a problem, abort
    if mount_and_capacity.len() != names.len() {
        println!("Error : we were not able to parse fdisk -l properly.");
        None
    } else {
        let mut choices = Vec::new();
        for (n_name, n) in names.iter().enumerate() {
            choices.push(Choice {
                name: n.to_string(),
                mount_and_capacity: mount_and_capacity[n_name].to_string(),
            });
        }
        Some(choices)
    }
}
