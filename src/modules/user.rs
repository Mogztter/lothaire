use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;
use std::num;
use std::str;
use util;
use modules::test;
use modules::group;

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub password: String,
    pub uid: i32,
    pub gid: i32,
    pub comment: String,
    pub home: String,
    pub init: String,
    pub group: String,
    pub groups: Vec<String>
}

#[derive(Debug)]
pub enum UserError {
    ParseInt(num::ParseIntError),
    Io(Error),
    ParseBool(str::ParseBoolError),
    Group(group::GroupError)
}

impl From<group::GroupError> for UserError {
    fn from(err: group::GroupError) -> UserError {
        UserError::Group(err)
    }
}

impl From<Error> for UserError {
    fn from(err: Error) -> UserError {
        UserError::Io(err)
    }
}

impl From<str::ParseBoolError> for UserError {
    fn from(err: str::ParseBoolError) -> UserError {
        UserError::ParseBool(err)
    }
}

impl From<num::ParseIntError> for UserError {
    fn from(err: num::ParseIntError) -> UserError {
        UserError::ParseInt(err)
    }
}

fn check_uid(uid: i32, user: &User, result: &mut test::TestResult) {
    let test_name = "user - uid";
    if user.uid == uid {
        let success = test::UnitSuccess {
            test: test_name.to_string(),
            expected: format!("{}", uid),
        };
        result.success +=1;
        result.summary.push(test::UnitResult::from(success))
    } else {
        let error = test::UnitError {
            test: test_name.to_string(),
            expected: format!("{}", uid),
            actual: format!("{}", user.uid),
            message: "incorrect uid".to_string(),
        };
        result.error +=1;
        result.summary.push(test::UnitResult::from(error))
    }
}

fn check_gid(gid: i32, user: &User, result: &mut test::TestResult) {
    let test_name = "user - gid";
    if user.gid == gid {
        let success = test::UnitSuccess {
            test: test_name.to_string(),
            expected: format!("{}", gid),
        };
        result.success +=1;
        result.summary.push(test::UnitResult::from(success))
    } else {
        let error = test::UnitError {
            test: test_name.to_string(),
            expected: format!("{}", gid),
            actual: format!("{}", user.gid),
            message: "incorrect uid".to_string(),
        };
        result.error +=1;
        result.summary.push(test::UnitResult::from(error))
    }
}

fn check_primary_group(group: &str, user: &User, result: &mut test::TestResult) {
    let test_name = "user - group";
    if user.group == group {
        let success = test::UnitSuccess {
            test: test_name.to_string(),
            expected: group.to_string(),
        };
        result.success +=1;
        result.summary.push(test::UnitResult::from(success))
    } else {
        let error = test::UnitError {
            test: test_name.to_string(),
            expected: group.to_string(),
            actual: user.group.to_string(),
            message: "incorrect primary group".to_string(),
        };
        result.error +=1;
        result.summary.push(test::UnitResult::from(error))
    }
}

fn check_secondary_groups(groups: &str, user: &User, result: &mut test::TestResult) {
    let test_name = "user - groups";
    let mut group_list: Vec<String> = groups.split(',').map(|s| s.to_string()).collect();
    group_list.sort();
    if group_list == user.groups {
        let success = test::UnitSuccess {
            test: test_name.to_string(),
            expected: groups.to_string(),
        };
        result.success +=1;
        result.summary.push(test::UnitResult::from(success))
    }
    else {
        let error = test::UnitError {
            test: test_name.to_string(),
            expected: groups.to_string(),
            actual: user.groups.join(","),
            message: "incorrect secondary groups".to_string(),
        };
        result.error +=1;
        result.summary.push(test::UnitResult::from(error))
    }

}

