use plotpy::{Barplot, Plot, StrError};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integ_tests";

#[test]
fn test_barplot_1() -> Result<(), StrError> {
    // data
    let xx = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let yy = [5, 4, 3, 2, 1, 0, 1, 2, 3, 4];

    // barplot object and options
    let mut barplot = Barplot::new();
    barplot.draw(&xx, &yy);

    let mut plot = Plot::new();
    plot.add(&barplot);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_barplot_1.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let c = lines_iter.count();
    assert!(c > 500 && c < 550);
    Ok(())
}

#[test]
fn test_barplot_2() -> Result<(), StrError> {
    // data
    let species = ["Adelie", "Chinstrap", "Gentoo"];
    let sex_counts = HashMap::from([
        ("Male", ([73.0, 34.0, 61.0], ["red", "green", "blue"])),
        ("Female", ([73.0, 34.0, 58.0], ["#DE3163", "#40E0D0", "#6495ED"])),
    ]);

    // barplot object and options
    let mut bar = Barplot::new();
    bar.set_with_text("center")
        .set_width(0.6)
        .set_extra("edgecolor='black'");

    // draw bars
    let mut bottom = [0.0, 0.0, 0.0];
    for (sex, (sex_count, colors)) in &sex_counts {
        bar.set_label(sex)
            .set_colors(colors)
            .set_bottom(&bottom)
            .draw_with_str(&species, sex_count);
        for i in 0..sex_count.len() {
            bottom[i] += sex_count[i];
        }
    }

    // plot
    let mut plot = Plot::new();
    plot.add(&bar);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_barplot_2.svg");
    plot.set_title("Number of penguins by sex").legend().save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let c = lines_iter.count();
    assert!(c > 1180 && c < 1200);
    Ok(())
}
