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

