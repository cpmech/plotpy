use plotpy::{Curve, Plot, StrError};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integ_tests";

#[test]
fn test_subplot() -> Result<(), StrError> {
    // curves
    let mut curve1 = Curve::new();
    let mut curve2 = Curve::new();
    let x = &[1.0, 2.0, 3.0, 4.0];
    let y = &[1.0, 1.424, 1.732, 2.0];
    let z = &[1.0, 4.0, 9.0, 16.0];
    curve1.draw(x, y);
    curve2.draw(x, z);

    // plot and subplots
    let mut plot = Plot::new();
    plot.set_subplot(1, 2, 1).add(&curve1);
    plot.set_subplot(1, 2, 2).add(&curve2);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_subplot_before.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let n = lines_iter.count();
    assert!(n > 600);
    assert!(n < 650);

    // clear current axis
    plot.set_subplot(1, 2, 1).clear_current_axis();

    // save figure again
    let path = Path::new(OUT_DIR).join("integ_subplot_after.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let n = lines_iter.count();
    assert!(n > 650);
    assert!(n < 680);
    Ok(())
}
