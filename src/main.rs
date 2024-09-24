use std::io::{self};

use helm_chart_version_bumper::{
    find_valid_yaml_files, handle_updated_of_helm_chart_version, is_argo_appcation, is_helm_chart,
};

fn main() -> io::Result<()> {
    let argo_path_bufs = find_valid_yaml_files();
    for path_buf in argo_path_bufs {
        if is_helm_chart(&path_buf) || is_argo_appcation(&path_buf) {
            println!("Version bump");
            handle_updated_of_helm_chart_version(&path_buf)?;
            println!("Remember to run git diff to check your updated files")
        } else {
            println!("Cannot bump file: {}, not recognized", &path_buf.display())
        }
    }

    // else {
    //     let args: Vec<String> = env::args().collect();
    //     eprintln!("Usage: {} in a directory containing a Chart.yaml file for a helm chart", args[0]);
    //     std::process::exit(1);
    // }

    Ok(())
}
