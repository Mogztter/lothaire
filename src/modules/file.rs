use modules::test;
use std::fs;
use std::io;

fn get_metadata(path: &str) -> Result<fs::Metadata, io::Error> {
    let metadata = try!(fs::metadata(path));
    Ok(metadata)
}

fn is_file(metadata: &fs::Metadata, result: &mut test::TestResult) {
    let test_name = "file - type";
    if metadata.is_file() {
        let success = test::UnitSuccess {
            test: test_name.to_string(),
            expected: format!("is a file: true"),
        };
        result.success +=1;
        result.summary.push(test::UnitResult::from(success))
    }
    else {
        let error = test::UnitError {
            test: test_name.to_string(),
            expected: format!("is a file: true"),
            actual: format!("is a directory: {}", metadata.is_dir()),
            message: "The file is a directory".to_string(),
        };
        result.error +=1;
        result.summary.push(test::UnitResult::from(error))
    }
}
