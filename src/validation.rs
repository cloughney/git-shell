use std::fs;

pub struct ValidatedInput {
	pub original_command: String,
	pub command: String,
	pub repository: String,
	pub user: String
}

pub struct InputValidator {
	original_command: String,
	args: Vec<String>
}

impl InputValidator {
	pub fn new() -> InputValidator {
		InputValidator {
			original_command: "".to_string(),
			args: vec![]
		}
	}

	pub fn arguments(mut self, args: Vec<String>) -> InputValidator {
		self.args = args;
		self
	}

	pub fn incoming_command(mut self, command: &str) -> InputValidator {
		self.original_command = command.to_string();
	    self
	}

	pub fn validate(self) -> Result<ValidatedInput, String> {
		let (username, command, mut repository);

		match validate_args(self.args) {
			Ok(clean_username) => username = clean_username,
			Err(message) => return Err(message)
		}

		match validate_command(&self.original_command) {
			Ok((clean_command, dirty_repo)) => {
				command = clean_command;
				repository = dirty_repo;
			},
			Err(message) => return Err(format!("User: '{}' - {}", username, message))
		}

		match validate_repo(&repository) {
			Ok(clean_repo) => repository = clean_repo,
			Err(message) => return Err(format!("User: '{}' - {}", username, message))
		}

		Ok(ValidatedInput {
			original_command: self.original_command.clone(),
			command: command.clone(),
			repository: repository.clone(),
			user: username.clone()
		})
	}
}

fn validate_args(args: Vec<String>) -> Result<String, String> {
	if args.len() != 2 {
		return Err("Incorrect number of arguments passed.".to_string());
	}

	Ok(args[1].to_string())
}

fn validate_command(command: &str) -> Result<(String, String), String> {
	let valid_commands = vec!["git-upload-pack", "git-receive-pack", "git-upload-archive"];
    
    let input: Vec<&str> = command.split(" ").collect();
    if input.len() != 2 {
    	return Err(format!("Incoming command is not valid '{:?}'", command));
    }

    let mut is_valid_command = false;
    for valid_command in &valid_commands {
    	if valid_command == &input[0] {
    		is_valid_command = true;
    		break;
    	}
    }

    if !is_valid_command {
    	return Err("Incoming command is not a valid git command".to_string());
    }

    Ok((input[0].to_string(), input[1].to_string()))
}

fn validate_repo(repository: &str) -> Result<String, String> {
	let len = repository.len();
	let clean_repo = &repository.clone()[1..len-1];

	if clean_repo.starts_with("/") {
		return Err(format!("Git repository may not be an absolute path. '{}'", clean_repo));
	}

	if let Err(_) = fs::metadata(&clean_repo) {
		return Err(format!("Invalid git repository. '{}'", clean_repo));
	}

	Ok(clean_repo.to_string())
}