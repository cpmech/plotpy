use plotpy::{Curve, DarkMode, Plot, StrError};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integ_tests";

#[test]
fn test_dark_mode_default() -> Result<(), StrError> {
    // curve
    let x = [1.0, 2.0, 3.0, 4.0];
    let y = [1.0, 4.0, 9.0, 16.0];
    let mut curve = Curve::new();
    curve.set_label("curve").draw(&x, &y);

    // dark mode enabler
    let dm = DarkMode::new();

    // plot
    let mut plot = Plot::new();
    plot.add(&dm).add(&curve);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_dark_mode_default.svg");
    plot.legend()
        .grid_and_labels("x", "y")
        .set_show_errors(true)
        .save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let n = lines_iter.count();
    assert!(n > 700 && n < 830);
    Ok(())
}

#[test]
fn test_dark_mode_mathematica() -> Result<(), StrError> {
    // curve
    let x = [1.0, 2.0, 3.0, 4.0];
    let y = [1.0, 4.0, 9.0, 16.0];
    let mut curve = Curve::new();
    curve.set_label("curve").draw(&x, &y);

    // dark mode enabler
    let mut dm = DarkMode::new();
    dm.set_mathematica();

    // plot
    let mut plot = Plot::new();
    plot.add(&dm).add(&curve);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_dark_mode_mathematica.svg");
    plot.legend()
        .grid_and_labels("x", "y")
        .set_show_errors(true)
        .save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let n = lines_iter.count();
    assert!(n > 700 && n < 830);
    Ok(())
}

#[test]
fn test_dark_mode_mocha() -> Result<(), StrError> {
    // curve
    let x = [1.0, 2.0, 3.0, 4.0];
    let y = [1.0, 4.0, 9.0, 16.0];
    let mut curve = Curve::new();
    curve.set_label("curve").draw(&x, &y);

    // dark mode enabler
    let mut dm = DarkMode::new();
    dm.set_mocha();

    // plot
    let mut plot = Plot::new();
    plot.add(&dm).add(&curve);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_dark_mode_mocha.svg");
    plot.legend()
        .grid_and_labels("x", "y")
        .set_show_errors(true)
        .save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let n = lines_iter.count();
    assert!(n > 700 && n < 830);
    Ok(())
}

#[test]
fn test_dark_mode_nordic() -> Result<(), StrError> {
    // curve
    let x = [1.0, 2.0, 3.0, 4.0];
    let y = [1.0, 4.0, 9.0, 16.0];
    let mut curve = Curve::new();
    curve.set_label("curve").draw(&x, &y);

    // dark mode enabler
    let mut dm = DarkMode::new();
    dm.set_nordic();

    // plot
    let mut plot = Plot::new();
    plot.add(&dm).add(&curve);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_dark_mode_nordic.svg");
    plot.legend()
        .grid_and_labels("x", "y")
        .set_show_errors(true)
        .save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let n = lines_iter.count();
    assert!(n > 700 && n < 830);
    Ok(())
}
