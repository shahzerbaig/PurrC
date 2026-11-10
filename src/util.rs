use crate::file;
use crate::lexer;
use crate::repl;
use std::env;
use std::io;
use std::io::Write;
pub fn check_input() {
    let args: Vec<String> = env::args().collect();

    // check is the arguments (File) is passed.
    if args.len() <= 2 {
        loop {
            // Go into repl
            print!(">> ");
            io::stdout().flush().unwrap();
            let repl_env = repl::repl();

            let take_input = &repl_env.get_input;
            let choice = take_input;

            println!("{:#?}, is from args", take_input);

            if choice == "exit" {
                break;
            }
        }
    } else {
        // reads the file and invoke the lexer
        let filename = match file::read_file_name() {
            Ok(name) => name,
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1); // or exit(0) if you truly want "success"
            }
        };
        lexer::read_char();

        println!("{}", filename);
        let input = file::read_file_content(filename);

        //let mut lexer = lexer::Lexer::new(&input);
    }
}
