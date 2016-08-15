#[macro_use]
extern crate clap;
use clap::App;
pub mod modules;
pub mod util;
use modules::user;

fn main() {

    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(matches) = matches.subcommand_matches("user") {
        let username = matches.value_of("username").unwrap();
        let exists = matches.value_of("exists").unwrap();
        let uid = matches.value_of("uid");
        let gid = matches.value_of("gid");
        let group = matches.value_of("group");
        let groups = matches.value_of("groups");
        let test_result = user::check(username, exists, uid, gid, group, groups);
        match test_result {
            Ok(result) => {
                if result.error == 0 {
                    println!("All tests uccess : {:?}", result);
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

