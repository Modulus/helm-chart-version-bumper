use std::{fs, path::PathBuf};

use std::{
    fs::OpenOptions,
    io::{self, stdin, Read, Write},
};


const VERSION_PREFIX_HELM_CHART: &str = "version: ";
const VERSION_PREFIX_ARGO: &str = "    targetRevision: ";

pub fn is_helm_chart(file_name: &PathBuf) -> bool {
    file_name.ends_with("Chart.yaml")
}

pub fn is_argo_appcation(file_path: &PathBuf) -> bool {
    match OpenOptions::new().read(true).open(file_path) {
        Ok(mut file) => {
            let mut content = String::new();
            match file.read_to_string(&mut content) {
                Ok(_) => {
                    return file_contains_argo_app_fields(&content);
                }
                Err(_) => {
                    eprintln!("Failed to read file: {}", &file_path.display());
                    return false;
                }
            };
        }
        Err(_) => {
            eprint!("Failed to open file!");
            return false;
        }
    }
}

pub fn file_contains_argo_app_fields(content: &String) -> bool {
    if content.contains("apiVersion: argoproj.io") && content.contains("kind: Application") {
        return true;
    }
    false
}

pub fn handle_updated_of_helm_chart_version(file_path: &PathBuf) -> Result<(), io::Error> {
    let content = read_file(file_path)?;
    // println!("===================================================================");
    // println!("Old file looked like this");
    // println!("{}", content.clone());
    // println!("===================================================================");
    Ok(if let Some(new_content) = update_version(content) {
        // println!("New file will look like this");
        // println!("{}", new_content);
        // println!("===================================================================");
        println!("Bump version of helm chart in file: {}", &file_path.display());
        print!("Do you want to apply this [y/n]?");
        io::stdout().flush()?;
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Error reading input");

        if input.contains("y") || input.contains("Y") {
            println!("Overwriting file!");
            // Write the updated content back to the file
            let mut file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(file_path)?;
            file.write_all(new_content.as_bytes())?;
        } else {
            println!("Skipping");
        }
    } else {
        eprint!("Failed to update anything!");
    })
}

pub fn read_file(file_path: &PathBuf) -> Result<String, io::Error> {
    let mut file = OpenOptions::new().read(true).open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn update_version(content: String) -> Option<String> {
    for line in content.lines() {
        if let Some(version_str) = get_version_string(line) {
            let version_str = version_str.trim();


            if let Some(new_version_str) = increment_version(version_str) {
                let mut new_version_full_str = String::from(get_version_prefix(line)?);
                new_version_full_str.push_str(new_version_str.as_str());

                println!("Will be replacing {:?} with {:?}", line, new_version_full_str);
                let new_content = content.replace(line, new_version_full_str.as_str());
                return Some(new_content);
            }
        }
    }
    return None;
}

pub fn increment_version<'a>(version_str: &'a str) -> Option<String> {
    let new_version = convert_to_int(version_str) + 1;
    let bumped_raw = format!("{:0>3}", new_version);
    let mut new_version = "".to_string();
    for (i, c) in bumped_raw.chars().enumerate() {
        new_version.push(c);
        if i < bumped_raw.len() - 1 {
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
    if line.starts_with(VERSION_PREFIX_HELM_CHART) {
        return line.strip_prefix(VERSION_PREFIX_HELM_CHART);
    } else if line.contains(VERSION_PREFIX_ARGO) {
        return line.strip_prefix(VERSION_PREFIX_ARGO);
    }
    return None;
}

fn get_version_prefix<'a>(line: &str) -> Option<&'a str> {
    if line.starts_with(VERSION_PREFIX_HELM_CHART) {
        return Some(VERSION_PREFIX_HELM_CHART);
    } else if line.contains(VERSION_PREFIX_ARGO) {
        return Some(&VERSION_PREFIX_ARGO);
    }
    return None;
}


pub fn find_valid_yaml_files() -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();

    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        let path_buf = &path.unwrap().path();

        if path_buf.clone().to_string_lossy().ends_with("yaml")
            || path_buf.clone().to_string_lossy().ends_with("yml")
        {
            if is_argo_appcation(&path_buf) || is_helm_chart(path_buf) {
                files.push(path_buf.clone());
            }
        }
    }

    return files;
}

#[cfg(test)]
mod tests {
    use std::{borrow::Borrow, collections::HashMap};

    use super::*;

    #[test]
    fn test_increment_version_number_should_return_expected_number() {
        let input = "0.2.0";
        let expected = "0.2.1";
        let result = increment_version(input);

        assert_eq!(expected, result.unwrap());
    }

