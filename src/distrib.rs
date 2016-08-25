use std::fs;

fn is_rhel() -> bool {
    let file = fs::File::open("/etc/redhat_release");
    match file {
        Ok(_) => true,
        Err(_) => false
    }
}

fn is_debian() -> bool {
    let file = fs::File::open("/etc/debian_version");
    match file {
        Ok(_) => true,
        Err(_) => false
    }
}

pub fn get_distrib() -> String {
    if is_rhel() {
        "rhel".to_string()
    }
    else if is_debian() {
        "debian".to_string()
    }
    else {
        "unknow".to_string()
    }
}

pub fn get_package_manager() -> String {
    match get_distrib().as_ref() {
        "rhel" => "rpm".to_string(),
        "debian" => "deb".to_string(),
        _ => "unknow".to_string()
    }
}
