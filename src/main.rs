use std::env;
use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::fs;
use std::path::PathBuf;

const VERSION_PREFIX : &str = "version: ";






fn main() -> io::Result<()> {

    
    if fs::exists("Chart.yaml")? {
        let file_path = find_full_file_path()?;

        let new_version = "version: 1.2.3"; // Change this to your desired version
    
        // Read the file content
        let mut file = OpenOptions::new().read(true).open(file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
    
        println!("{:?}", content);
    
        for line in content.lines() {
            if line.starts_with(VERSION_PREFIX) {
                println!("Current line: {}", line);
                if let Some(version_str) = line.strip_prefix(VERSION_PREFIX) {
                    println!("XXXXX{:?}", version_str.to_string());
                    let version_int = convert_to_int(version_str);
            
                    println!("Extracted version as integer: {}", version_int);
                } else {
                    println!("No version found");
                }
            }
        }
    
    }
    else {
        println!("Chart.yaml not found!");
        std::process::exit(1);
    }
    

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} in a directory containing a Chart.yaml file for a helm chart", args[0]);
        std::process::exit(1);
    }

    // Create a regex to find the version string
    // let re = Regex::new(r"version: \d+\.\d+\.\d+").unwrap();
    // let updated_content = re.replace(&content, new_version);

    // Write the updated content back to the file
    // let mut file = OpenOptions::new().write(true).truncate(true).open(file_path)?;
    // file.write_all(updated_content.as_bytes())?;

    Ok(())
}

fn convert_to_int(version_str: &str) -> i32 {
    let version_int: i32 = version_str
        .split('.')
        .collect::<String>()
        .parse()
        .expect("Failed to parse version number");
    version_int
}

fn find_full_file_path() -> Result<PathBuf, io::Error> {
    println!("Found chart!");
    let current_path = env::current_dir()?;
    println!("Current dir is: {}", current_path.display());
    let file_path = PathBuf::new().join(current_path).join("Chart.yaml");
    println!("Opening file at: {:?}", file_path.display());
    Ok(file_path)
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_valid_input_convert_to_int(){

        let mut map = HashMap::new();
        map.insert("0.0.1", 1);
        map.insert("0.0.2", 2);
        map.insert("0.2.0", 20);
        map.insert("10.0.0", 1000);

        for (str_value, expected_number) in map {
            let number = convert_to_int(str_value);

            assert_eq!(expected_number, number);
        }
    }

}