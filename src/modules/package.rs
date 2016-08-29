use std::process::Command;
use std::io;
use distrib;
use modules::test;
use std::str;

#[derive(Debug)]
pub enum PackageError {
    Io(io::Error),
    ParseBool(str::ParseBoolError)
}

impl From<io::Error> for PackageError {
    fn from(err: io::Error) -> PackageError {
        PackageError::Io(err)
    }
}

impl From<str::ParseBoolError> for PackageError {
    fn from(err: str::ParseBoolError) -> PackageError {
        PackageError::ParseBool(err)
    }
}

pub fn check_deb(package: &str, version: Option<&str>, installed: bool, result: &mut test::TestResult) -> Result<(), io::Error> {
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
        result.success += 1;
        result.summary.push(test::UnitResult::from(success))
    }
    else {
        let error = test::UnitError {
            test: test_name.to_string(),
            expected: expected_string,
            actual: format!("package found: {}", found),
            message: "Package test fail".to_string(),
        };
        result.error += 1;
        result.summary.push(test::UnitResult::from(error))
    }
    Ok(())
}

pub fn check_rpm(package: &str, version: Option<&str>, installed: bool, result: &mut test::TestResult) -> Result<(), io::Error> {
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
        result.success += 1;
        result.summary.push(test::UnitResult::from(success))
    }
    else {
        let error = test::UnitError {
            test: test_name.to_string(),
            expected: expected_string,
            actual: format!("package found: {}", found),
            message: "Package test fail".to_string(),
        };
        result.error += 1;
        result.summary.push(test::UnitResult::from(error))
    }
    Ok(())
}

fn package_manager_not_found (package: &str, version: Option<&str>, installed: bool, result: &mut test::TestResult) {
    let test_name = "package";
    let expected_string = if version.is_some() {
        format!("name: {}, version: {}, installed: {}", package, version.unwrap(), installed)
    }
    else {
        format!("name: {}, installed: {}", package, installed)
    };
    let error = test::UnitError {
        test: test_name.to_string(),
        expected: expected_string,
        actual: "package manager not found".to_string(),
        message: "lothaire failed to determine your package manager".to_string(),
    };
    result.error += 1;
    result.summary.push(test::UnitResult::from(error));
}

pub fn check(package: &str, installed: &str, version: Option<&str>) -> Result<test::TestResult, PackageError> {
    let installed_bool: bool = try!(installed.parse());
    let mut result = test::TestResult {
        success: 0,
        error: 0,
        summary: Vec::new()
    };
    match distrib::get_package_manager().as_ref() {
        "rpm" => try!(check_rpm(package, version, installed_bool, &mut result)),
        "deb" => try!(check_deb(package, version, installed_bool, &mut result)),
        _ => package_manager_not_found(package, version, installed_bool, &mut result)
    };
    Ok(result)
}

// TESTS


#[test]
fn check_rpm_test_success() {
    if distrib::get_package_manager() == "rpm" {
        let mut result = test::TestResult {
            success: 0,
            error: 0,
            summary: Vec::new()
        };
        let openssl_version = "1.0.1e";
        let mut package_name = "openssl";
        // package exists and version ok
        let _ = check_rpm(package_name, Some(openssl_version), true, &mut result).unwrap();
        match result.summary[0] {
            test::UnitResult::Success(ref s) => assert_eq!(s.expected, format!("name: {}, version: {}, installed: {}", package_name, openssl_version, true)),
            test::UnitResult::Error(_) => panic!("error in test")
        }
        // package exists and no version
        let _ = check_rpm(package_name, None, true, &mut result).unwrap();
        match result.summary[1] {
            test::UnitResult::Success(ref s) => assert_eq!(s.expected, format!("name: {}, installed: {}", package_name, true)),
            test::UnitResult::Error(_) => panic!("error in test")
        }
        // package exists but incorrect version
        let _ = check_rpm(package_name, Some("1.0.2"), true, &mut result).unwrap();
        match result.summary[2] {
            test::UnitResult::Error(ref s) => {
                assert_eq!(s.expected, format!("name: {}, version: {}, installed: {}", package_name, "1.0.2", true));
                assert_eq!(s.actual, "package found: false");
            },
            test::UnitResult::Success(_) => panic!("error in test")
        }
        // package dont exists and no version and test not installed
        package_name = "notexists";
        let _ = check_rpm(package_name, None, false, &mut result).unwrap();
        match result.summary[3] {
            test::UnitResult::Success(ref s) => assert_eq!(s.expected, format!("name: {}, installed: {}", package_name, false)),
            test::UnitResult::Error(_) => panic!("error in test")
        }
        // package dont exists and no version and test installed
        let _ = check_rpm(package_name, None, true, &mut result).unwrap();
        match result.summary[4] {
            test::UnitResult::Error(ref e) => {
                assert_eq!(e.expected, format!("name: {}, installed: {}", package_name, true));
                assert_eq!(e.actual, format!("package found: {}", false));
            }
            test::UnitResult::Success(_) => panic!("error in test")
        }
        // package not exists and incorrect version
        let _ = check_rpm(package_name, Some("1"), true, &mut result).unwrap();
        match result.summary[5] {
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
        let mut result = test::TestResult {
            success: 0,
            error: 0,
            summary: Vec::new()
        };
        let openssl_version = "1.0.1t-1+deb8u2";
        let mut package_name = "openssl";
        // package exists and version ok
        let _ = check_deb(package_name, Some(openssl_version), true, &mut result);
        match result.summary[0] {
            test::UnitResult::Success(ref s) => assert_eq!(s.expected, format!("name: {}, version: {}, installed: {}", package_name, openssl_version, true)),
            test::UnitResult::Error(_) => panic!("error in test")
        }
        // package exists and no version
        let _ = check_deb(package_name, None, true, &mut result);
        match result.summary[1] {
            test::UnitResult::Success(ref s) => assert_eq!(s.expected, format!("name: {}, installed: {}", package_name, true)),
            test::UnitResult::Error(_) => panic!("error in test")
        }
        // package exists but incorrect version
        let _ = check_deb(package_name, Some("1.0.2"), true, &mut result);
        match result.summary[2] {
            test::UnitResult::Error(ref s) => {
                assert_eq!(s.expected, format!("name: {}, version: {}, installed: {}", package_name, "1.0.2", true));
                assert_eq!(s.actual, "package found: false");
            },
            test::UnitResult::Success(_) => panic!("error in test")
        }
        // package dont exists and no version and test not installed
        package_name = "notexists";
        let _ = check_deb(package_name, None, false, &mut result);
        match result.summary[3] {
            test::UnitResult::Success(ref s) => assert_eq!(s.expected, format!("name: {}, installed: {}", package_name, false)),
            test::UnitResult::Error(_) => panic!("error in test")
        }
        // package dont exists and no version and test installed
        let _ = check_deb(package_name, None, true, &mut result);
        match result.summary[4] {
            test::UnitResult::Error(ref e) => {
                assert_eq!(e.expected, format!("name: {}, installed: {}", package_name, true));
                assert_eq!(e.actual, format!("package found: {}", false));
            }
            test::UnitResult::Success(_) => panic!("error in test")
        }
        // package not exists and incorrect version
        let _ = check_deb(package_name, Some("1"), true, &mut result);
        match result.summary[5] {
            test::UnitResult::Error(ref e) => {
                assert_eq!(e.expected, format!("name: {}, version: {}, installed: {}", package_name, "1", true));
                assert_eq!(e.actual, format!("package found: {}", false));
            }
            test::UnitResult::Success(_) => panic!("error in test")
        }
    }
}

