mod utils;
use utils::ls;

use std::env;

use crate::utils::{confirm, delete_object};


fn main() {
    let current_dir = env::current_dir().unwrap();
    let cwd_str = format!("{}{}", current_dir.to_str().unwrap(), "/");

    println!(
        "The current directory is: {}",
        current_dir.to_str().unwrap()
    );

    println!("Press q to quit");

    let path_result = ls(&current_dir).unwrap();

    for dir in path_result.1 {
        let printable_dir = dir.to_str().unwrap().replace(&cwd_str, "");
        println!("{}", printable_dir);
        if confirm("Remove directory?").unwrap() {
            let _ = delete_object(&dir, false);
        }
    }

    for file in path_result.0 {
        let printable_file = file.to_str().unwrap().replace(&cwd_str, "");
        println!("{}", printable_file);
        if confirm("Remove file?").unwrap() {
            let _ = delete_object(&file, true);
        }
    }

    println!("Directory cleaned");

}