    #[test]
    fn test_increment_version_number_should_return_expected_number_map() {
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
    fn test_get_version_string_has_valid_input_returns_correct_string() {
        let line = "version: 2.2.2";

        let result = get_version_string(line);
        assert_eq!("2.2.2", result.unwrap());
    }

    #[test]
    fn test_get_version_invalid_input_returns_none() {
        let line = "VERSION: 22.2.2.2";

        let result = get_version_string(line);
        assert!(result.is_none());
    }

    #[test]
    fn test_valid_input_convert_to_int_should_result_in_correct_number() {
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
    fn test_invalid_input_empty_string_convert_to_int_shoud_fail() {
        convert_to_int("");
    }

    #[test]
    #[should_panic]
    fn test_invalid_input_spaces_convert_to_int_should_fail() {
        convert_to_int("   ");
    }

    #[test]
    #[should_panic]
    fn test_invalid_input_letters_convert_to_int_should_fail() {
        convert_to_int("JKLASJDKLASJDL");
    }

    #[test]
    fn test_update_version_has_correct_output_based_on_file_input_as_string() {
        let input = "apiVersion: v2\nname: some-deploy-rules\ndescription: A Stoopid Helm chart for Kubernetes Something using images\nicon: https://www.dictionary.com/e/wp-content/uploads/2018/03/thisisfine-1.jpg\nkeywords:\n- thisisfine\n- development\n- \n# A chart can be either an 'application' or a 'library' chart.\n#\n# Application charts are a collection of templates that can be packaged into versioned archives\n# to be deployed.\n#\n# Library charts provide useful utilities or functions for the chart developer. They're included as\n# a dependency of application charts to inject those utilities and functions into the rendering\n# pipeline. Library charts do not define any templates and therefore cannot be deployed.\ntype: application\n\n# This is the chart version. This version number should be incremented each time you make changes\n# to the chart and its templates, including the app version.\n# Versions are expected to follow Semantic Versioning (https://semver.org/)\nversion: 0.2.0\n\n# This is the version number of the application being deployed. This version number should be\n# incremented each time you make changes to the application. Versions are not expected to\n# follow Semantic Versioning. They should reflect the version the application is using.\n# It is recommended to use it with quotes.\nappVersion: \"1.16.0\" \ndependencies:\n- name: common\n  repository: oci://registry-1.docker.io/bitnamicharts\n  tags:\n  - bitnami-common\n  version: 2.x.x".to_string();

        let expected_output = "apiVersion: v2\nname: some-deploy-rules\ndescription: A Stoopid Helm chart for Kubernetes Something using images\nicon: https://www.dictionary.com/e/wp-content/uploads/2018/03/thisisfine-1.jpg\nkeywords:\n- thisisfine\n- development\n- \n# A chart can be either an 'application' or a 'library' chart.\n#\n# Application charts are a collection of templates that can be packaged into versioned archives\n# to be deployed.\n#\n# Library charts provide useful utilities or functions for the chart developer. They're included as\n# a dependency of application charts to inject those utilities and functions into the rendering\n# pipeline. Library charts do not define any templates and therefore cannot be deployed.\ntype: application\n\n# This is the chart version. This version number should be incremented each time you make changes\n# to the chart and its templates, including the app version.\n# Versions are expected to follow Semantic Versioning (https://semver.org/)\nversion: 0.2.1\n\n# This is the version number of the application being deployed. This version number should be\n# incremented each time you make changes to the application. Versions are not expected to\n# follow Semantic Versioning. They should reflect the version the application is using.\n# It is recommended to use it with quotes.\nappVersion: \"1.16.0\" \ndependencies:\n- name: common\n  repository: oci://registry-1.docker.io/bitnamicharts\n  tags:\n  - bitnami-common\n  version: 2.x.x".to_string();

        let result = update_version(input).unwrap();

        assert_eq!(result, expected_output);
    }

    #[test]
    fn test_file_contains_argo_app_fields_returns_true() {
        let input: String = "apiVersion: argoproj.io/v1alpha1
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
                            "
        .into();
        let result = file_contains_argo_app_fields(&input);
        assert!(result);
    }

    #[test]
    fn test_is_argo_appcation_has_empty_string_returns_false() {
        let result = is_argo_appcation(PathBuf::from("").borrow());
        assert!(result == false);
    }

    #[test]
    fn test_is_helm_chart_has_correct_file_name_returns_true() {
        let path_buf = PathBuf::from("Chart.yaml");
        assert!(is_helm_chart(&path_buf));
    }

    #[test]
    fn test_get_version_prefix_has_helm_chart_prefix_returns_correct_prefix() {
        let line = "version: 500";
        let prefix = get_version_prefix(line).unwrap();

        assert_eq!(VERSION_PREFIX_HELM_CHART, prefix);
    }

    #[test]
    fn test_get_version_prefix_has_aro_app_prefix_returns_correct_prefix() {
        let line = "    targetRevision: 100000";
        let prefix = get_version_prefix(line).unwrap();

        assert_eq!(VERSION_PREFIX_ARGO, prefix);
    }
}
