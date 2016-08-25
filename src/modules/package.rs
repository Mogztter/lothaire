use std::process::Command;
use std::io;
use distrib;

pub fn check_debian(package: &str, version: &str, installed: bool) -> Result<String, io::Error>{
    let command_result = try!(Command::new("dpkg-query")
        .arg("-f")
        .arg("'${status} ${version}\n'")
        .arg("-W")
        .arg(package)
        .output());
    let out = String::from_utf8_lossy(&command_result.stdout);
    Ok("compile".to_string())
}
