use plotpy::{linspace, Curve, FillBetween, Plot, StrError};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integ_tests";

#[test]
fn test_fill_between_1() -> Result<(), StrError> {
    // data
    let x = linspace(-1.0, 2.0, 21);
    let y1: Vec<_> = x.iter().map(|&x| x * x).collect();
    let y2: Vec<_> = x.iter().map(|&x| x).collect();

    // draw
    let mut fb = FillBetween::new();
    fb.draw(&x, &y1, Some(&y2));

    // add fb to plot
    let mut plot = Plot::new();
    plot.add(&fb);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_fill_between_1.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let n = lines_iter.count();
    assert!(n > 420 && n < 500);
    Ok(())
}

#[test]
fn test_fill_between_2() -> Result<(), StrError> {
    // data
    let x = linspace(-1.0, 2.0, 21);
    let y1: Vec<_> = x.iter().map(|&x| x * x).collect();
    let y2: Vec<_> = x.iter().map(|&x| x).collect();

    // draw
    let mut fb = FillBetween::new();
    fb.set_interpolate(true);
    fb.set_facecolor("#ffaabb").set_where("y1>=y2").draw(&x, &y1, Some(&y2));
    fb.set_facecolor("#c1e3ff").set_where("y2>=y1").draw(&x, &y1, Some(&y2));

    // add fb to plot
    let mut plot = Plot::new();
    plot.add(&fb);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_fill_between_2.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let n = lines_iter.count();
    assert!(n > 450 && n < 500);
    Ok(())
}

#[test]
fn test_fill_between_3() -> Result<(), StrError> {
    // data and curve
    let x = linspace(-1.0, 2.0, 21);
    let y: Vec<_> = x.iter().map(|&x| x * x).collect();
    let mut curve = Curve::new();
    curve.set_line_color("black").draw(&x, &y);

    // draw
    let mut fb = FillBetween::new();
    fb.set_where("y1>=0.5").set_extra("alpha=0.5").draw(&x, &y, None);

    // add curve and fb to plot
    let mut plot = Plot::new();
    plot.add(&curve).add(&fb);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_fill_between_3.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let n = lines_iter.count();
    assert!(n > 500 && n < 560);
    Ok(())
}
