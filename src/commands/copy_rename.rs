use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub fn run_copy_rename(input_file: String, ext_override: Option<String>) {
    let input_path = Path::new(&input_file);

    if !input_path.exists() {
        eprintln!("Input file does not exist: {}", input_path.display());
        return;
    }

    let extension = match &ext_override {
        Some(ext) => ext.clone(),
        None => match input_path.extension().and_then(|e| e.to_str()) {
            Some(e) => e.to_string(),
            None => {
                eprintln!("Could not determine extension.");
                return;
            }
        },
    };

    println!("Enter comma-separated output names (without extension):");
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let names: Vec<String> = input
        .trim()
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if names.is_empty() {
        eprintln!("No names entered.");
        return;
    }

    let mut will_overwrite = vec![];
    for new_base in &names {
        let new_filename = format!("{}.{}", new_base, extension);
        if Path::new(&new_filename).exists() {
            will_overwrite.push(new_filename);
        }
    }

    if !will_overwrite.is_empty() {
        println!("The following files already exist and will be overwritten:");
        for file in &will_overwrite {
            println!(" - {}", file);
        }
        print!("Do you want to proceed? (y/N): ");
        io::stdout().flush().unwrap();

        let mut confirm = String::new();
        io::stdin().read_line(&mut confirm).unwrap();
        let confirm = confirm.trim().to_lowercase();

        if confirm != "y" && confirm != "yes" {
            println!("Aborted.");
            return;
        }
    }

    for new_base in names {
        let new_filename = format!("{}.{}", new_base, extension);
        match fs::copy(&input_path, &new_filename) {
            Ok(_) => println!("Copied to {}", new_filename),
            Err(e) => eprintln!("Failed to copy to {}: {}", new_filename, e),
        }
    }
}
