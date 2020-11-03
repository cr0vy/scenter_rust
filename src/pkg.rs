use std::process::Command;
use std::str;

pub fn get_installed_list() -> Vec<String> {
    let output = Command::new("apt")
        .arg("list")
        .arg("--installed")
        .output()
        .expect("Failed to execute apt process");

    parse_output(output)
}

pub fn parse_output(output: std::process::Output) -> Vec<String> {
    let parsed = str::from_utf8(&output.stdout).expect("Error");
    let mut arr: Vec<String> = vec![];

    for line in parsed.split("\n") {
        if line != "" {
            let name_list: Vec<&str> = line.split("/").collect();
            let name = name_list.get(0).unwrap().to_string();
            arr.push(name);
        }
    }

    arr
}
