use super::*;
use std::path::Path;

pub struct Plot {
    buffer: String,
}

impl Plot {
    pub fn new() -> Self {
        Plot {
            buffer: String::new(),
        }
    }

    /// Saves figure to disk
    ///
    /// # Arguments
    ///
    /// * `output_dir` - Creates a directory to save the figure, and temporary files
    /// * `filename_key` - The filename without extension
    /// * `filename_ext` - The extension of the filename; e.g., "png" or "svg"
    pub fn save(
        &self,
        output_dir: &str,
        filename_key: &str,
        filename_ext: &str,
    ) -> std::io::Result<String> {
        // filename
        let ext = filename_ext.replace(".", "");
        let filename_py = format!("{}.py", filename_key);
        let filename_fig = format!("{}.{}", filename_key, ext);
        let filepath_fig = Path::new(output_dir).join(filename_fig);

        // update commands
        let commands = format!(
            "{}\nfn='{}'\nplt.savefig(fn, bbox_inches='tight', bbox_extra_artists=EXTRA_ARTISTS)\nprint(f'figure {{fn}} created',fn)\n",
            self.buffer,
            filepath_fig.to_string_lossy(),
        );

        // call python
        let output = call_python3(&commands, output_dir, &filename_py)?;
        println!("{}", output);

        // done
        Ok("".to_string())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn new_plot_works() -> Result<(), Box<dyn std::error::Error>> {
        let plt = Plot::new();
        assert_eq!(plt.buffer.len(), 0);
        plt.save("/tmp/rplotpy", "test", "svg")?;
        let svg = fs::read_to_string("/tmp/rplotpy/test.svg")?;
        let lines = svg.lines().collect::<Vec<_>>();
        assert_eq!(lines.len(), 33);
        Ok(())
    }
}