fn check_exists(user_result: &Option<User>, exists: bool, result: &mut test::TestResult) {
    match (user_result.is_some(), exists) {
         (true, false) => { // user exists but not exists wanted
             result.error += 1;
             let error = test::UnitError {
                 test: "user - exists".to_string(),
                 expected: "false".to_string(),
                 actual: "true".to_string(),
                 message: "user exists".to_string()
             };
             result.summary.push(test::UnitResult::from(error));
         },
        (false, false) => { // user doesnt exists and not exists wanted
            result.success +=1;
            let success = test::UnitSuccess {
                test: "user - exists".to_string(),
                expected: "false".to_string(),
            };
            result.summary.push(test::UnitResult::from(success));
        },
        (false, true) => { // user doesnt exists but exists wanted
            result.error += 1;
            let error = test::UnitError {
                test: "user - exists".to_string(),
                expected: "true".to_string(),
                actual: "false".to_string(),
                message: "user doesn't exists".to_string()
             };
             result.summary.push(test::UnitResult::from(error));
        },
        (true, true) => { // user exists and exists wanted
            result.success +=1;
            let success = test::UnitSuccess {
                test: "user - exists".to_string(),
                expected: "true".to_string(),
            };
            result.summary.push(test::UnitResult::from(success));
        }
    }
}

pub fn check(username: &str,
             exists: &str,
             uid: Option<&str>,
             gid: Option<&str>,
             group: Option<&str>,
             groups: Option<&str>)
             -> Result<test::TestResult, UserError> {

    let mut result = test::TestResult {
        success: 0,
        error: 0,
        summary: Vec::new()
    };
    let exists_bool: bool = try!(exists.parse());
    let user_result = try!(get_user(username));
    check_exists(&user_result, exists_bool, &mut result);
    match user_result {
        None => Ok(result),
        Some(user) => {
            let uid = try!(util::parse_int(uid));
            uid.map(|uid_int| {
                check_uid(uid_int, &user, &mut result);
            });
            let gid_int = try!(util::parse_int(gid)).unwrap_or(-1);
            if gid_int != -1 {
                check_gid(gid_int, &user, &mut result);
            };
            group.map(|g| {
                check_primary_group(g, &user, &mut result);
            });
            groups.map(|g| {
                check_secondary_groups(g, &user, &mut result);
            });

            Ok(result)
        }
    }
}

/// for a given username, returns an Option<User>
fn get_user(username: &str) -> Result<Option<User>, UserError> {
    let user_line = try!(get_user_line(username));
    match user_line {
        None => Ok(None),
        Some(l) => {
            let user = try!(parse_user_line(&l));
            Ok((Some(user)))
        }
    }
}

/// For a Vec<String> (representing a /etc/passwd line), returns the user
/// Also add the user groups informations
fn parse_user_line(user_line: &Vec<String>) -> Result<User, UserError> {
    let username = &user_line[0];
    let password = &user_line[1];
    let uid = try!(user_line[2].parse::<i32>());
    let gid = try!(user_line[3].parse::<i32>());
    let comment = &user_line[4];
    let home = &user_line[5];
    let init = &user_line[6];

    let primary_group_line = try!(group::get_group_line_from_gid(gid));
    let group = match primary_group_line {
        None => "".to_string(),
        Some(line) => line[0].to_string()
    };
    let mut secondary_groups = try!(group::get_user_secondary_groups(username));
    secondary_groups.sort();
    Ok(User {
        name: username.to_string(),
        password: password.to_string(),
        uid: uid,
        gid: gid,
        comment: comment.to_string(),
        home: home.to_string(),
        init: init.to_string(),
        group: group,
        groups: secondary_groups
    })

}

/// returns a Option<Vec<String>> representing a /etc/passwd line given an username
fn get_user_line(username: &str) -> Result<Option<Vec<String>>, Error> {
    let password_file = try!(File::open("/etc/passwd"));
    let reader = BufReader::new(password_file);
    let lines = reader.lines();
    for l in lines {
        let line = try!(l);
        let line_vec: Vec<&str> = line.trim().split(':').collect();
        if line_vec[0] == username {
            return Ok(Some(line_vec.iter()
                           .map(|v| v.to_string())
                           .collect()));
        }
    }
    Ok(None)

}


// TESTS


#[test]
fn get_user_line_test_success() {
    let result = get_user_line("root").unwrap();
    assert_eq!(result,
               Some(vec!["root".to_string(),
                         "x".to_string(),
                         "0".to_string(),
                         "0".to_string(),
                         "root".to_string(),
                         "/root".to_string(),
                         "/bin/bash".to_string()]));
    let result = get_user_line("not_exists").unwrap();
    assert!(result.is_none());
}

