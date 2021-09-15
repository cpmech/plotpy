use plotpy::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integration_tests";

fn gen_xyz(n: usize) -> (Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<Vec<f64>>) {
    assert!(n > 1);
    let mut x = vec![vec![0.0; n]; n];
    let mut y = vec![vec![0.0; n]; n];
    let mut z = vec![vec![0.0; n]; n];
    let (min, max) = (-2.0, 2.0);
    let d = (max - min) / ((n - 1) as f64);
    for i in 0..n {
        let v = min + (i as f64) * d;
        for j in 0..n {
            let u = min + (j as f64) * d;
            x[i][j] = u;
            y[i][j] = v;
            z[i][j] = u * u + v * v;
        }
    }
    (x, y, z)
}

#[test]
fn test_contour() -> Result<(), &'static str> {
    // contour object and options
    let mut contour = Contour::new();
    contour.colors = vec![
        "#fcaeae".to_string(),
        "#da98d1".to_string(),
        "#c45178".to_string(),
        "#5594d2".to_string(),
        "#e6af69".to_string(),
        "#e6d969".to_string(),
    ];
    contour.levels = vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    contour.colorbar_label = "temperature".to_string();
    contour.line_color = "yellow".to_string();
    contour.line_style = ":".to_string();
    contour.line_width = 2.5;
    contour.with_selected = true;
    contour.selected_line_color = "#69e699".to_string();
    contour.selected_line_width = 5.0;
    contour.selected_level = 1.0;

    // draw contour
    let n = 9;
    let (x, y, z) = gen_xyz(n);
    contour.draw(&x, &y, &z);

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
    assert!(lines_iter.count() > 1500);
    Ok(())
}

#[test]
fn test_contour_colors() -> Result<(), &'static str> {
    // contour object and options
    let mut contour = Contour::new();
    contour.colors = vec!["#f00".to_string(), "#0f0".to_string(), "#00f".to_string()];
    contour.no_lines = true;
    contour.no_labels = true;
    contour.no_inline_labels = true;
    contour.no_colorbar = true;

    // draw contour
    let n = 9;
    let (x, y, z) = gen_xyz(n);
    contour.draw(&x, &y, &z);

    // add contour to plot
    let mut plot = Plot::new();
    plot.add(&contour);

    // save figure
    let path = Path::new(OUT_DIR).join("contour_colors.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 580);
    Ok(())
}

#[test]
fn test_contour_colormap_index() -> Result<(), &'static str> {
    for index in 0..10 {
        // contour object and options
        let mut contour = Contour::new();
        contour.colormap_index = index;
        contour.no_lines = true;
        contour.no_labels = true;
        contour.no_inline_labels = true;
        contour.no_colorbar = true;

        // draw contour
        let n = 9;
        let (x, y, z) = gen_xyz(n);
        contour.draw(&x, &y, &z);

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
        assert!(lines_iter.count() > 770);
    }
    Ok(())
}

#[test]
fn test_contour_colormap_name() -> Result<(), &'static str> {
    for name in ["Pastel1", "tab20c", "gnuplot2"] {
        // contour object and options
        let mut contour = Contour::new();
        contour.colormap_name = name.to_string();
        contour.no_lines = true;
        contour.no_labels = true;
        contour.no_inline_labels = true;
        contour.no_colorbar = true;

        // draw contour
        let n = 9;
        let (x, y, z) = gen_xyz(n);
        contour.draw(&x, &y, &z);

        // add contour to plot
        let mut plot = Plot::new();
        plot.add(&contour);

        // save figure
        let filename = format!("contour_colormap_{}.svg", name);
        let path = Path::new(OUT_DIR).join(&filename);
        plot.save(&path)?;

        // check number of lines
        let file = File::open(path).map_err(|_| "cannot open file")?;
        let buffered = BufReader::new(file);
        let lines_iter = buffered.lines();
        assert!(lines_iter.count() > 770);
    }
    Ok(())
}
