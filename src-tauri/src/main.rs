// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::process;

#[cfg(unix)]
use std::os::unix::process::ExitStatusExt;

#[cfg(windows)]
use std::os::windows::process::ExitStatusExt;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn perform_command(command: String) -> String {

    if command.starts_with("cd ") {
        let directory = command.trim_start_matches("cd ");
        change_dir(directory.to_string());
        return "".to_string();
    }

    let output = if cfg!(target_os = "windows") {
        process::Command::new("cmd")
            .args(["/C", &command])
            .output()
    } else {
        process::Command::new("sh")
        .arg("-c")
        .arg(&command)
        .output()
    };


    match output {
        Ok(output) => {

            let use_output = match output.status.code() {
                Some(code) => {
                    if code == 0 {
                        output.stdout
                    } else {
                        output.stderr
                    }
                },
                None => {
                    output.stderr
                }
            };

            let output = String::from_utf8_lossy(&use_output);
            let output = output.trim();
            return output.to_string();
        },
        Err(error) => {
            println!("Error: {}", error);
            return format!("Error: {}", error);
        }
    };

}

fn change_dir(directory: String) {
    if directory == ".."  {
        let current_dir = env::current_dir()
            .expect("Failed to get current directory");

        let current_dir = current_dir.to_string_lossy();
        let current_dir = current_dir.to_string();

        let current_dir = current_dir.split("/").collect::<Vec<&str>>();
        let current_dir = current_dir[..current_dir.len()-1].join("/");

        env::set_current_dir(&current_dir)
            .expect("Failed to set current directory");

        return;
    }

    env::set_current_dir(&directory)
        .expect("Failed to set current directory");
}

#[tauri::command]
fn get_username() -> String {
    let output = process::Command::new("whoami")
    .output()
    .unwrap_or(process::Output {
        status: process::ExitStatus::from_raw(0),
        stdout: "Command not found".as_bytes().to_vec(),
        stderr: Vec::new(),
    });

    let output = String::from_utf8_lossy(&output.stdout);
    let output = output.trim();

    return output.to_string();
}

#[tauri::command]
fn get_current_directory() -> String {
    let current_dir = env::current_dir()
        .expect("Failed to get current directory");

    current_dir.to_string_lossy().to_string()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_username, perform_command, get_current_directory])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
