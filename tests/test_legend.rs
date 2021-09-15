use plotpy::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integration_tests";

#[test]
#[test]
fn test_legend_1() -> Result<(), &'static str> {
    // curve and options
    let mut curve1 = Curve::new();
    curve1.label = "my-curve".to_string();

    // another curve
    let mut curve2 = Curve::new();
    curve2.label = "another-curve".to_string();

    // draw curve
    let x = &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let y = &[
        1.0, 1.41421356, 1.73205081, 2.0, 2.23606798, 2.44948974, 2.64575131, 2.82842712, 3.0, 3.16227766,
    ];
    let y2 = &[1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 3.5, 3.5, 3.5, 3.5];
    curve1.draw(x, y);
    curve2.draw(x, y2);

    // legend and options
    let mut legend = Legend::new();
    legend.show_frame = false;
    legend.outside = true;
    legend.fontsize = 18.0;
    legend.num_col = 2;

    // draw legend
    legend.draw();

    // add curve and legend to plot
    let mut plot = Plot::new();
    plot.add(&curve1);
    plot.add(&curve2);
    plot.add(&legend); // must be after a (labelled) curve

    // save figure
    let path = Path::new(OUT_DIR).join("legend_1.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 780);
    Ok(())
}

#[test]
fn test_legend_2() -> Result<(), &'static str> {
    // curve and options
    let mut curve1 = Curve::new();
    curve1.label = "my-curve".to_string();

    // another curve
    let mut curve2 = Curve::new();
    curve2.label = "another-curve".to_string();

    // draw curve
    let x = &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let y = &[
        1.0, 1.41421356, 1.73205081, 2.0, 2.23606798, 2.44948974, 2.64575131, 2.82842712, 3.0, 3.16227766,
    ];
    let y2 = &[1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 3.5, 3.5, 3.5, 3.5];
    curve1.draw(x, y);
    curve2.draw(x, y2);

    // legend and options
    let mut legend = Legend::new();

    // draw legend
    legend.draw();

    // add curve and legend to plot
    let mut plot = Plot::new();
    plot.add(&curve1);
    plot.add(&curve2);
    plot.add(&legend); // must be after a (labelled) curve

    // save figure
    let path = Path::new(OUT_DIR).join("legend_2.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 780);
    Ok(())
}
