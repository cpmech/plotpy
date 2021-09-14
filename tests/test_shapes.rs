use plotpy::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integration_tests";

#[test]
fn test_shapes() -> Result<(), &'static str> {
    // shapes object and options
    let mut shapes = Shapes::new();
    shapes.edge_color = "#cd0000".to_string();
    shapes.face_color = "#1862ab".to_string();

    // draw shapes
    shapes.arrow_scale = 50.0;
    shapes.arrow_style = "fancy".to_string();
    shapes.arrow(0.0, 0.0, 1.0, 1.0);

    // add shapes to plot
    let mut plot = Plot::new();
    plot.add(&shapes);

    // save figure
    let path = Path::new(OUT_DIR).join("shapes.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert_eq!(lines_iter.count(), 437);
    Ok(())
}
