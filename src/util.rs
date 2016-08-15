use std::num;
use std::str;

pub fn parse_int(value: Option<&str>) -> Result<Option<i32>, num::ParseIntError> {
    match value {
        None => Ok(None),
        Some(v) => {
            let result = try!(v.parse::<i32>());
            Ok(Some(result))
        }
    }
}

pub fn parse_bool(value: Option<&str>) -> Result<Option<bool>, str::ParseBoolError> {
    match value {
        None => Ok(None),
        Some(v) => {
            let result = try!(v.parse::<bool>());
            Ok(Some(result))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_int_test_success() {
        let result = parse_int(Some("3")).unwrap();
        assert_eq!(result, Some(3));
        let result = parse_int(Some("42")).unwrap();
        assert_eq!(result, Some(42));
        let result = parse_int(None).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn parse_int_test_error() {
        let result = parse_int(Some("hello"));
        assert!(result.is_err());
        let result = parse_int(Some("1.3"));
        assert!(result.is_err());
    }

    #[test]
    fn parse_bool_test_success() {
        let result = parse_bool(Some("true")).unwrap();
        assert_eq!(result, Some(true));
        let result = parse_bool(Some("false")).unwrap();
        assert_eq!(result, Some(false));
        let result = parse_bool(None).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn parse_bool_test_error() {
        let result = parse_bool(Some("hello"));
        assert!(result.is_err());
        let result = parse_bool(Some("1"));
        assert!(result.is_err());
    }
}