#[test]
fn check_uid_test_success() {
    let mut result = test::TestResult {
        success: 0,
        error: 0,
        summary: Vec::new()
    };
    let user = User {
        name: "root".to_string(),
        password: "x".to_string(),
        uid: 0,
        gid: 0,
        comment: "root".to_string(),
        home: "/root".to_string(),
        init: "/bin/bash".to_string(),
        group: "root".to_string(),
        groups: vec![]
    };
    check_uid(0, &user, &mut result);
    assert_eq!(result.success, 1);
    assert_eq!(result.error, 0);
    assert_eq!(result.summary.len(), 1);
    {
        let ref summary = result.summary[0];
        match summary {
            &test::UnitResult::Success(ref s) => assert_eq!(s.expected, "0"),
            &test::UnitResult::Error(_) => panic!("Error in test")
        }
    }
    check_uid(1, &user, &mut result);
    assert_eq!(result.success, 1);
    assert_eq!(result.error, 1);
    assert_eq!(result.summary.len(), 2);
    {
        let ref summary = result.summary[1];
        match summary {
            &test::UnitResult::Success(_) => panic!("Error in test"),
            &test::UnitResult::Error(ref s) => {
                assert_eq!(s.expected, "1");
                assert_eq!(s.actual, "0")
            }
        }
    }
}

#[test]
fn check_gid_test_success() {
    let mut result = test::TestResult {
        success: 0,
        error: 0,
        summary: Vec::new()
    };
    let user = User {
        name: "root".to_string(),
        password: "x".to_string(),
        uid: 0,
        gid: 0,
        comment: "root".to_string(),
        home: "/root".to_string(),
        init: "/bin/bash".to_string(),
        group: "root".to_string(),
        groups: vec![]
    };
    check_gid(0, &user, &mut result);
    assert_eq!(result.success, 1);
    assert_eq!(result.error, 0);
    assert_eq!(result.summary.len(), 1);
    {
        let ref summary = result.summary[0];
        match summary {
            &test::UnitResult::Success(ref s) => assert_eq!(s.expected, "0"),
            &test::UnitResult::Error(_) => panic!("Error in test")
        }
    }
    check_gid(1, &user, &mut result);
    assert_eq!(result.success, 1);
    assert_eq!(result.error, 1);
    assert_eq!(result.summary.len(), 2);
    {
        let ref summary = result.summary[1];
        match summary {
            &test::UnitResult::Success(_) => panic!("Error in test"),
            &test::UnitResult::Error(ref s) => {
                assert_eq!(s.expected, "1");
                assert_eq!(s.actual, "0")
            }
        }
    }
}

#[test]
fn parse_user_line_test_success() {
    let line = vec!["root".to_string(),
                    "x".to_string(),
                    "0".to_string(),
                    "0".to_string(),
                    "root".to_string(),
                    "/root".to_string(),
                    "/bin/bash".to_string()];
    let result = parse_user_line(&line);
    assert!(result.is_ok());
    let result_user = result.unwrap();
    assert_eq!(result_user.name, "root");
    assert_eq!(result_user.password, "x");
    assert_eq!(result_user.uid, 0);
    assert_eq!(result_user.gid, 0);
    assert_eq!(result_user.comment, "root");
    assert_eq!(result_user.home, "/root");
    assert_eq!(result_user.init, "/bin/bash");
    assert_eq!(result_user.group, "root");
    let groups: Vec<String> = Vec::new();
    assert_eq!(result_user.groups, groups);
}

#[test]
fn parse_user_line_test_error() {
    let line = vec!["root".to_string(),
                    "x".to_string(),
                    "0".to_string(),
                    "hello".to_string(),
                    "root".to_string(),
                    "/root".to_string(),
                    "/bin/bash".to_string()];
    let result = parse_user_line(&line);
    assert!(result.is_err());
}

