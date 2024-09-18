use plotpy::{AsMatrix, Boxplot, Plot, StrError};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integ_tests";

#[test]
fn test_boxplot_1() -> Result<(), StrError> {
    let x = vec![vec![1,2,3,4,5],
                 vec![2,3,4,5,6],
                 vec![3,4,5,6,7],
                 vec![4,5,6,7,8],
                 vec![5,6,7,8,9],
                 vec![6,7,8,9,10]];
  
    let ticks: Vec<_> = (1..=x.size().1).into_iter().collect();
    let labels = ["x1", "x2", "x3", "x4", "x5"];
  
    // boxplot object and options
    let mut boxes = Boxplot::new();
    boxes.draw(&x);

    let mut plot = Plot::new();
    plot.add(&boxes);
  
    // save figure
    let path = Path::new(OUT_DIR).join("integ_boxplot_1.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let c = lines_iter.count();
    assert!(c > 750 && c < 800);
  
    Ok(())
}
