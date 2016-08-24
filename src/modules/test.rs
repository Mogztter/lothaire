#[derive(Debug)]
pub struct UnitError {
    pub test: String,
    pub expected: String,
    pub actual: String,
    pub message: String,
}

#[derive(Debug)]
pub struct UnitSuccess {
    pub test: String,
    pub expected: String,
}

#[derive(Debug)]
pub enum UnitResult {
    Error(UnitError),
    Success(UnitSuccess)
}

impl From<UnitError> for UnitResult {
    fn from(err: UnitError) -> UnitResult {
        UnitResult::Error(err)
    }
}

impl From<UnitSuccess> for UnitResult {
    fn from(suc: UnitSuccess) -> UnitResult {
        UnitResult::Success(suc)
    }
}

#[derive(Debug)]
pub struct TestResult {
    pub success: i32,
    pub error: i32,
    pub summary: Vec<UnitResult>
}

pub fn check_exists<T>(value: &Option<T>, exists: bool, result: &mut TestResult, test_name: String) {
    match (value.is_some(), exists) {
         (true, false) => { // user exists but not exists wanted
             result.error += 1;
             let error = UnitError {
                 test: test_name,
                 expected: "false".to_string(),
                 actual: "true".to_string(),
                 message: "the element exists".to_string()
             };
             result.summary.push(UnitResult::from(error));
         },
        (false, false) => { // user doesnt exists and not exists wanted
            result.success +=1;
            let success = UnitSuccess {
                test: test_name,
                expected: "false".to_string(),
            };
            result.summary.push(UnitResult::from(success));
        },
        (false, true) => { // user doesnt exists but exists wanted
            result.error += 1;
            let error = UnitError {
                test: test_name,
                expected: "true".to_string(),
                actual: "false".to_string(),
                message: "the element doesn't exists".to_string()
             };
             result.summary.push(UnitResult::from(error));
        },
        (true, true) => { // user exists and exists wanted
            result.success +=1;
            let success = UnitSuccess {
                test: test_name,
                expected: "true".to_string(),
            };
            result.summary.push(UnitResult::from(success));
        }
    }
}
