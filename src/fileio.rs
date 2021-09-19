use super::PYTHON_HEADER;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

/// Writes a python file and call python3 on it
///
/// # Arguments
///
/// * `python_commands` - Python commands to be written to file
/// * `output_dir` - Output directory to be created
/// * `filename_py` - Filename with extension .py
///
/// # Note
///
/// The contents of PYTHON_HEADER are added at the beginning of the file.
///
pub(crate) fn call_python3(python_commands: &String, path: &Path) -> Result<String, &'static str> {
    // create directory
    if let Some(p) = path.parent() {
        fs::create_dir_all(p).map_err(|_| "cannot create directory")?;
    }

    // combine header with commands
    let mut contents = String::new();
    contents.push_str(PYTHON_HEADER);
    contents.push_str(python_commands);

    // write file
    let mut file = File::create(path).map_err(|_| "cannot create file")?;
    file.write_all(contents.as_bytes()).map_err(|_| "cannot write file")?;

    // force sync
    file.sync_all().map_err(|_| "cannot sync file")?;

    // execute file
    let output = Command::new("python3")
        .arg(path)
        .output()
        .map_err(|_| "cannot run python3")?;

    // results
    let out = match String::from_utf8(output.stdout) {
        Ok(v) => v,
        Err(e) => format!("ERROR: cannot convert command line stdout\n{}", e),
    };
    let err = match String::from_utf8(output.stderr) {
        Ok(v) => v,
        Err(e) => format!("ERROR: cannot convert command line stderr\n{}", e),
    };
    let mut results = String::new();
    if out.len() > 0 {
        results.push_str(&out);
    }
    if err.len() > 0 {
        results.push_str(&err)
    }

    // done
    Ok(results)
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::{call_python3, PYTHON_HEADER};
    use std::fs;
    use std::path::Path;

    const OUT_DIR: &str = "/tmp/plotpy/unit_tests";

    #[test]
    fn call_python3_works() -> Result<(), &'static str> {
        let commands = "print(\"Python says: Hello World!\")".to_string();
        let path = Path::new("call_python3_works.py");
        let output = call_python3(&commands, &path)?;
        let data = fs::read_to_string(&path).map_err(|_| "cannot read test file")?;
        let mut correct = String::from(PYTHON_HEADER);
        correct.push_str(&commands);
        assert_eq!(data, correct);
        assert_eq!(output, "Python says: Hello World!\n");
        Ok(())
    }

    #[test]
    fn call_python3_create_dir_works() -> Result<(), &'static str> {
        let commands = "print(\"Python says: Hello World!\")".to_string();
        let path = Path::new(OUT_DIR).join("call_python3_works.py");
        let output = call_python3(&commands, &path)?;
        let data = fs::read_to_string(&path).map_err(|_| "cannot read test file")?;
        let mut correct = String::from(PYTHON_HEADER);
        correct.push_str(&commands);
        assert_eq!(data, correct);
        assert_eq!(output, "Python says: Hello World!\n");
        Ok(())
    }

    #[test]
    fn call_python3_twice_works() -> Result<(), &'static str> {
        let path = Path::new(OUT_DIR).join("call_python3_twice_works.py");
        // first
        let commands_first = "print(\"Python says: Hello World!\")".to_string();
        let output_first = call_python3(&commands_first, &path)?;
        let data_first = fs::read_to_string(&path).map_err(|_| "cannot read test file")?;
        let mut correct_first = String::from(PYTHON_HEADER);
        correct_first.push_str(&commands_first);
        assert_eq!(data_first, correct_first);
        assert_eq!(output_first, "Python says: Hello World!\n");
        // second
        let commands_second = "print(\"Python says: Hello World! again\")".to_string();
        let output_second = call_python3(&commands_second, &path)?;
        let data_second = fs::read_to_string(&path).map_err(|_| "cannot read test file")?;
        let mut correct_second = String::from(PYTHON_HEADER);
        correct_second.push_str(&commands_second);
        assert_eq!(data_second, correct_second);
        assert_eq!(output_second, "Python says: Hello World! again\n");
        Ok(())
    }
}
