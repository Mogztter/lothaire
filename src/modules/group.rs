use std::fs;
use std::num;
use std::io::prelude::*;
use std::io;
use modules::test;
use std::str;
use util;

#[derive(Debug)]
pub enum GroupError {
    ParseInt(num::ParseIntError),
    Io(io::Error),
    ParseBool(str::ParseBoolError)
}


impl From<io::Error> for GroupError {
    fn from(err: io::Error) -> GroupError {
        GroupError::Io(err)
    }
}

impl From<num::ParseIntError> for GroupError {
    fn from(err: num::ParseIntError) -> GroupError {
        GroupError::ParseInt(err)
    }
}

impl From<str::ParseBoolError> for GroupError {
    fn from(err: str::ParseBoolError) -> GroupError {
        GroupError::ParseBool(err)
    }
}

#[derive(Debug)]
pub struct Group {
    pub name: String,
    pub password: String,
    pub gid: i32
}

pub fn get_group_line_from_gid(gid: i32) -> Result<Option<Vec<String>>, GroupError> {
    let group_file = try!(fs::File::open("/etc/group"));
    let reader = io::BufReader::new(group_file);
    let lines = reader.lines();
    for l in lines {
        let line = try!(l);
        let line_vec: Vec<&str> = line.trim().split(':').collect();
        let line_gid = try!(line_vec[2].parse::<i32>());
        if line_gid == gid {
            return Ok(Some(line_vec.iter()
                .map(|v| v.to_string())
                .collect()));
        }
    }
    Ok(None)
}

pub fn parse_group_line(line: &Vec<String>) -> Result<Group, num::ParseIntError> {
    let gid = try!(line[2].parse::<i32>());
    let name = &line[0];
    let password = &line[1];
    Ok(Group {
        name: name.to_string(),
        password: password.to_string(),
        gid: gid
    })
}

fn check_gid(gid: i32, group: &Group, result: &mut test::TestResult) {
    let test_name = "group - gid";
    if group.gid == gid {
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
            actual: format!("{}", group.gid),
            message: "incorrect gid".to_string(),
        };
        result.error +=1;
        result.summary.push(test::UnitResult::from(error))
    }
}

pub fn check(name: &str,
             exists: &str,
             gid: Option<&str>) -> Result<test::TestResult, GroupError> {
    let mut result = test::TestResult {
        success: 0,
        error: 0,
        summary: Vec::new()
    };
    let exists_bool: bool = try!(exists.parse());
    let group_result = try!(get_group_from_name(name));
    test::check_exists(&group_result, exists_bool, &mut result, "group - exists".to_string());
    match group_result {
        None => Ok(result),
        Some(group) => {
            let gid = try!(util::parse_int(gid));
            gid.map(|gid_int| {
                check_gid(gid_int, &group, &mut result);
            });
            Ok(result)
        }

    }
}

pub fn get_group_from_name(name: &str) -> Result<Option<Group>, GroupError> {
    let group_line = try!(get_group_line_from_name(name));
    match group_line {
        None => Ok(None),
        Some(l) => {
            let group = try!(parse_group_line(&l));
            Ok(Some(group))
        }
    }
}

pub fn get_group_from_gid(gid: i32) -> Result<Option<Group>, GroupError> {
    let group_line = try!(get_group_line_from_gid(gid));
    match group_line {
        None => Ok(None),
        Some(l) => {
            let group = try!(parse_group_line(&l));
            Ok(Some(group))
        }
    }
}

pub fn get_group_line_from_name(name: &str) -> Result<Option<Vec<String>>, io::Error> {
    let group_file = try!(fs::File::open("/etc/group"));
    let reader = io::BufReader::new(group_file);
    let lines = reader.lines();
    for l in lines {
        let line = try!(l);
        let line_vec: Vec<&str> = line.trim().split(':').collect();
        if line_vec[0] == name {
            return Ok(Some(line_vec.iter()
                .map(|v| v.to_string())
                .collect()));
        }
    }
    Ok(None)
}

pub fn get_user_secondary_groups(username: &str) -> Result<Vec<String>, io::Error> {
    let group_file = try!(fs::File::open("/etc/group"));
    let reader = io::BufReader::new(group_file);
    let lines = reader.lines();
    let mut result = Vec::new();
    for l in lines {
        let line = try!(l);
        let line_vec: Vec<&str> = line.trim().split(':').collect();
        if line_vec[3] != "" {
            let groups_vec: Vec<&str> = line_vec[3].split(',').collect();
            if groups_vec.contains(&username) {
                result.push(line_vec[0].to_string());
            }
         }
    }
    Ok(result)
}


// TESTS


#[test]
fn get_group_line_from_gid_test_success() {
    let mut result = get_group_line_from_gid(0).unwrap();
    assert_eq!(result,
               Some(vec!["root".to_string(), "x".to_string(), "0".to_string(), "".to_string()]));
    result = get_group_line_from_gid(999999).unwrap();
    assert!(result.is_none());
}

