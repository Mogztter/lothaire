use std::fs;
use std::num;
use std::io::prelude::*;
use std::io;

#[derive(Debug)]
pub enum GroupError {
    ParseInt(num::ParseIntError),
    Io(io::Error)
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
