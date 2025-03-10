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

#[test]
fn test_barplot_3() -> Result<(), StrError> {
    // data
    let fruits = [1.0, 2.0, 3.0];
    let prices = [10.0, 20.0, 30.0];
    let errors = [3.0, 2.0, 1.0];

    // barplot object and options
    let mut bar = Barplot::new();
    bar.set_errors(&errors)
        .set_horizontal(true)
        .set_with_text("edge")
        .draw(&fruits, &prices);

    // plot
    let mut plot = Plot::new();
    plot.set_inv_y().add(&bar);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_barplot_3.svg");
    plot.set_title("Fruits").set_label_x("price").save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let c = lines_iter.count();
    assert!(c > 660 && c < 700);
    Ok(())
}

#[test]
fn test_barplot_4() -> Result<(), StrError> {
    // data
    let fruits = ["Apple", "Banana", "Orange"];
    let prices = [10.0, 20.0, 30.0];
    let errors = [3.0, 2.0, 1.0];

    // barplot object and options
    let mut bar = Barplot::new();
    bar.set_errors(&errors)
        .set_horizontal(true)
        .set_with_text("edge")
        .draw_with_str(&fruits, &prices);

    // plot
    let mut plot = Plot::new();
    plot.set_inv_y().add(&bar);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_barplot_4.svg");
    plot.set_title("Fruits").set_label_x("price").save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let c = lines_iter.count();
    assert!(c > 770 && c < 810);
    Ok(())
}

#[test]
fn test_barplot_5() -> Result<(), StrError> {
    // data
    let y = vec![-2.5, -15.0, -5.0, -7.5, -2.5, -5.0, -17.5];
    let e = vec![0.5, 0.4, 0.1, 0.7, 0.2, 0.0, 1.7];
    let _x: Vec<f64> = (0..y.len()).map(|a| a as f64).collect();
    let _x_str = vec!["Uno", "Dos", "Tres", "Cuatro", "Cinco", "Seis", "Siete"];

    // barplot
    let mut bar = Barplot::new();
    bar.set_errors(&e)
        // .draw(&_x, &y); // requires numbers, as expected
        .draw_with_str(&_x_str, &y); // requires string, as expected

    // plot
    let mut plot = Plot::new();
    plot.add(&bar);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_barplot_5.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let c = lines_iter.count();
    assert!(c > 830 && c < 900);
    Ok(())
}
