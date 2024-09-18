use plotpy::{AsMatrix, Boxplot, Plot, StrError};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integ_tests";

#[test]
fn test_boxplot_1() -> Result<(), StrError> {
    let data = vec![
        vec![1, 2, 3, 4, 5],
        vec![2, 3, 4, 5, 6],
        vec![3, 4, 5, 6, 7],
        vec![4, 5, 6, 7, 8],
        vec![5, 6, 7, 8, 9],
        vec![6, 7, 8, 9, 10],
        vec![15, 15, 15, 15, 15], // outliers
    ];

    let ticks: Vec<_> = (1..=data.size().1).into_iter().collect();
    let labels = ["first", "second", "third", "fourth", "fifth"];

    // boxplot object and options
    let mut boxes = Boxplot::new();
    boxes.set_symbol("b+").set_horizontal(true).draw(&data);

    let mut plot = Plot::new();
    plot.add(&boxes)
        .set_inv_y()
        .set_title("boxplot integration test")
        .set_ticks_y_labels(&ticks, &labels);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_boxplot_1.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let c = lines_iter.count();
    assert!(c > 1000 && c < 1100);
    Ok(())
}
