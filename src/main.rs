#![windows_subsystem = "windows"]

use std::{os::windows::process::CommandExt, process::Command, time::Duration};

use wait_timeout::ChildExt;
use winapi::um::winbase::CREATE_NO_WINDOW;

fn main() {
    let mut child = Command::new("ssh-add")
        .creation_flags(CREATE_NO_WINDOW)
        .args(["-L"])
        .spawn()
        .unwrap();

    let one_sec = Duration::from_secs(2);
    let status_code = match child.wait_timeout(one_sec).unwrap() {
        Some(status) => status.code(),
        None => {
            // child hasn't exited yet
            child.kill().unwrap();
            child.wait().unwrap().code()
        }
    };

    // command got killed
    if status_code != Some(0) {
        println!("restarting gpg-agent");
        Command::new("gpgconf")
            .creation_flags(CREATE_NO_WINDOW)
            .args(["--kill", "gpg-agent"])
            .spawn()
            .unwrap()
            .wait()
            .expect("killing gpg-agent failed");

        Command::new("gpgconf")
            .args(["--launch", "gpg-agent"])
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
            .unwrap()
            .wait()
            .expect("launching gpg-agent failed");
    }
}