#[test]
fn get_user_test_success() {
    let mut result = get_user("root");
    assert!(result.is_ok());
    let user_option = result.unwrap();
    let user = user_option.unwrap();
    assert_eq!(user.name, "root");
    assert_eq!(user.password, "x");
    assert_eq!(user.uid, 0);
    assert_eq!(user.gid, 0);
    assert_eq!(user.comment, "root");
    assert_eq!(user.home, "/root");
    assert_eq!(user.init, "/bin/bash");
    let groups: Vec<String> = Vec::new();
    assert_eq!(user.groups, groups);

    result = get_user("notexists");
    assert!(result.is_ok());
    let user_option = result.unwrap();
    assert!(user_option.is_none())
}

#[test]
fn check_exists_test_success() {
    let mut result = test::TestResult {
        success: 0,
        error: 0,
        summary: Vec::new()
    };
    let user = User {
        name: "root".to_string(),
        password: "x".to_string(),
        uid: 0,
        gid: 0,
        comment: "root".to_string(),
        home: "/root".to_string(),
        init: "/bin/bash".to_string(),
        group: "root".to_string(),
        groups: vec![]
    };
    let user_result = Some(user);
    check_exists(&user_result, true, &mut result);
    assert_eq!(result.success, 1);
    assert_eq!(result.error, 0);
    assert_eq!(result.summary.len(), 1);
    {
        let ref summary = result.summary[0];
        match summary {
            &test::UnitResult::Success(ref s) => {
                assert_eq!(s.expected, "true");
            }
            &test::UnitResult::Error(_) => panic!("Error in test")
        }
    }
    check_exists(&user_result, false, &mut result);
    assert_eq!(result.success, 1);
    assert_eq!(result.error, 1);
    assert_eq!(result.summary.len(), 2);
    {
        let ref summary = result.summary[1];
        match summary {
            &test::UnitResult::Success(_) => panic!("Error in test"),
            &test::UnitResult::Error(ref s) => {
                assert_eq!(s.actual, "true");
                assert_eq!(s.expected, "false");
            }
        }
    }
    check_exists(&None, false, &mut result);
    assert_eq!(result.success, 2);
    assert_eq!(result.error, 1);
    assert_eq!(result.summary.len(), 3);
    {
        let ref summary = result.summary[2];
        match summary {
            &test::UnitResult::Success(ref s) => assert_eq!(s.expected, "false"),
            &test::UnitResult::Error(_) => panic!("Error in test")
        }
    }
    check_exists(&None, true, &mut result);
    assert_eq!(result.success, 2);
    assert_eq!(result.error, 2);
    assert_eq!(result.summary.len(), 4);
    {
        let ref summary = result.summary[3];
        match summary {
            &test::UnitResult::Success(_) => panic!("Error in test"),
            &test::UnitResult::Error(ref s) => {
                assert_eq!(s.expected, "true");
                assert_eq!(s.actual, "false");
            }
        }
    }
}

#[test]
fn check_primary_group_test_success() {
    let mut result = test::TestResult {
        success: 0,
        error: 0,
        summary: Vec::new()
    };
    let user = User {
        name: "root".to_string(),
        password: "x".to_string(),
        uid: 0,
        gid: 0,
        comment: "root".to_string(),
        home: "/root".to_string(),
        init: "/bin/bash".to_string(),
        group: "root".to_string(),
        groups: vec![]
    };
    check_primary_group("root", &user, &mut result);
    assert_eq!(result.success, 1);
    assert_eq!(result.error, 0);
    assert_eq!(result.summary.len(), 1);
    {
        let ref summary = result.summary[0];
        match summary {
            &test::UnitResult::Success(ref s) => assert_eq!(s.expected, "root"),
            &test::UnitResult::Error(_) => panic!("Error in test")
        }
    }
    check_primary_group("notingroup", &user, &mut result);
    assert_eq!(result.success, 1);
    assert_eq!(result.error, 1);
    assert_eq!(result.summary.len(), 2);
    {
        let ref summary = result.summary[1];
        match summary {
            &test::UnitResult::Success(_) => panic!("Error in test"),
            &test::UnitResult::Error(ref s) => {
                assert_eq!(s.expected, "notingroup");
                assert_eq!(s.actual, "root");
            }
        }
    }
}


