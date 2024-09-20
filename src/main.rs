use std::borrow::Borrow;
use std::env;
use std::fs::{DirEntry, OpenOptions};
use std::io::{self, Read, Write, stdin};
use std::fs;
use std::path::PathBuf;

use log::{debug, error, log_enabled, info, Level};



const VERSION_PREFIX : &str = "version: ";


enum Type {
    HelmChartYamlFile,
    ArgoAppYamlFile
}



fn main() -> io::Result<()> {
    env_logger::init();

    
    if fs::exists("Chart.yaml")? {
        info!("Handling Chart.yaml");
        find_argo_yaml();
        // handle_helm_chart_yaml()?;    
    }

 
    else {
        let args: Vec<String> = env::args().collect();
        eprintln!("Usage: {} in a directory containing a Chart.yaml file for a helm chart", args[0]);
        std::process::exit(1);
    }
    

    Ok(())
}


fn find_argo_yaml() -> Vec<DirEntry> {
    let mut files: Vec<DirEntry> = Vec::new();

    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        debug!("Found files");
        let file_name = &path.unwrap().path();
        info!("Name: {}", file_name.display());

        if file_name.clone().to_string_lossy().ends_with("yaml") && !file_name.clone().to_string_lossy().starts_with("Chart."){
            debug!("Found yaml file that is not a Helm Chart file!");
            debug!("Found yaml file {}", file_name.display());
            let mut content = String::new();
            let mut file = OpenOptions::new().read(true).open(file_name).unwrap();
            file.read_to_string(&mut content).unwrap();

            if is_argo_appcation(&content){
                println!("Is argo application!");
            }
            
        }
    }
  
    return files;

}


fn is_argo_appcation(content: &String) -> bool {
    return content.contains("apiVersion: argoproj.io") && content.contains("kind: Application")   
}

fn handle_helm_chart_yaml() -> Result<(), io::Error> {
    let file_path = find_full_file_path()?;
    let mut file = OpenOptions::new().read(true).open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    println!("===================================================================");
    println!("Old file looked like this");
    println!("{}",content.clone());
    println!("===================================================================");
    Ok(if let Some(new_content) = update_version(content){
    
        println!("New file will look like this");
        println!("{}", new_content);
        println!("===================================================================");

        print!("Do you want to apply this [y/n]?");
        io::stdout().flush()?;
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Error reading input");
    
        if input.contains("y") {
            println!("Overwriting file!");
            // Write the updated content back to the file
            let mut file = OpenOptions::new().write(true).truncate(true).open(find_full_file_path()?)?;
            file.write_all(new_content.as_bytes())?;
        }
        else {
            println!("Skipping");
        }
    }
    else {
        eprint!("Failed to update anything!");
    })
}

