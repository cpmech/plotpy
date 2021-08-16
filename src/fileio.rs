use super::*;
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
pub(crate) fn call_python3(
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
    let mut file = File::create(&filepath)?;
    file.write_all(contents.as_bytes())?;
    file.sync_all()?;

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
    fn call_python3_works() -> Result<(), Box<dyn std::error::Error>> {
        let commands = "print(\"Python says: Hello World!\")";
        let out_dir = "/tmp/plotpy";
        let filename = "call_python3_works.py";
        let output = call_python3(commands, out_dir, filename)?;
        let data = fs::read_to_string(Path::new(out_dir).join(filename))?;
        let mut correct = String::from(PYTHON_HEADER);
        correct.push_str(&commands);
        assert_eq!(data, correct);
        assert_eq!(output, "Python says: Hello World!\n");
        Ok(())
    }

    #[test]
    fn call_python3_twice_works() -> Result<(), Box<dyn std::error::Error>> {
        let out_dir = "/tmp/plotpy";
        let filename = "call_python3_twice_works.py";
        // first
        let commands_first = "print(\"Python says: Hello World!\")";
        let output_first = call_python3(commands_first, out_dir, filename)?;
        let data_first = fs::read_to_string(Path::new(out_dir).join(filename))?;
        let mut correct_first = String::from(PYTHON_HEADER);
        correct_first.push_str(&commands_first);
        assert_eq!(data_first, correct_first);
        assert_eq!(output_first, "Python says: Hello World!\n");
        // second
        let commands_second = "print(\"Python says: Hello World! again\")";
        let output_second = call_python3(commands_second, out_dir, filename)?;
        let data_second = fs::read_to_string(Path::new(out_dir).join(filename))?;
        let mut correct_second = String::from(PYTHON_HEADER);
        correct_second.push_str(&commands_second);
        assert_eq!(data_second, correct_second);
        assert_eq!(output_second, "Python says: Hello World! again\n");
        Ok(())
    }
}