#[test]
fn get_user_secondary_groups_test_success() {
    let mut result = get_user_secondary_groups("root").unwrap();
    assert!(result.is_empty());
    result = get_user_secondary_groups("user1").unwrap();
    assert_eq!(result.len(), 2);
    assert!(result.contains(&"group2".to_string()));
}


#[test]
fn get_group_line_from_name_test_success() {
    let mut result = get_group_line_from_name("group1").unwrap();
    assert_eq!(result,
               Some(vec!["group1".to_string(), "x".to_string(), "2001".to_string(), "".to_string()]));
    result = get_group_line_from_name("foobargroup").unwrap();
    assert!(result.is_none());
}


#[test]
fn parse_group_line_test_success() {
    let line = vec!["group1".to_string(),
                    "x".to_string(),
                    "2001".to_string()];
    let result = parse_group_line(&line);
    assert!(result.is_ok());
    let group = result.unwrap();
    assert_eq!(group.name, "group1");
    assert_eq!(group.password, "x");
    assert_eq!(group.gid, 2001);
}


#[test]
fn parse_group_line_test_error() {
    let line = vec!["group1".to_string(),
                    "x".to_string(),
                    "hello".to_string()];
    let result = parse_group_line(&line);
    assert!(result.is_err());

}


#[test]
fn get_group_from_name_test_success() {
    let mut group = get_group_from_name("group1").unwrap().unwrap();
    assert_eq!(group.name, "group1");
    assert_eq!(group.password, "x");
    assert_eq!(group.gid, 2001);
    group = get_group_from_name("group2").unwrap().unwrap();
    assert_eq!(group.name, "group2");
    assert_eq!(group.password, "x");
    assert_eq!(group.gid, 2002);
    let group_opt = get_group_from_name("notexists").unwrap();
    assert!(group_opt.is_none())

}

#[test]
fn get_group_from_gid_test_success() {
    let mut group = get_group_from_gid(2001).unwrap().unwrap();
    assert_eq!(group.name, "group1");
    assert_eq!(group.password, "x");
    assert_eq!(group.gid, 2001);
    group = get_group_from_gid(2002).unwrap().unwrap();
    assert_eq!(group.name, "group2");
    assert_eq!(group.password, "x");
    assert_eq!(group.gid, 2002);
    let group_opt = get_group_from_gid(99999).unwrap();
    assert!(group_opt.is_none())

}

#[test]
fn check_gid_test_success() {
    let mut result = test::TestResult {
        success: 0,
        error: 0,
        summary: Vec::new()
    };
    let group = Group {
        name: "group1".to_string(),
        password: "x".to_string(),
        gid: 2001
    };
    check_gid(2001, &group, &mut result);
    assert_eq!(result.success, 1);
    assert_eq!(result.error, 0);
    assert_eq!(result.summary.len(), 1);
    {
        let ref summary = result.summary[0];
        match summary {
            &test::UnitResult::Success(ref s) => assert_eq!(s.expected, "2001"),
            &test::UnitResult::Error(_) => panic!("Error in test")
        }
    }
    check_gid(1, &group, &mut result);
    assert_eq!(result.success, 1);
    assert_eq!(result.error, 1);
    assert_eq!(result.summary.len(), 2);
    {
        let ref summary = result.summary[1];
        match summary {
            &test::UnitResult::Success(_) => panic!("Error in test"),
            &test::UnitResult::Error(ref s) => {
                assert_eq!(s.expected, "1");
                assert_eq!(s.actual, "2001")
            }
        }
    }
}


#[test]
fn check_test_success() {
    let mut result = check("group1", "true", None).unwrap();
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
    result = check("group1", "true", Some("2001")).unwrap();
    assert_eq!(result.error, 0);
    assert_eq!(result.success, 2);
    assert_eq!(result.summary.len(), 2);
    {
        let ref summary = result.summary[1];
        match summary {
            &test::UnitResult::Success(ref s) => assert_eq!(s.expected, "2001"),
            &test::UnitResult::Error(_) => panic!("Error in test")
        }
    }
    result = check("group1", "true", Some("2002")).unwrap();
    assert_eq!(result.error, 1);
    assert_eq!(result.success, 1);
    assert_eq!(result.summary.len(), 2);
    {
        let ref summary = result.summary[1];
        match summary {
            &test::UnitResult::Success(_) => panic!("Error in test"),
            &test::UnitResult::Error(ref err) => {
                assert_eq!(err.expected, "2002");
                assert_eq!(err.actual, "2001")
            }
        }
    }
    result = check("dontexists", "false", Some("2000")).unwrap();
    assert_eq!(result.error, 0);
    assert_eq!(result.success, 1);
    assert_eq!(result.summary.len(), 1);
    {
        let ref summary = result.summary[0];
        match summary {
            &test::UnitResult::Success(ref s) => assert_eq!(s.expected, "false"),
            &test::UnitResult::Error(_) => panic!("Error in test")
        }
    }
}
