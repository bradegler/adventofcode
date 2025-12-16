use chrono::{Datelike, Local};
use clap::Parser;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day of the Advent of Code puzzle (default: today's day)
    #[arg(index = 1)]
    day: Option<u32>,

    /// Year of the Advent of Code puzzle (default: today's year)
    #[arg(index = 2)]
    year: Option<i32>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let now = Local::now();

    let year = args.year.unwrap_or_else(|| now.year());
    let day = args.day.unwrap_or_else(|| now.day());

    println!("Generating scaffolding for Year: {}, Day: {}", year, day);

    // Paths
    let src_bin_dir = PathBuf::from("src").join("bin");
    let day_dir_name = format!("aoc{}_{:02}", year, day);
    let day_dir_path = src_bin_dir.join(&day_dir_name);
    let main_rs_path = day_dir_path.join("main.rs");
    
    let testdata_dir = PathBuf::from("testdata").join(year.to_string());
    let testdata_file_name = format!("{}_{:02}.txt", year, day);
    let testdata_file_path = testdata_dir.join(&testdata_file_name);

    // Create directories
    if !day_dir_path.exists() {
        fs::create_dir_all(&day_dir_path)?;
        println!("Created directory: {:?}", day_dir_path);
    }

    if !testdata_dir.exists() {
        fs::create_dir_all(&testdata_dir)?;
        println!("Created directory: {:?}", testdata_dir);
    }

    // Create testdata file
    if !testdata_file_path.exists() {
        fs::File::create(&testdata_file_path)?;
        println!("Created empty file: {:?}", testdata_file_path);
    } else {
        println!("File already exists: {:?}", testdata_file_path);
    }

    // Generate main.rs
    let template_path = Path::new("gen").join("main_template.rs");
    if template_path.exists() {
        let template_content = fs::read_to_string(&template_path)?;
        let new_content = template_content
            .replace("%%YEAR%%", &year.to_string())
            .replace("%%DAY%%", &day.to_string());
        
        // Only write if it doesn't exist to avoid overwriting work
        if !main_rs_path.exists() {
            let mut file = fs::File::create(&main_rs_path)?;
            file.write_all(new_content.as_bytes())?;
            println!("Generated: {:?}", main_rs_path);
        } else {
            println!("File already exists, skipping generation: {:?}", main_rs_path);
        }
    } else {
        eprintln!("Warning: Template file not found at {:?}", template_path);
    }

    // Run 'agy' command
    println!("Running 'agy'...");
    let status = Command::new("agy")
        .arg(main_rs_path.to_str().unwrap())
        .arg(testdata_file_path.to_str().unwrap())
        .status();

    match status {
        Ok(s) => {
            if s.success() {
                println!("'agy' command executed successfully.");
            } else {
                eprintln!("'agy' command failed with exit code: {:?}", s.code());
            }
        }
        Err(e) => {
            eprintln!("Failed to execute 'agy': {}", e);
            eprintln!("(This is expected if 'agy' is not in your PATH. You may need to run it manually.)");
        }
    }

    Ok(())
}
