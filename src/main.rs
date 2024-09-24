use std::io::{self};

use helm_chart_version_bumper::{find_valid_yaml_files, handle_helm_chart_yaml, is_argo_appcation, is_helm_chart};

fn main() -> io::Result<()> {
    env_logger::init();

    
        let argo_path_bufs = find_valid_yaml_files();
        for path_buf in argo_path_bufs {
            if is_helm_chart(&path_buf){
                println!("Handle helm chart bump");
                handle_helm_chart_yaml(&path_buf)?;    
            }
            else if is_argo_appcation(&path_buf){
                println!("Handle argo Application file");
            }
        }

 
    // else {
    //     let args: Vec<String> = env::args().collect();
    //     eprintln!("Usage: {} in a directory containing a Chart.yaml file for a helm chart", args[0]);
    //     std::process::exit(1);
    // }
    

    Ok(())
}

