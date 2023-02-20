use colored::*;
use pathdiff::diff_paths;
use rayon::prelude::*;
use std::io;
use std::path::Path;
use std::{error::Error, fs::create_dir_all};
use walkdir::WalkDir;
// use neo4rs::*;

/*
    Path:
        Name      = x.path().file_name().unwrap().to_str().unwrap()
        Parent    = x.path().parent().unwrap().to_str().unwrap()
        Full-Path = x.path().display().to_string()
*/

// [Neo4J] Save Directory into Database
fn add_directory_node(path: &Path) {
    // let _name: &str = path.file_name().unwrap().to_str().unwrap();
    // let _parent: &str = path.parent().unwrap().to_str().unwrap();
    let full_path: String = path.display().to_string();

    println!("DIR : {}", full_path.blue());
    // TODO! #{1} Insert folder into Database
}

// [Neo4J] Save Filename into Database
fn add_file_node(path: &Path) {
    let name: &str = path.file_name().unwrap().to_str().unwrap();
    // let _parent: &str = path.parent().unwrap().to_str().unwrap();
    // let _full_path: String = path.display().to_string();

    println!("File: {}", name);
    // TODO! #{2} Insert file into Database
}

// Create Folder for the thumbnails
fn create_folder(path: &Path, root: &str, dest: &Path) -> std::io::Result<()> {
    let relative_path = diff_paths(path, &root);
    create_dir_all(dest.join(relative_path.unwrap()))?;
    Ok(())
}

// Generate thumbnail for the image file
fn create_thumbnail(path: &Path, root: &str, dest: &Path) -> Result<(), Box<dyn Error>> {
    let relative_path = diff_paths(path, &root);
    let img = image::open(path)?;
    // TODO #{4} Specify thumbnail size?
    Ok(img
        .thumbnail(200, 200)
        .save(dest.join(relative_path.unwrap()))?)
}

fn main() {
    // Divider line, for readability
    let divider: &str = "================================================";

    // Loop to get 'proper' directory input from user
    let lib_dir: String = loop {
        println!("{}", divider.purple().bold());
        println!("Please input the {}: ", "Library Directory".purple().bold());
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
            println!("The folder {}! Continue...", "exist".green());
            println!("{}", divider.green());
            break user_input;
        } else {
            println!("{}", divider.red());
            println!("The folder {}! Please re-check.", "doesn't exist".red());
            println!("{}", divider.red());
        }
    };

    // RunTime Logger
    use std::time::Instant;
    let now = Instant::now();

    // Library are original files, Destination is for the thumbnails
    let lib_root = lib_dir.clone();
    // TODO! #{3} receive thumbnail path input from user?
    let dest_root = Path::new("D:\\Pictures\\thumbnail");

    // Traverse the Library directory, save structure to Database, and generate the thumbnails
    WalkDir::new(lib_dir)
        .into_iter()
        .par_bridge()
        .filter_map(|v| v.ok())
        .for_each(|x| {
            if x.path().is_dir() {
                // Folder:
                match create_folder(x.path(), &lib_root, dest_root) {
                    Ok(_value) => {
                        add_directory_node(x.path()); // to Database
                    }
                    Err(error) => {
                        println!("ERROR: {}", error.to_string());
                    }
                }
            } else {
                // File:
                match create_thumbnail(x.path(), &lib_root, dest_root) {
                    Ok(_value) => {
                        add_file_node(x.path()); // to Database
                    }
                    Err(error) => {
                        println!(
                            "ERROR:{} : {}",
                            x.path().file_name().unwrap().to_str().unwrap().red(),
                            error.to_string()
                        );
                    }
                }
            }
        });

    // RunTime Logger
    let elapsed = now.elapsed();
    println!("{}", divider.purple().bold());
    println!(
        "    Process took: {} seconds to complete.",
        elapsed.as_secs().to_string().red()
    );
    println!("{}", divider.purple().bold());
}

// TODO! List
// TODO! #{1} save folder structure into Database
// TODO! #{2} save (only image) file into Database
// TODO! #{3} receive user input for Thumbnail directory
// TODO  #{4} let user specify thumbnail sizes (as options; s:150px, m:200px, l:250px, etc.)
// TODO  #{5} refactor user input parts (both Library and Thumbnail) into its own function
