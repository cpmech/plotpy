use plotpy::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integration_tests";

#[test]
fn test_histogram_1() -> Result<(), &'static str> {
    let mut histogram = Histogram::new();
    histogram.colors = vec!["#cd0000".to_string(), "#1862ab".to_string(), "#cd8c00".to_string()];
    histogram.style = "barstacked".to_string();

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
    let path = Path::new(OUT_DIR).join("histogram_1.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert_eq!(lines_iter.count(), 694);
    Ok(())
}

#[test]
fn test_histogram_2() -> Result<(), &'static str> {
    let mut histogram = Histogram::new();
    histogram.no_fill = true;
    histogram.number_bins = 16;
    histogram.stacked = true;

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
    let path = Path::new(OUT_DIR).join("histogram_2.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert_eq!(lines_iter.count(), 838);
    Ok(())
}
