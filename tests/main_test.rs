
#[cfg(test)]
mod tests {
    use std::{borrow::Borrow, collections::HashMap, path::PathBuf};

    use helm_chart_version_bumper::find_valid_yaml_files;

    use super::*;

    #[test]
    fn test_find_valid_yaml_files_only_returns_valid_files(){
        let files = find_valid_yaml_files();
        assert!(files.len() >=2);
        assert!(files.iter().any(| pb : &PathBuf | pb.ends_with("Chart.yaml")));
        assert!(files.iter().any(| pb : &PathBuf | pb.ends_with("argo.yaml")));
    }

}
