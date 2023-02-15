// use neo4rs::*;
use colored::*;
use std::io;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

// [WalkDir]
// Helper function to detect if the file/folder supposed to be hidden
// (in Linux system; '.filename') then skip the file if it is.
fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| entry.depth() == 0 || !s.starts_with("."))
        .unwrap_or(false)
}

fn main() {
    // The divider line, for readability
    let line_divider: &str = "================================================";

    let root_directory: String = loop {
        println!("{}", line_divider.white().bold());
        println!("Please input the {}: ", "Library Directory".purple().bold());
        println!("(example: {} )", "D:/path/to/folder".italic());
        println!("{}", line_divider.white().bold());

        // Receive the Directory input from user
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input");

        // Clean up input, just to be sure
        // then store into a new String variable
        let user_input: String = user_input.trim().to_string();

        // Check if the path exist.
        // If so, return the path to 'root_directory' variable
        if Path::new(&user_input).exists() {
            println!("{}", line_divider.green());
            println!("The path seems {}! Continue...", "correct".green());
            println!("{}", line_divider.green());
            break user_input;
        } else {
            println!("{}", line_divider.red());
            println!(
                "The folder {}! Please re-check and try again.",
                "doesn't exist".red()
            );
            println!("{}", line_divider.red());
        }
    };

    // RunTime Logger
    use std::time::Instant;
    let now = Instant::now();

    // Traverse the directory recurcively, then save filename & path to the DataBase
    WalkDir::new(root_directory)
        .into_iter()
        .filter_entry(|e| is_not_hidden(e))
        .filter_map(|v| v.ok())
        .for_each(|x| println!("{}", x.path().display()));

    // RunTime Logger
    let elapsed = now.elapsed();
    println!("{}", line_divider.white().bold());
    println!("    The Process took: {:.2?} to completed.", elapsed);
    println!("{}", line_divider.white().bold());
}
