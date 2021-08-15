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

pub(crate) fn make_svg(data: &str, matplotlib_version: &str) -> String {
    format!(
        r#"<?xml version="1.0" encoding="utf-8" standalone="no"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN"
  "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<!-- Created with matplotlib (https://matplotlib.org/) -->
<svg height="360pt" version="1.1" viewBox="0 0 475.2 360" width="475.2pt" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
 <metadata>
  <rdf:RDF xmlns:cc="http://creativecommons.org/ns#" xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns:rdf="http://www.w3.org/1999/02/22-rdf-syntax-ns#">
   <cc:Work>
    <dc:type rdf:resource="http://purl.org/dc/dcmitype/StillImage"/>
    <dc:date>2021-08-15T18:48:19.889191</dc:date>
    <dc:format>image/svg+xml</dc:format>
    <dc:creator>
     <cc:Agent>
      <dc:title>Matplotlib {}, https://matplotlib.org/</dc:title>
     </cc:Agent>
    </dc:creator>
   </cc:Work>
  </rdf:RDF>
 </metadata>
 <defs>
  <style type="text/css">*{{stroke-linecap:butt;stroke-linejoin:round;}}</style>
 </defs>{}</svg>
"#,
        matplotlib_version, data
    )
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

    #[test]
    fn make_svg_works() {
        let data = r#"
 <g id="figure_1">
  <g id="patch_1">
   <path d="M 0 360 
L 475.2 360 
L 475.2 0 
L 0 0 
z
" style="fill:#ffffff;"/>
  </g>
 </g>
 "#;
        let res = make_svg(data, "v3.3.4");
        let lines = res.lines().collect::<Vec<_>>();
        assert_eq!(lines.len(), 33);
    }
}
