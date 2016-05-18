use std::fs::OpenOptions;
use std::io::prelude::*;

use chrono::*;

pub fn debug(message: &str) {
	let mut file = OpenOptions::new()
		.create(true)
		.append(true)
		.open("git-shell.log")
		.expect("Failed to open log file");

	let now = UTC::now().format("%Y-%m-%d %H:%M:%S");

	writeln!(file, "[{}]\t{}", now, message)
		.expect("Failed to write to log file");
}