use colored::*;
// use neo4rs::*;
use image::imageops::FilterType;
use image::ImageError;
use rayon::prelude::*;
use std::fs::{create_dir_all, metadata};
use std::io;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

// [WalkDir]
// Helper function to detect if the file/folder supposed to be hidden
fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| entry.depth() == 0 || !s.starts_with("."))
        .unwrap_or(false)
}

/*
    Path:
            Name      = x.path().file_name().unwrap().to_str().unwrap()
            Parent    = x.path().parent().unwrap().to_str().unwrap()
            Full-Path = x.path().display().to_string()
            TODO ...just the path without drive letter ? (for thumbnail)
*/

// [Neo4J] Save Directory name into Database
fn add_directory_node(path: &Path) {
    let _name: &str = path.file_name().unwrap().to_str().unwrap();
    let _parent: &str = path.parent().unwrap().to_str().unwrap();
    let full_path: String = path.display().to_string();

    println!("DIR : {}", full_path.blue());

    // TODO! Insert into Database
    /*
    [Neo4j] Connection
    let url = "127.0.0.1:7687";
    let user = "neo4j";
    let pass = "realplayer";
    let graph = Graph::new(url, user, pass).await.unwrap();

    graph
        .run(
            query("CREATE (f:Folder { name: $name, path: $path })")
                .param("name", dir_name)
                .param("path", full_path),
        )
        .await
        .unwrap();
    */
}

// [Neo4J] Save filename into Database
fn add_file_node(path: &Path) {
    let _name: &str = path.file_name().unwrap().to_str().unwrap();
    let _parent: &str = path.parent().unwrap().to_str().unwrap();
    let full_path: String = path.display().to_string();

    println!("File: {}", full_path);
    // TODO! Insert into Database
}

fn main() {
    // The divider line, for readability
    let divider: &str = "================================================";

    // Loop to get 'proper' directory input from user
    let lib_dir: String = loop {
        println!("{}", divider.white().bold());
        println!("Please input the {}: ", "Root Directory".purple().bold());
        println!("(example: {} )", "D:\\path\\to\\folder".italic());
        println!("{}", divider.white().bold());

        // Receive the input
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input");

        // Clean up input, then store into a new String variable
        let user_input: String = user_input.trim().to_string();

        // Check if the input directory exist.
        if Path::new(&user_input).exists() {
            println!("{}", divider.green());
            println!("The path seems {}! Continue...", "correct".green());
            println!("{}", divider.green());
            break user_input;
        } else {
            println!("{}", divider.red());
            println!(
                "The folder {}! Please re-check and try again.",
                "doesn't exist".red()
            );
            println!("{}", divider.red());
        }
    };

    // TODO! receive thumbnail path input from user?
    let thumb_dir = std::path::Path::new("D:\\Pictures\\tn");
    println!(
        "Saving thumbnails into '{}'...",
        thumb_dir.display().to_string().red()
    );

    // RunTime Logger
    use std::time::Instant;
    let now = Instant::now();

    // Traverse the directory, save structure to Database, and generate the thumbnails
    WalkDir::new(lib_dir)
        .into_iter()
        .filter_entry(|e| is_not_hidden(e))
        .filter_map(|v| v.ok())
        .for_each(|x| {
            // Check if it's a File or Folder
            let current = metadata(x.path()).unwrap();
            if current.is_dir() {
                // Directory:
                add_directory_node(x.path());
            } else {
                // File:
                add_file_node(x.path());
                // TODO! Generate thumbnails function
                // generate_thumbnail();
            }
        });

    // RunTime Logger
    let elapsed = now.elapsed();
    println!("{}", divider.white().bold());
    println!("    The Process took: {:.2?} to completed.", elapsed);
    println!("{}", divider.white().bold());
}
