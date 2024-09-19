use std::env;
use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::fs;
use std::path::PathBuf;

const VERSION_PREFIX : &str = "version: ";






fn main() -> io::Result<()> {

    
    if fs::exists("Chart.yaml")? {
        let file_path = find_full_file_path()?;

        let mut file = OpenOptions::new().read(true).open(file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
    
        println!("{:?}", content);
    
        for line in content.lines() {
            if let Some(version_str) = get_version_string(line) {
                println!("Version found: {:?}", version_str.to_string());
                let version_str = version_str.trim();

                let new_version = increment_version(version_str);
                println!("New version string: {:?}", new_version.unwrap());

                let version_int = convert_to_int(version_str);
        
                println!("Extracted version as integer: {}", version_int);
            }
        }
    
    }
    else {
        let args: Vec<String> = env::args().collect();
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

fn increment_version<'a>(version_str: &'a str) -> Option<String> {//-> Option<&'a str> {
    let  new_version = convert_to_int(version_str)  + 1;
    let bumped_raw = format!("{:0>3}", new_version);
    let mut new_version = "".to_string();    
    for (i, c) in bumped_raw.chars().enumerate() {
        new_version.push(c);
        if i < bumped_raw.len() - 1{
            new_version.push_str(".");
        }
    }

    return Some(new_version);

}

fn convert_to_int(version_str: &str) -> i32 {
    let version_int: i32 = version_str
        .split('.')
        .collect::<String>()
        .parse()
        .expect("Failed to parse version number");
    version_int
}

fn get_version_string<'a>(line: &'a str) -> Option<&'a str> {
    if line.starts_with(VERSION_PREFIX) {
        return line.strip_prefix(VERSION_PREFIX);
    }

    return None

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
    fn test_increment_version_number_should_return_expected_number(){
        let input = "0.2.0";
        let expected = "0.2.1";
        let result = increment_version(input);

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn test_increment_version_number_should_return_expected_number_small(){
        let input = "0.0.5";
        let expected = "0.0.6";
        let result = increment_version(input);

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn test_get_version_string_has_valid_input_returns_correct_string(){
        let line = "version: 2.2.2";

        let result = get_version_string(line);
        assert_eq!("2.2.2", result.unwrap());
    }

    #[test]
    fn test_get_version_invalid_input_returns_none(){
        let line = "VERSION: 22.2.2.2";

        let result = get_version_string(line);
        assert!(result.is_none());
    }

    #[test]
    fn test_valid_input_convert_to_int_should_result_in_correct_number(){

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

    #[test]
    #[should_panic]
    fn test_invalid_input_empty_string_convert_to_int_shoud_fail(){
        convert_to_int("");
    }

    #[test]
    #[should_panic]
    fn test_invalid_input_spaces_convert_to_int_should_fail(){
        convert_to_int("   ");
    }

    #[test]
    #[should_panic]
    fn test_invalid_input_letters_convert_to_int_should_fail(){
        convert_to_int("JKLASJDKLASJDL");
    }


}