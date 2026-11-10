use std::{
    io::{self, Write},
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug)]
pub struct ReplEnv {
    pub env: Environment,
    pub created_at: u64,
    pub get_input: String,
}
#[derive(Debug)]
pub struct Environment {
    name: String,
}

pub fn repl() -> ReplEnv {
    // Time it is created
    let created_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System clock is before Unix epoch")
        .as_secs();

    // Creating an empty Environment object
    let environment = create_empty_enviroment();

    fn get_input() -> String {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();

        input.to_string()
    }
    ReplEnv {
        env: environment,
        created_at,
        get_input: get_input(),
    }
}

fn create_empty_enviroment() -> Environment {
    Environment {
        name: String::new(),
    }
}
