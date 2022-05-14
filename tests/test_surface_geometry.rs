use plotpy::{Plot, StrError, Surface};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integ_tests";

#[test]
fn test_surface_geometry() -> Result<(), StrError> {
    // plane
    let mut plane = Surface::new();
    plane.set_colormap_name("terrain").draw_plane_nzz(
        &[-1.0, 1.0, 1.0],
        &[1.0, -1.0, 1.0],
        -1.0,
        1.0,
        -1.0,
        1.0,
        3,
        3,
    )?;

    // cap and cup
    let mut cap = Surface::new();
    let mut cup = Surface::new();
    let (x, y, z, r) = (-1.0, -1.0, -1.0, 0.5);
    cap.set_line_color("red")
        .set_with_surface(false)
        .set_with_wireframe(true)
        .draw_hemisphere(&[x, y, z], r, -180.0, 180.0, 10, 10, false)?;
    cup.draw_hemisphere(&[x, y, z], r, -180.0, 180.0, 10, 10, true)?;

    // add features to plot
    let mut plot = Plot::new();
    plot.add(&plane).add(&cap).add(&cup);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_surface_geometry.svg");
    plot.set_equal_axes(true).save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 1690);
    Ok(())
}

#[test]
fn test_superquadric() -> Result<(), StrError> {
    let mut s1 = Surface::new();
    s1.draw_superquadric(
        &[-1.0, -1.0, -1.0],
        &[1.0, 1.0, 1.0],
        &[0.5, 0.5, 0.5],
        -180.0,
        180.0,
        -90.0,
        90.0,
        40,
        20,
    )?;

    let mut s2 = Surface::new();
    s2.draw_superquadric(
        &[1.0, -1.0, -1.0],
        &[1.0, 1.0, 1.0],
        &[1.0, 1.0, 1.0],
        -180.0,
        180.0,
        -90.0,
        90.0,
        40,
        20,
    )?;

    let mut s3 = Surface::new();
    s3.draw_superquadric(
        &[-1.0, 1.0, 1.0],
        &[1.0, 1.0, 1.0],
        &[4.0, 4.0, 4.0],
        -180.0,
        180.0,
        -90.0,
        90.0,
        40,
        20,
    )?;

    let mut s4 = Surface::new();
    s4.draw_sphere(&[1.0, 1.0, 1.0], 1.0, 40, 20)?;

    // add features to plot
    let mut plot = Plot::new();
    plot.add(&s1).add(&s2).add(&s3).add(&s4);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_superquadric.svg");
    plot.set_equal_axes(true)
        .set_figure_size_points(600.0, 600.0)
        // .save_and_show(&path)?;
        .save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 19950);
    Ok(())
}
