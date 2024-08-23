use plotpy::{Histogram, Plot, StrError};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integ_tests";

#[test]
fn test_histogram_1() -> Result<(), StrError> {
    let mut histogram = Histogram::new();
    histogram
        .set_colors(&vec!["#cd0000", "#1862ab", "#cd8c00"])
        .set_style("barstacked");

    // draw histogram
    let values = vec![
        vec![1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 4, 5, 6], // first series
        vec![-1, -1, 0, 1, 2, 3],                    // second series
        vec![5, 6, 7, 8],                            // third series
    ];
    let labels = ["first".to_string(), "second".to_string(), "third".to_string()];
    histogram.draw(&values, &labels);

    // add surface to plot
    let mut plot = Plot::new();
    plot.add(&histogram);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_histogram_1.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 670);
    Ok(())
}

#[test]
fn test_histogram_2() -> Result<(), StrError> {
    let mut histogram = Histogram::new();
    histogram
        .set_no_fill(true)
        .set_number_bins(16)
        .set_stacked(true)
        .set_extra("orientation='horizontal'");

    // draw histogram
    let values = vec![
        vec![1, 1, 1, 2, 2, 2, 2, 2, 3, 3, 4, 5, 6], // first series
        vec![-1, -1, 0, 1, 2, 3],                    // second series
        vec![5, 6, 7, 8],                            // third series
    ];
    let labels = ["first", "second", "third"];
    histogram.draw(&values, &labels);

    // add histogram to plot
    let mut plot = Plot::new();
    plot.add(&histogram);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_histogram_2.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 810);
    Ok(())
}
