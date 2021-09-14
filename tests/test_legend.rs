use plotpy::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integration_tests";

#[test]
fn test_legend() -> Result<(), &'static str> {
    // curve and options
    let mut curve = Curve::new();
    curve.label = "my-curve".to_string();

    // draw curve
    let x = &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let y = &[
        1.0, 1.41421356, 1.73205081, 2.0, 2.23606798, 2.44948974, 2.64575131, 2.82842712, 3.0, 3.16227766,
    ];
    curve.draw(x, y);

    // legend and options
    let mut legend = Legend::new();
    legend.show_frame = false;
    legend.outside = true;

    // draw legend
    legend.draw();

    // add curve and legend to plot
    let mut plot = Plot::new();
    plot.add(&curve);
    plot.add(&legend); // must be after a (labelled) curve

    // save figure
    let path = Path::new(OUT_DIR).join("legend.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert_eq!(lines_iter.count(), 637);
    Ok(())
}
