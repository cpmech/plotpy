use super::*;
use std::fs;
use std::path::Path;
use std::process::Command;

fn run(python_commands: &str, filename: &str) -> std::io::Result<String> {
    // create directory
    fs::create_dir_all(DIR_TEMPORARY)?;

    // combine header with commands
    let mut contents = String::new();
    contents.push_str(PYTHON_HEADER);
    contents.push_str(python_commands);

    // write file
    let filepath = Path::new(DIR_TEMPORARY).join(filename);
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
    use std::io;

    #[test]
    fn run_works() -> Result<(), Box<dyn std::error::Error>> {
        let commands = "print(\"Python says: Hello World!\")".to_string();
        let output = run(&commands, "generate.py")?;
        let entries = fs::read_dir(DIR_TEMPORARY)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;
        assert_eq!(entries.len(), 1);
        let data = fs::read_to_string(Path::new(DIR_TEMPORARY).join("generate.py"))?;
        let mut correct = String::from(PYTHON_HEADER);
        correct.push_str(&commands);
        assert_eq!(data, correct);
        assert_eq!(output, "Python says: Hello World!\n");
        Ok(())
    }
}
