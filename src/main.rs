use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use pathdiff::diff_paths;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::io;
use std::path::Path;
use std::{error::Error, fs::create_dir_all};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::Result as db_Result;
use surrealdb::Surreal;
use tokio::runtime::Runtime;
use walkdir::WalkDir;

static DB: Surreal<Client> = Surreal::init();

#[derive(Serialize, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

#[derive(Serialize, Deserialize)]
struct Image {
    name: String,
    path: String,
    deleted: bool,
}

#[derive(Serialize, Deserialize)]
struct Folder {
    name: String,
    path: String,
    deleted: bool,
}
// Save Directory Structure into Database
async fn store_dir(path: &Path) -> db_Result<()> {
    let name: String = path.file_name().unwrap().to_string_lossy().to_string();
    let full_path: String = path.display().to_string();

    let _created: Folder = DB
        .create("folder")
        .content(Folder {
            name: name,
            path: full_path,
            deleted: false,
        })
        .await?;
    Ok(())
}

// Save File Path into Database
async fn store_file(path: &Path) -> db_Result<()> {
    let name: String = path.file_name().unwrap().to_string_lossy().to_string();
    let full_path: String = path.display().to_string();

    let _created: Image = DB
        .create("image")
        .content(Image {
            name: name,
            path: full_path,
            deleted: false,
        })
        .await?;
    Ok(())
}

// (Re)Create Folder Structure for the thumbnails
fn create_folder(path: &Path, root: &str, dest: &Path) -> std::io::Result<()> {
    let relative_path = diff_paths(path, &root);
    create_dir_all(dest.join(relative_path.unwrap()))?;
    Ok(())
}

// Generate thumbnail from the image file
fn create_thumbnail(path: &Path, root: &str, dest: &Path) -> Result<(), Box<dyn Error>> {
    let relative_path = diff_paths(path, &root);
    let img = image::open(path)?;
    // TODO #{4} Specify thumbnail size?
    Ok(img
        .thumbnail(200, 200)
        .save(dest.join(relative_path.unwrap()))?)
}

// Loop to get 'proper' directory input from user
fn get_directory(dir_type: &str) -> String {
    let divider: &str = "================================================";
    let lib_name;
    match dir_type {
        "src" => lib_name = "Image Library",
        "des" => lib_name = "Thumbnails",
        _ => lib_name = "ERROR!",
    }
    let lib_dir: String = loop {
        println!("{}", divider.blue().bold());
        println!("Please enter {} path: ", lib_name.bright_purple());
        println!("(example: {} )", "D:\\path\\to\\folder".italic());
        println!("{}", divider.blue().bold());

        // Receive the input
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input");

        let user_input: String = user_input.trim().to_string();

        // Check if the input directory exist. If not, re-enter.
        if Path::new(&user_input).exists() {
            println!("{}", divider);
            println!(
                "{} folder {}! Continue...",
                lib_name.green(),
                "exist".green()
            );
            println!("{}", divider);
            break user_input;
        } else {
            // for 'Image Library', make sure the directory does exist.
            if dir_type == "src" {
                println!("{}", divider);
                println!(
                    "{} folder {}! Please re-check.",
                    lib_name.red(),
                    "doesn't exist".red()
                );
                println!("{}", divider);
            } else {
                // but for 'Thumbnails', doesn't matter, it will be created anyway.
                println!("{}", divider);
                println!(
                    "{} folder will be create at: {}",
                    lib_name.green(),
                    user_input.clone().as_str().green()
                );
                println!("{}", divider);
                break user_input;
            }
        }
    };
    return lib_dir;
}

// Check extension of a file (code from StackOverflow)
pub trait FileExtension {
    fn has_extension<S: AsRef<str>>(&self, extensions: &[S]) -> bool;
}

impl<P: AsRef<Path>> FileExtension for P {
    fn has_extension<S: AsRef<str>>(&self, extensions: &[S]) -> bool {
        if let Some(ref extension) = self.as_ref().extension().and_then(OsStr::to_str) {
            return extensions
                .iter()
                .any(|x| x.as_ref().eq_ignore_ascii_case(extension));
        }

        false
    }
}

#[tokio::main]
async fn main() -> db_Result<()> {
    // Divider line, for readability
    let divider: &str = "================================================";

    // Get path for original image files
    let lib_dir = get_directory("src");
    let lib_root = lib_dir.clone();
    // Get path for the thumbnail files
    let dest_srt = get_directory("des");
    let dest_root = Path::new(&dest_srt);

    // RunTime Logger
    use std::time::Instant;
    let now = Instant::now();

    // Progress Bar
    let mut num_files = 0;
    let root_pb = lib_dir.clone();
    WalkDir::new(root_pb)
        .into_iter()
        .filter_map(|v| v.ok())
        .for_each(|x| {
            if x.path()
                .has_extension(&["png", "jpg", "jpeg", "gif", "bmp"])
            {
                num_files = num_files + 1;
            }
        });

    let pb = ProgressBar::new(num_files);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}) {percent}%",
        )
        .unwrap(),
    );

    // Connect to SurrealDB
    DB.connect::<Ws>("127.0.0.1:8000").await?;

    // Sign-in to SurrealDB
    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    // Select a namespace / database
    DB.use_ns("cauldron_thumbnail")
        .use_db("cauldron_thumbnail")
        .await?;

    // Traverse the Library directory, save structure to SurrealDB, and generate the thumbnails
    WalkDir::new(lib_dir)
        .into_iter()
        .par_bridge()
        .filter_map(|v| v.ok())
        .for_each(|x| {
            if x.path().is_dir() {
                // Folder:
                match create_folder(x.path(), &lib_root, dest_root) {
                    Ok(_value) => {
                        let rt = Runtime::new().unwrap();
                        let future = store_dir(x.path());
                        let _r = rt.block_on(future);
                    }
                    Err(_error) => {
                        // println!("ERROR: {}", error.to_string());
                        // TODO #{6} Handle Errors properly
                    }
                }
            } else {
                // File:
                match create_thumbnail(x.path(), &lib_root, dest_root) {
                    Ok(_value) => {
                        let rt = Runtime::new().unwrap();
                        let future = store_file(x.path());
                        let _r = rt.block_on(future);
                        pb.inc(1); // Progress Bar
                    }
                    Err(_error) => {
                        // println!("ERROR: {}", error.to_string());
                        // TODO #{6} Handle Errors properly
                    }
                }
            }
        });

    pb.finish_with_message("Done.");

    // RunTime Logger
    let elapsed = now.elapsed();
    println!("{}", divider.bold());
    println!(
        "{} Process took: {} seconds to complete.",
        "Done!".green(),
        elapsed.as_secs().to_string().green()
    );
    println!("{}", divider.bold());

    dont_disappear::any_key_to_continue::default();

    Ok(())
}

// TODO  #{4} let user specify thumbnail sizes (as options; s:150px, m:200px, l:250px, etc.)
// TODO  #{6} Handle Errors properly (collect all, then print afterward)
