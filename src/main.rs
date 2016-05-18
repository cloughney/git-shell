extern crate chrono;

use std::process;
use std::process::Command;

mod log;
mod validation;


fn main() {
	let args: Vec<String> = std::env::args().collect();
	let command = match option_env!("SSH_ORIGINAL_COMMAND") {
    	Some(value) => value,
    	None => exit("SSH_ORIGINAL_COMMAND does not exist.")
    };

	let input: validation::ValidatedInput = 
		match validation::InputValidator::new()
			.arguments(args)
			.incoming_command(command)
			.validate() {
			Ok(input) => input,
			Err(message) => exit(&message)
		};

    let mut shell_process = Command::new("git-shell")
    	.arg("-c")
    	.arg(input.original_command)
    	.spawn()
    	.unwrap_or_else(|e| {
	    	exit(&format!("Failed to spawn git shell process: '{}'", e));
	    });

    shell_process
    	.wait()
    	.unwrap_or_else(|e| {
    		exit(&format!("Failed to wait on git shell process: '{}'", e));
    	});
}

fn exit(message: &str) -> ! {
	log::debug(message);
	process::exit(0);
}