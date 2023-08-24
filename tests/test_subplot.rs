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

    // clear the current axes
    plot.set_subplot(1, 2, 1).clear_current_axes();

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

#[test]
fn test_gridspec_1() -> Result<(), StrError> {
    // curves
    let mut curve1 = Curve::new();
    let mut curve2 = Curve::new();
    let x = &[1.0, 2.0, 3.0, 4.0];
    let y = &[1.0, 1.424, 1.732, 2.0];
    let z = &[1.0, 4.0, 9.0, 16.0];
    curve1.draw(x, y);
    curve2.draw(x, z);

    // plot and gridspec
    let mut plot = Plot::new();
    plot.set_gridspec("my_grid", 3, 1, "wspace=0, hspace=0.7")
        .set_subplot_grid("my_grid", "0:2", "0")
        .add(&curve1)
        .set_subplot_grid("my_grid", "2", "0")
        .add(&curve2);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_gridspec_1.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let n = lines_iter.count();
    assert!(n > 650);
    assert!(n < 700);
    Ok(())
}

#[test]
fn test_gridspec_rotation_and_align_labels() -> Result<(), StrError> {
    // curves
    let mut curve = Curve::new();
    let x = &[1000.0, 2000.0, 3000.0, 4000.0];
    let y = &[1.0, 2.0, 3.0, 4.0];
    curve.draw(x, y);

    // plot and gridspec
    let mut plot = Plot::new();
    plot.set_gridspec("grid", 2, 2, "hspace=0.35")
        .set_subplot_grid("grid", "0", ":")
        .set_label_x("x-label 0 0")
        .set_label_y("y-label 0 0")
        .set_rotation_ticks_y(55.0)
        .add(&curve)
        .set_subplot_grid("grid", "1", "0")
        .set_label_x("x-label 1 0")
        .set_label_y("y-label 1 0")
        .set_rotation_ticks_x(55.0)
        .add(&curve)
        .set_subplot_grid("grid", "1", "1")
        .set_label_x("x-label 1 1")
        .set_label_y("y-label 1 1")
        .add(&curve)
        .set_align_labels();

    // save figure
    let path = Path::new(OUT_DIR).join("integ_gridspec_2.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let n = lines_iter.count();
    assert!(n > 930);
    assert!(n < 960);
    Ok(())
}
