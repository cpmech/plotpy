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
fn test_surface() -> Result<(), &'static str> {
    let mut surface = Surface::new();
    surface.wireframe = true;
    surface.colorbar = true;
    surface.colorbar_label = "temperature".to_string();
    surface.line_color = "yellow".to_string();
    surface.line_style = ":".to_string();
    surface.line_width = 0.75;

    // draw surface
    let n = 9;
    let (x, y, z) = gen_xyz(n);
    surface.draw(&x, &y, &z);

    // add surface to plot
    let mut plot = Plot::new();
    plot.add(&surface);

    // save figure
    let path = Path::new(OUT_DIR).join("surface.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert_eq!(lines_iter.count(), 1600);
    Ok(())
}