fn update_version(content: String) -> Option<String> {
    for line in content.lines() {
        if let Some(version_str) = get_version_string(line) {
            info!("Version found: {:?}", version_str.to_string());
            let version_str = version_str.trim();

            let new_version = increment_version(version_str);
            info!("New version string: {:?}", new_version.unwrap());

            if let Some(new_version_str) = increment_version(version_str){
                let mut new_version_full_str  = String::from(VERSION_PREFIX);
                new_version_full_str.push_str(new_version_str.as_str());

                debug!("Replacing {:?} with {:?}", line, new_version_full_str);
                let new_content = content.replace(line, new_version_full_str.as_str());
                return Some(new_content);
            }
        }
    }
    return None;

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
    use std::{borrow::Borrow, collections::HashMap};

    use super::*;

    #[test]
    fn test_increment_version_number_should_return_expected_number(){
        let input = "0.2.0";
        let expected = "0.2.1";
        let result = increment_version(input);

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn test_increment_version_number_should_return_expected_number_map(){
        let mut map = HashMap::new();
        map.insert("0.0.1", "0.0.2");
        map.insert("0.0.9", "0.1.0");
        map.insert("8.0.0", "8.0.1");
        map.insert("8.9.1", "8.9.2");
        map.insert("0.0.9", "0.1.0");
        map.insert("9.9.9", "1.0.0.0");

        for (str_value, expected_number) in map {
            let number = increment_version(str_value).unwrap();

            assert_eq!(expected_number, number);
        }
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


    #[test]
    fn test_update_version_has_correct_output_based_on_file_input_as_string(){
        let input = "apiVersion: v2\nname: some-deploy-rules\ndescription: A Stoopid Helm chart for Kubernetes Something using images\nicon: https://www.dictionary.com/e/wp-content/uploads/2018/03/thisisfine-1.jpg\nkeywords:\n- thisisfine\n- development\n- \n# A chart can be either an 'application' or a 'library' chart.\n#\n# Application charts are a collection of templates that can be packaged into versioned archives\n# to be deployed.\n#\n# Library charts provide useful utilities or functions for the chart developer. They're included as\n# a dependency of application charts to inject those utilities and functions into the rendering\n# pipeline. Library charts do not define any templates and therefore cannot be deployed.\ntype: application\n\n# This is the chart version. This version number should be incremented each time you make changes\n# to the chart and its templates, including the app version.\n# Versions are expected to follow Semantic Versioning (https://semver.org/)\nversion: 0.2.0\n\n# This is the version number of the application being deployed. This version number should be\n# incremented each time you make changes to the application. Versions are not expected to\n# follow Semantic Versioning. They should reflect the version the application is using.\n# It is recommended to use it with quotes.\nappVersion: \"1.16.0\" \ndependencies:\n- name: common\n  repository: oci://registry-1.docker.io/bitnamicharts\n  tags:\n  - bitnami-common\n  version: 2.x.x".to_string();

        let expected_output = "apiVersion: v2\nname: some-deploy-rules\ndescription: A Stoopid Helm chart for Kubernetes Something using images\nicon: https://www.dictionary.com/e/wp-content/uploads/2018/03/thisisfine-1.jpg\nkeywords:\n- thisisfine\n- development\n- \n# A chart can be either an 'application' or a 'library' chart.\n#\n# Application charts are a collection of templates that can be packaged into versioned archives\n# to be deployed.\n#\n# Library charts provide useful utilities or functions for the chart developer. They're included as\n# a dependency of application charts to inject those utilities and functions into the rendering\n# pipeline. Library charts do not define any templates and therefore cannot be deployed.\ntype: application\n\n# This is the chart version. This version number should be incremented each time you make changes\n# to the chart and its templates, including the app version.\n# Versions are expected to follow Semantic Versioning (https://semver.org/)\nversion: 0.2.1\n\n# This is the version number of the application being deployed. This version number should be\n# incremented each time you make changes to the application. Versions are not expected to\n# follow Semantic Versioning. They should reflect the version the application is using.\n# It is recommended to use it with quotes.\nappVersion: \"1.16.0\" \ndependencies:\n- name: common\n  repository: oci://registry-1.docker.io/bitnamicharts\n  tags:\n  - bitnami-common\n  version: 2.x.x".to_string();

        let result = update_version(input).unwrap();

        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_is_argo_app_file_is_argo_app_file_returns_true(){
        let input : String = "apiVersion: argoproj.io/v1alpha1
                            kind: Application
                            metadata:
                            name: demo-app
                            namespace: some-namespace
                            finalizers:
                                - resources-finalizer.argocd.argoproj.io
                            labels:
                                odm.hmm.com/instance: demo-app
                            annotations:
                                gitops-trace.hmm.com/build-reason: IndividualCI
                            spec:
                            destination:
                                namespace: some-namespace
                                server: https://kubernetes.default.svc
                            project: some-project
                            source:
                                chart: here
                                helm:
                                valuesObject: 
                                    container:
                                    image:
                                        repository: ubuntu
                                        version: 24.04
                                targetRevision: 0.3.3
                            syncPolicy:
                                automated:
                                prune: true
                            ".into();
        let result = is_argo_appcation(&input);
        assert!(result);        

    }

    #[test]
    fn is_argo_appcation_has_empty_string_returns_false(){
        let result = is_argo_appcation("".to_string().borrow());
        assert!(result == false);
    }
}