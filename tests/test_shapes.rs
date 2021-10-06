use plotpy::{Plot, Shapes};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integ_tests";

#[test]
fn test_shapes() -> Result<(), &'static str> {
    // shapes object and common options
    let mut shapes = Shapes::new();
    shapes.set_edge_color("#cd0000").set_face_color("#1862ab");

    // draw arc
    shapes.draw_arc(0.5, 0.5, 0.4, 195.0, -15.0);

    // draw arrow
    shapes.set_arrow_scale(50.0).set_arrow_style("fancy");
    shapes.draw_arrow(0.0, 0.0, 1.0, 1.0);

    // draw circle
    shapes.set_face_color("None").set_edge_color("grey");
    shapes.draw_circle(0.5, 0.5, 0.5);

    // draw polyline
    shapes.set_edge_color("blue");
    let a = 0.2;
    let c = f64::sqrt(3.0) / 2.0;
    let p = vec![vec![0.1, 0.5], vec![0.1 + a, 0.5], vec![0.1 + a / 2.0, 0.5 + a * c]];
    let q = vec![vec![0.9, 0.5], vec![0.9 - a, 0.5], vec![0.9 - a / 2.0, 0.5 + a * c]];
    shapes.draw_polyline(&p, true);
    shapes.draw_polyline(&q, false);

    // add shapes to plot
    let mut plot = Plot::new();
    plot.add(&shapes);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_shapes.svg");
    plot.set_equal_axes(true);
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 450);
    Ok(())
}
