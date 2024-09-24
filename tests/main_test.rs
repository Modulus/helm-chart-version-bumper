
#[cfg(test)]
mod tests {
    use std::{env::temp_dir, fs::{self, OpenOptions}, io::{self, Write}, path::PathBuf};

    use helm_chart_version_bumper::{find_valid_yaml_files, read_file, update_version};

    #[test]
    fn test_find_valid_yaml_files_only_returns_valid_files(){
        let files = find_valid_yaml_files();
        assert!(files.len() >=2);
        assert!(files.iter().any(| pb : &PathBuf | pb.ends_with("Chart.yaml")));
        assert!(files.iter().any(| pb : &PathBuf | pb.ends_with("argo.yaml")));
    }

    #[test]
    fn test_update_version_in_memory_actualy_changes_version_for_helm_chart(){

        let origina_file_contens = read_file(&PathBuf::from("./Chart.yaml")).unwrap();
        assert!(origina_file_contens.contains("version: 0.2.0"));
        println!("{}", origina_file_contens);

        let new_contens = update_version(origina_file_contens).unwrap();

        assert!(new_contens.contains("version: 0.2.0") == false);
        assert!(new_contens.contains("version: 0.2.1"));
    }

    #[test]
    fn test_update_version_in_memory_actualy_changes_version_for_argo_app_yaml(){

        let origina_file_contens = read_file(&PathBuf::from("./argo.yaml")).unwrap();
        assert!(origina_file_contens.contains("    targetRevision: 0.3.3"));
        // println!("{}", origina_file_contens);

        let new_contens = update_version(origina_file_contens).unwrap();

        assert!(new_contens.contains("    targetRevision: 0.3.3") == false);
        assert!(new_contens.contains("    targetRevision: 0.3.4"));
    }

    // #[test]
    // fn test_replace_version_in_temp_file_actually_updates_helm_chart_file(){
    //     let temp_file = temp_dir().join("Chart.yaml");
    //     println!("TEMP_DIR: {}", &temp_file.display());

    //     let origina_file_contents = read_file(&PathBuf::from("./Chart.yaml")).unwrap();
    //     assert!(origina_file_contents.contains("version: 0.2.0"));
    //     println!("{}", origina_file_contents);

    //     let created = create_file(&origina_file_contents, temp_file.to_str().unwrap());
    //     assert!(created);

    //     update_version(origina_file_contents);

    //     assert!(fs::remove_file(temp_file.to_str().unwrap()).is_ok());

    //     // let temp_file = read_file(&temp_dir).unwrap();
    // }

    // fn create_file(content: &String, temp_file: &str)  -> bool {
    //     println!("Creating file: {}", temp_file);
    //     let mut file = OpenOptions::new().create_new(true).read(true).write(true).create(true).open(temp_file).unwrap();
    //     file.write(content.as_bytes()).unwrap();

    //     return true;
    // }

}