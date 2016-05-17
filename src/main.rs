use std::env;
use std::process;
use std::process::Command;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::prelude::*;

fn main() {
	let valid_commands = vec!["git-upload-pack", "git-receive-pack", "git-upload-archive"];

	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		exit("Incorrect number of arguments passed.");
	}

	let user = &args[1];
	
    let command = match env::var("SSH_ORIGINAL_COMMAND") {
    	Ok(value) => value,
    	Err(_) => exit("SSH_ORIGINAL_COMMAND is empty.")
    };
    //let command = "git-upload-pack something";

    let command: Vec<&str> = command.split(" ").collect();
    if command.len() != 2 {
    	panic!("Incoming command was wrong and junk.");
    }

    let mut real_command = "";

    for valid_command in &valid_commands {
    	if valid_command == &command[0] {
    		real_command = valid_command;
    		break;
    	}
    }

    let output = Command::new(real_command).arg(&command[1]).output().unwrap_or_else(|e| {
    	exit("Failed to run command");
    });

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn log(message: &str) {
	let mut file = OpenOptions::new()
		.create(true)
		.append(true)
		.open("git-shell.log")
		.expect("Failed to open log file");

	writeln!(file, "{}", message)
		.expect("Failed to write to log file");
}

fn exit(message: &str) -> ! {
	log(message);
	process::exit(0);
}