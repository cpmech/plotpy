use super::*;
use std::fs;
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
/// # Examples
///
/// ```
/// use rplotpy::*;
/// let res = call_python3("print(\"Hello World!\")", "/tmp/rplotpy", "commands.py").unwrap();
/// assert_eq!(res, "Hello World!\n".to_string());
/// ```
///
pub fn call_python3(
    python_commands: &str,
    output_dir: &str,
    filename_py: &str,
) -> std::io::Result<String> {
    // create directory
    fs::create_dir_all(output_dir)?;

    // combine header with commands
    let mut contents = String::new();
    contents.push_str(PYTHON_HEADER);
    contents.push_str(python_commands);

    // write file
    let filepath = Path::new(output_dir).join(filename_py);
    fs::write(&filepath, contents)?;

    // execute file
    let output = Command::new("python3").arg(&filepath).output()?;

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
    use super::*;

    #[test]
    fn run_works() -> Result<(), Box<dyn std::error::Error>> {
        let commands = "print(\"Python says: Hello World!\")";
        let out_dir = "/tmp/rplotpy";
        let filename = "test.py";
        let output = call_python3(commands, out_dir, filename)?;
        let data = fs::read_to_string(Path::new(out_dir).join(filename))?;
        let mut correct = String::from(PYTHON_HEADER);
        correct.push_str(&commands);
        assert_eq!(data, correct);
        assert_eq!(output, "Python says: Hello World!\n");
        Ok(())
    }
}
