#[macro_use]
extern crate clap;
use clap::App;
pub mod modules;
pub mod util;
pub mod distrib;
use modules::user;
use modules::group;
use modules::package;

fn main() {

    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    // user subcommand
    if let Some(matches) = matches.subcommand_matches("user") {
        let username = matches.value_of("name").unwrap();
        let exists = matches.value_of("exists").unwrap();
        let uid = matches.value_of("uid");
        let gid = matches.value_of("gid");
        let group = matches.value_of("group");
        let groups = matches.value_of("groups");
        let test_result = user::check(username, exists, uid, gid, group, groups);
        match test_result {
            Ok(result) => {
                if result.error == 0 {
                    println!("All tests success : {:?}", result);
                    std::process::exit(0);
                }
                else {
                    println!("Error during tests : {:?}", result);
                    std::process::exit(1);
                }
            }
            Err(error) => {
                println!("System error : {:?}", error);
                std::process::exit(2);
            }
        }

    }

    // group subcommand
    if let Some(matches) = matches.subcommand_matches("group") {
        let name = matches.value_of("name").unwrap();
        let exists = matches.value_of("exists").unwrap();
        let gid = matches.value_of("gid");
        let test_result = group::check(name, exists, gid);
        match test_result {
            Ok(result) => {
                if result.error == 0 {
                    println!("All tests success : {:?}", result);
                    std::process::exit(0);
                }
                else {
                    println!("Error during tests : {:?}", result);
                    std::process::exit(1);
                }
            }
            Err(error) => {
                println!("System error : {:?}", error);
                std::process::exit(2);
            }
        }
    }

    // package subcommand
    if let Some(matches) = matches.subcommand_matches("package") {
        let name = matches.value_of("name").unwrap();
        let installed = matches.value_of("installed").unwrap();
        let version = matches.value_of("version");
        let test_result = package::check(name, installed, version);
        match test_result {
            Ok(result) => {
                if result.error == 0 {
                    println!("All tests success : {:?}", result);
                    std::process::exit(0);
                }
                else {
                    println!("Error during tests : {:?}", result);
                    std::process::exit(1);
                }
            }
            Err(error) => {
                println!("System error : {:?}", error);
                std::process::exit(2);
            }
        }
    }
}

