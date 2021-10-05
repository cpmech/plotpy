use plotpy::{Curve, Plot, SlopeIndicator};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integ_tests";

#[test]
fn test_slope_indicator() -> Result<(), &'static str> {
    // curves
    let mut curve1 = Curve::new();
    let mut curve2 = Curve::new();
    curve1.draw(&[-1.0, 1.0], &[-1.0, 1.0]);
    curve2.draw(&[-1.0, 1.0], &[1.0, -1.0]);

    // indicator object and options
    let mut indicator1 = SlopeIndicator::new();
    let mut indicator2 = SlopeIndicator::new();
    let mut indicator3 = SlopeIndicator::new();
    let mut indicator4 = SlopeIndicator::new();
    let mut indicator5 = SlopeIndicator::new();
    let mut indicator6 = SlopeIndicator::new();
    let mut indicator7 = SlopeIndicator::new();
    let mut indicator8 = SlopeIndicator::new();

    // configure indicators
    indicator2.set_flipped(true);
    indicator4.set_flipped(true);
    indicator6.set_flipped(true);
    indicator8.set_flipped(true);

    // draw indicator
    indicator1.draw(1.0, -0.5, -0.5, 0.25);
    indicator2.draw(1.0, -0.5, -0.5, 0.25);
    indicator3.draw(1.0, 0.5, 0.5, 0.25);
    indicator4.draw(1.0, 0.5, 0.5, 0.25);
    indicator5.draw(-1.0, -0.5, 0.5, 0.25);
    indicator6.draw(-1.0, -0.5, 0.5, 0.25);
    indicator7.draw(-1.0, 0.5, -0.5, 0.25);
    indicator8.draw(-1.0, 0.5, -0.5, 0.25);

    // add features to plot
    let mut plot = Plot::new();
    plot.add(&curve1)
        .add(&curve2)
        .add(&indicator1)
        .add(&indicator2)
        .add(&indicator3)
        .add(&indicator4)
        .add(&indicator5)
        .add(&indicator6)
        .add(&indicator7)
        .add(&indicator8);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_slope_indicator.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 630);
    Ok(())
}
