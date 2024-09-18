use std::env;
use std::fs::OpenOptions;
use std::io::{self, Read, Write};

const VERSION_PREFIX : &str = "version: ";

fn main() -> io::Result<()> {
    // Get the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }
    let file_path = &args[1];

    let new_version = "version: 1.2.3"; // Change this to your desired version

    // Read the file content
    let mut file = OpenOptions::new().read(true).open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    println!("{:?}", content);

    if let Some(version_str) = content.strip_prefix(VERSION_PREFIX) {
        let version_int: i32 = version_str
            .split('.')
            .collect::<String>()
            .parse()
            .expect("Failed to parse version number");

        println!("Extracted version as integer: {}", version_int);
    } else {
        println!("No version found");
    }

    // Create a regex to find the version string
    // let re = Regex::new(r"version: \d+\.\d+\.\d+").unwrap();
    // let updated_content = re.replace(&content, new_version);

    // Write the updated content back to the file
    let mut file = OpenOptions::new().write(true).truncate(true).open(file_path)?;
    // file.write_all(updated_content.as_bytes())?;

    Ok(())
}
