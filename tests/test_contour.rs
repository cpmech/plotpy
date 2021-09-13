use plotpy::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integration_tests";

fn gen_xyz() -> (
    &'static [&'static [f64]],
    &'static [&'static [f64]],
    &'static [&'static [f64]],
) {
    let x: &[&[f64]] = &[
        &[-2.0, -1.5, -1.0, -0.5, 0.0, 0.5, 1.0, 1.5],
        &[-2.0, -1.5, -1.0, -0.5, 0.0, 0.5, 1.0, 1.5],
        &[-2.0, -1.5, -1.0, -0.5, 0.0, 0.5, 1.0, 1.5],
        &[-2.0, -1.5, -1.0, -0.5, 0.0, 0.5, 1.0, 1.5],
        &[-2.0, -1.5, -1.0, -0.5, 0.0, 0.5, 1.0, 1.5],
        &[-2.0, -1.5, -1.0, -0.5, 0.0, 0.5, 1.0, 1.5],
        &[-2.0, -1.5, -1.0, -0.5, 0.0, 0.5, 1.0, 1.5],
        &[-2.0, -1.5, -1.0, -0.5, 0.0, 0.5, 1.0, 1.5],
    ];
    let y: &[&[f64]] = &[
        &[-2.0, -2.0, -2.0, -2.0, -2.0, -2.0, -2.0, -2.0],
        &[-1.5, -1.5, -1.5, -1.5, -1.5, -1.5, -1.5, -1.5],
        &[-1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0],
        &[-0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5],
        &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        &[0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5],
        &[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
        &[1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5, 1.5],
    ];
    let z: &[&[f64]] = &[
        &[8.00, 6.25, 5.00, 4.25, 4.00, 4.25, 5.00, 6.25],
        &[6.25, 4.50, 3.25, 2.50, 2.25, 2.50, 3.25, 4.50],
        &[5.00, 3.25, 2.00, 1.25, 1.00, 1.25, 2.00, 3.25],
        &[4.25, 2.50, 1.25, 0.50, 0.25, 0.50, 1.25, 2.50],
        &[4.00, 2.25, 1.00, 0.25, 0.00, 0.25, 1.00, 2.25],
        &[4.25, 2.50, 1.25, 0.50, 0.25, 0.50, 1.25, 2.50],
        &[5.00, 3.25, 2.00, 1.25, 1.00, 1.25, 2.00, 3.25],
        &[6.25, 4.50, 3.25, 2.50, 2.25, 2.50, 3.25, 4.50],
    ];
    (x, y, z)
}

#[test]
fn test_contour() -> Result<(), &'static str> {
    // contour object and options
    let mut contour = Contour::new();
    contour.colorbar_label = "temperature".to_string();
    contour.with_selected = true;
    contour.selected_level = 1.0;

    // draw contour
    let (x, y, z) = gen_xyz();
    contour.draw(x, y, z)?;

    // add contour to plot
    let mut plot = Plot::new();
    plot.add(&contour);

    // save figure
    let path = Path::new(OUT_DIR).join("contour.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert_eq!(lines_iter.count(), 1484);
    Ok(())
}

#[test]
fn test_contour_colors() -> Result<(), &'static str> {
    // contour object and options
    let mut contour = Contour::new();
    contour.colors = vec!["#f00".to_string(), "#0f0".to_string(), "#00f".to_string()];
    contour.levels = vec![0.25, 0.5, 1.0];
    contour.no_lines = true;
    contour.no_labels = true;
    contour.no_inline_labels = true;
    contour.no_colorbar = true;

    // draw contour
    let (x, y, z) = gen_xyz();
    contour.draw(x, y, z)?;

    // add contour to plot
    let mut plot = Plot::new();
    plot.add(&contour);

    // save figure
    let path = Path::new(OUT_DIR).join("contour_with_options.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert_eq!(lines_iter.count(), 474);
    Ok(())
}

#[test]
fn test_contour_colormap() -> Result<(), &'static str> {
    for index in 0..10 {
        // contour object and options
        let mut contour = Contour::new();
        contour.levels = vec![0.25, 0.5, 1.0];
        contour.colormap_index = index;
        contour.no_lines = true;
        contour.no_labels = true;
        contour.no_inline_labels = true;
        contour.no_colorbar = true;

        // draw contour
        let (x, y, z) = gen_xyz();
        contour.draw(x, y, z)?;

        // add contour to plot
        let mut plot = Plot::new();
        plot.add(&contour);

        // save figure
        let filename = format!("contour_colormap_{}.svg", index);
        let path = Path::new(OUT_DIR).join(&filename);
        plot.save(&path)?;

        // check number of lines
        let file = File::open(path).map_err(|_| "cannot open file")?;
        let buffered = BufReader::new(file);
        let lines_iter = buffered.lines();
        assert_eq!(lines_iter.count(), 474);
    }
    Ok(())
}
