use super::*;
use std::fs;
use std::path::Path;
use std::process::Command;

fn run(filename: &str) -> std::io::Result<()> {
    // create directory
    fs::create_dir_all(DIR_TEMPORARY)?;

    // write file
    let filepath = Path::new(DIR_TEMPORARY).join(filename);
    fs::write(&filepath, PYTHON_HEADER)?;

    // execute file
    Command::new("python3").arg(&filepath).output()?;

    // done
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn run_works() -> Result<(), Box<dyn std::error::Error>> {
        run("generate.py")?;
        let entries = fs::read_dir(DIR_TEMPORARY)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;
        assert_eq!(entries.len(), 1);
        let data = fs::read_to_string(Path::new(DIR_TEMPORARY).join("generate.py"))?;
        let correct = String::from(PYTHON_HEADER);
        assert_eq!(data, correct);
        Ok(())
    }
}
