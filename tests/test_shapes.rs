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

    // draw arc
    shapes.arc(0.5, 0.5, 0.4, 195.0, -15.0);

    // draw arrow
    shapes.arrow_scale = 50.0;
    shapes.arrow_style = "fancy".to_string();
    shapes.arrow(0.0, 0.0, 1.0, 1.0);

    // draw circle
    shapes.face_color = "None".to_string();
    shapes.edge_color = "grey".to_string();
    shapes.circle(0.5, 0.5, 0.5);

    // draw polyline
    shapes.edge_color = "blue".to_string();
    let a = 0.2;
    let c = f64::sqrt(3.0) / 2.0;
    let p = vec![vec![0.1, 0.5], vec![0.1 + a, 0.5], vec![0.1 + a / 2.0, 0.5 + a * c]];
    let q = vec![vec![0.9, 0.5], vec![0.9 - a, 0.5], vec![0.9 - a / 2.0, 0.5 + a * c]];
    shapes.polyline(&p, true);
    shapes.polyline(&q, false);

    // add shapes to plot
    let mut plot = Plot::new();
    plot.add(&shapes);

    // save figure
    let path = Path::new(OUT_DIR).join("shapes.svg");
    plot.equal();
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert_eq!(lines_iter.count(), 506);
    Ok(())
}