#[test]
fn check_secondary_groups_test_success() {
    let mut result = test::TestResult {
        success: 0,
        error: 0,
        summary: Vec::new()
    };
    let mut user = User {
        name: "root".to_string(),
        password: "x".to_string(),
        uid: 0,
        gid: 0,
        comment: "root".to_string(),
        home: "/root".to_string(),
        init: "/bin/bash".to_string(),
        group: "root".to_string(),
        groups: vec![]
    };
    check_secondary_groups("", &user, &mut result);
    assert_eq!(result.success, 0);
    assert_eq!(result.error, 1);
    assert_eq!(result.summary.len(), 1);
    {
        let ref summary = result.summary[0];
        match summary {
            &test::UnitResult::Success(_) => panic!("Error in test"),
            &test::UnitResult::Error(ref s) => {
                assert_eq!(s.expected, "");
                assert_eq!(s.actual, "") // strange
            }
        }
    }
    // TODO refactor this test
    user = User {
        name: "mathieu".to_string(),
        password: "x".to_string(),
        uid: 1000,
        gid: 1000,
        comment: "mathieu".to_string(),
        home: "/home/mathieu".to_string(),
        init: "/bin/bash".to_string(),
        group: "mathieu".to_string(),
        groups: vec!["cdrom".to_string(), "floppy".to_string()]
    };
    check_secondary_groups("cdrom", &user, &mut result);
    assert_eq!(result.success, 0);
    assert_eq!(result.error, 2);
    assert_eq!(result.summary.len(), 2);
    {
        let ref summary = result.summary[1];
        match summary {
            &test::UnitResult::Success(_) => panic!("Error in test"),
            &test::UnitResult::Error(ref s) => {
                assert_eq!(s.expected, "cdrom");
                assert_eq!(s.actual, "cdrom,floppy")
            }
        }
    }

    // TODO refactor this test
    user = User {
        name: "mathieu".to_string(),
        password: "x".to_string(),
        uid: 1000,
        gid: 1000,
        comment: "mathieu".to_string(),
        home: "/home/mathieu".to_string(),
        init: "/bin/bash".to_string(),
        group: "mathieu".to_string(),
        groups: vec!["cdrom".to_string(), "floppy".to_string()]
    };
    check_secondary_groups("cdrom,floppy", &user, &mut result);
    assert_eq!(result.success, 1);
    assert_eq!(result.error, 2);
    assert_eq!(result.summary.len(), 3);
    {
        let ref summary = result.summary[2];
        match summary {
            &test::UnitResult::Success(ref s) => assert_eq!(s.expected, "cdrom,floppy"),
            &test::UnitResult::Error(_) => panic!("Error in test")
        }
    }
}


#[test]
fn check_test_success() {
    let mut result = check("root", "true", None, None, None, None).unwrap();
    assert_eq!(result.error, 0);
    assert_eq!(result.success, 1);
    assert_eq!(result.summary.len(), 1);
    {
        let ref summary = result.summary[0];
        match summary {
            &test::UnitResult::Success(ref s) => assert_eq!(s.expected, "true"),
            &test::UnitResult::Error(_) => panic!("Error in test")
        }
    }
    result = check("root", "true", Some("0"), None, None, None).unwrap();
    assert_eq!(result.error, 0);
    assert_eq!(result.success, 2);
    assert_eq!(result.summary.len(), 2);
    {
        let ref summary = result.summary[1];
        match summary {
            &test::UnitResult::Success(ref s) => assert_eq!(s.expected, "0"),
            &test::UnitResult::Error(_) => panic!("Error in test")
        }
    }
    result = check("root", "true", Some("1"), None, None, None).unwrap();
    assert_eq!(result.error, 1);
    assert_eq!(result.success, 1);
    assert_eq!(result.summary.len(), 2);
    {
        let ref summary = result.summary[1];
        match summary {
            &test::UnitResult::Success(_) => panic!("Error in test"),
            &test::UnitResult::Error(ref err) => {
                assert_eq!(err.expected, "1");
                assert_eq!(err.actual, "0")
            }
        }
    }
}

#[test]
fn check_test_error() {
    let mut result = check("root", "true", Some("hello"), None, None, None);
    assert!(result.is_err());
    result = check("root", "hello", Some("0"), None, None, None);
    assert!(result.is_err());
}



