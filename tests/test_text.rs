use plotpy::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integration_tests";

#[test]
fn test_text() -> Result<(), &'static str> {
    // text object and options
    let mut text = Text::new();
    text.set_color("blue")
        .set_align_horizontal("center")
        .set_align_vertical("center")
        .set_fontsize(50.0)
        .set_rotation(45.0);

    // draw text
    text.draw(0.5, 0.5, "message");

    // add text to plot
    let mut plot = Plot::new();
    plot.add(&text);

    // save figure
    let path = Path::new(OUT_DIR).join("text.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 560);
    Ok(())
}
