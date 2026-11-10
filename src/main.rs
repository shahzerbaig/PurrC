use std::error::Error;

mod file;
mod lexer;
mod repl;
mod token;
mod util;
fn main() -> Result<(), Box<dyn Error>> {
    println!("Monkey Lang");
    util::check_input();
    //file::print_file_content(file_name.clone());

    // 1. Read Monkey source [DONE]
    // 2. Lex it
    // 3. Parse it
    // 4. Compile it
    // 5. Output compiled code

    Ok(())
}
