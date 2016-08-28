use std::process::Command;
use std::io;
use distrib;
use modules::test;

pub fn check_deb(package: &str, version: Option<&str>, installed: bool) -> Result<test::UnitResult, io::Error> {
    let test_name = "package";
    let command_result = try!(Command::new("dpkg-query")
        .arg("-f")
        .arg("${status}---${version}\n")
        .arg("-W")
        .arg(package)
        .output());
    let out = String::from_utf8_lossy(&command_result.stdout);
    let mut found = false;
    let out_lines: Vec<&str> = out.split("\n").collect();
    for line in out_lines {
        let line_array: Vec<&str> = line.split("---").collect();
        if line_array[0] == "install ok installed" { // package installed
            if version.is_some() { // version needed
                if line_array.len() == 2 {
                    let version = version.unwrap();
                    if version == line_array[1] { // version ok
                        found = true;
                        break;
                    }
                }
            } // version not ok
            else {
                found = true;
                break;
            }
        }
    }
    let expected_string = if version.is_some() {
        format!("name: {}, version: {}, installed: {}", package, version.unwrap(), installed)
    }
    else {
        format!("name: {}, installed: {}", package, installed)
    };
    if found == installed {
        let success = test::UnitSuccess {
            test: test_name.to_string(),
            expected: expected_string
        };
        Ok(test::UnitResult::from(success))
    }
    else {
        let error = test::UnitError {
            test: test_name.to_string(),
            expected: expected_string,
            actual: format!("package found: {}", found),
            message: "Package test fail".to_string(),
        };
        Ok(test::UnitResult::from(error))
    }
}

pub fn check_rpm(package: &str, version: Option<&str>, installed: bool) -> Result<test::UnitResult, io::Error> {
    let test_name = "package";
    let command_result = try!(Command::new("rpm")
        .arg("-q")
        .arg("--queryformat")
        .arg("%{name}---%{version}")
        .arg(package)
        .output());
    let out = String::from_utf8_lossy(&command_result.stdout);
    let out_array: Vec<&str> = out.split("---").collect();
    let mut found = false;
    if version.is_some() {
        let version = version.unwrap();
        if out == format!("{}---{}", package, version) {
            found = true;
        }
    }
    else {
        if out_array[0] == package {
            found = true;
        }
    }
    let expected_string = if version.is_some() {
        format!("name: {}, version: {}, installed: {}", package, version.unwrap(), installed)
    }
    else {
        format!("name: {}, installed: {}", package, installed)
    };
    if found == installed {
        let success = test::UnitSuccess {
            test: test_name.to_string(),
            expected: expected_string
        };
        Ok(test::UnitResult::from(success))
    }
    else {
        let error = test::UnitError {
            test: test_name.to_string(),
            expected: expected_string,
            actual: format!("package found: {}", found),
            message: "Package test fail".to_string(),
        };
        Ok(test::UnitResult::from(error))
    }
}


// TESTS


#[test]
fn check_rpm_test_success() {
    if distrib::get_package_manager() == "rpm" {
        let openssl_version = "1.0.1e";
        let mut package_name = "openssl";
        // package exists and version ok
        let  mut result = check_rpm(package_name, Some(openssl_version), true).unwrap();
        match result {
            test::UnitResult::Success(ref s) => assert_eq!(s.expected, format!("name: {}, version: {}, installed: {}", package_name, openssl_version, true)),
            test::UnitResult::Error(_) => panic!("error in test")
        }
        // package exists and no version
        result = check_rpm(package_name, None, true).unwrap();
        match result {
            test::UnitResult::Success(ref s) => assert_eq!(s.expected, format!("name: {}, installed: {}", package_name, true)),
            test::UnitResult::Error(_) => panic!("error in test")
        }
        // package exists but incorrect version
        result = check_rpm(package_name, Some("1.0.2"), true).unwrap();
        match result {
            test::UnitResult::Error(ref s) => {
                assert_eq!(s.expected, format!("name: {}, version: {}, installed: {}", package_name, "1.0.2", true));
                assert_eq!(s.actual, "package found: false");
            },
            test::UnitResult::Success(_) => panic!("error in test")
        }
        // package dont exists and no version and test not installed
        package_name = "notexists";
        result = check_rpm(package_name, None, false).unwrap();
        match result {
            test::UnitResult::Success(ref s) => assert_eq!(s.expected, format!("name: {}, installed: {}", package_name, false)),
            test::UnitResult::Error(_) => panic!("error in test")
        }
        // package dont exists and no version and test installed
        result = check_rpm(package_name, None, true).unwrap();
        match result {
            test::UnitResult::Error(ref e) => {
                assert_eq!(e.expected, format!("name: {}, installed: {}", package_name, true));
                assert_eq!(e.actual, format!("package found: {}", false));
            }
            test::UnitResult::Success(_) => panic!("error in test")
        }
        // package not exists and incorrect version
        result = check_rpm(package_name, Some("1"), true).unwrap();
        match result {
            test::UnitResult::Error(ref e) => {
                assert_eq!(e.expected, format!("name: {}, version: {}, installed: {}", package_name, "1", true));
                assert_eq!(e.actual, format!("package found: {}", false));
            }
            test::UnitResult::Success(_) => panic!("error in test")
        }
    }
}

#[test]
fn check_deb_test_success() {
    if distrib::get_package_manager() == "deb" {
        let openssl_version = "1.0.1t-1+deb8u2";
        let mut package_name = "openssl";
        // package exists and version ok
        let mut result = check_deb(package_name, Some(openssl_version), true).unwrap();
        match result {
            test::UnitResult::Success(ref s) => assert_eq!(s.expected, format!("name: {}, version: {}, installed: {}", package_name, openssl_version, true)),
            test::UnitResult::Error(_) => panic!("error in test")
        }
        // package exists and no version
        result = check_deb(package_name, None, true).unwrap();
        match result {
            test::UnitResult::Success(ref s) => assert_eq!(s.expected, format!("name: {}, installed: {}", package_name, true)),
            test::UnitResult::Error(_) => panic!("error in test")
        }
        // package exists but incorrect version
        result = check_deb(package_name, Some("1.0.2"), true).unwrap();
        match result {
            test::UnitResult::Error(ref s) => {
                assert_eq!(s.expected, format!("name: {}, version: {}, installed: {}", package_name, "1.0.2", true));
                assert_eq!(s.actual, "package found: false");
            },
            test::UnitResult::Success(_) => panic!("error in test")
        }
        // package dont exists and no version and test not installed
        package_name = "notexists";
        result = check_deb(package_name, None, false).unwrap();
        match result {
            test::UnitResult::Success(ref s) => assert_eq!(s.expected, format!("name: {}, installed: {}", package_name, false)),
            test::UnitResult::Error(_) => panic!("error in test")
        }
        // package dont exists and no version and test installed
        result = check_deb(package_name, None, true).unwrap();
        match result {
            test::UnitResult::Error(ref e) => {
                assert_eq!(e.expected, format!("name: {}, installed: {}", package_name, true));
                assert_eq!(e.actual, format!("package found: {}", false));
            }
            test::UnitResult::Success(_) => panic!("error in test")
        }
        // package not exists and incorrect version
        result = check_deb(package_name, Some("1"), true).unwrap();
        match result {
            test::UnitResult::Error(ref e) => {
                assert_eq!(e.expected, format!("name: {}, version: {}, installed: {}", package_name, "1", true));
                assert_eq!(e.actual, format!("package found: {}", false));
            }
            test::UnitResult::Success(_) => panic!("error in test")
        }
    }
}

