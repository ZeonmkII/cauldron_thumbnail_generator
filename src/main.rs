use colored::*;
// use neo4rs::*;
// use image::imageops::FilterType;
// use image::ImageError;
use pathdiff::diff_paths;
// use rayon::prelude::*;
use std::fs::create_dir_all;
use std::io;
use std::path::Path;
use walkdir::{DirEntry, WalkDir};

// [WalkDir] We'll skip the file if it is supposed to be hidden
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
*/

// [Neo4J] Save Directory into Database
fn add_directory_node(path: &Path) {
    let _name: &str = path.file_name().unwrap().to_str().unwrap();
    let _parent: &str = path.parent().unwrap().to_str().unwrap();
    let full_path: String = path.display().to_string();

    println!("DIR : {}", full_path.blue());
    // TODO! Insert folder into Database
}

// [Neo4J] Save Filename into Database
fn add_file_node(path: &Path) {
    let name: &str = path.file_name().unwrap().to_str().unwrap();
    let _parent: &str = path.parent().unwrap().to_str().unwrap();
    let _full_path: String = path.display().to_string();

    println!("File: {}", name);
    // TODO! Insert file into Database
}

// Create Folder for the thumbnails
fn create_folder(path: &Path, root: &str, dest: &Path) -> std::io::Result<()> {
    let relative_path = diff_paths(path, &root);
    create_dir_all(dest.join(relative_path.unwrap()))?;
    Ok(())
}

// Generate thumbnail for the image file
fn create_thumbnail(path: &Path, root: &str, dest: &Path) -> std::io::Result<()> {
    // TODO! Implement thumbnail generator function
    // 1. check if file is image type
    // 2. generate thumbnail according to size_input
    Ok(())
}

fn main() {
    // Divider line, for readability
    let divider: &str = "================================================";

    // Loop to get 'proper' directory input from user
    let lib_dir: String = loop {
        println!("{}", divider.purple().bold());
        println!(
            "Please input the {}: ",
            "Image Library Directory".purple().bold()
        );
        println!("(example: {} )", "D:\\path\\to\\folder".italic());
        println!("{}", divider.purple().bold());

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

    // RunTime Logger
    use std::time::Instant;
    let now = Instant::now();

    // Library are original files, Destination is for the thumbnails
    let lib_root = lib_dir.clone();
    // TODO! receive thumbnail path input from user?
    let dest_root = Path::new("D:\\Pictures\\thumbnail");

    // Traverse the Library directory, save structure to Database, and generate the thumbnails
    WalkDir::new(lib_dir)
        .into_iter()
        .filter_entry(|e| is_not_hidden(e))
        .filter_map(|v| v.ok())
        .for_each(|x| {
            // Is it a File or Folder?
            if x.path().is_dir() {
                // Folder: create empty folder, keeping the same directory structure
                match create_folder(x.path(), &lib_root, dest_root) {
                    Ok(_value) => {
                        add_directory_node(x.path()); // add to Database
                    }
                    Err(error) => {
                        println!("Error: {}", error.to_string());
                    }
                }
            } else {
                // File: generate thumbnail for image file, into the prepared folder
                match create_thumbnail(x.path(), &lib_root, dest_root) {
                    Ok(_value) => {
                        add_file_node(x.path()); // add to Database
                    }
                    Err(error) => {
                        println!("Error: {}", error.to_string());
                    }
                }
            }
        });

    // RunTime Logger
    let elapsed = now.elapsed();
    println!("{}", divider.white().bold());
    println!("    The Process took: {:.2?} to completed.", elapsed);
    println!("{}", divider.white().bold());
}
