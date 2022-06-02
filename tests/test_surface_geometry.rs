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
fn test_surface_cylinder() -> Result<(), StrError> {
    let mut surface = Surface::new();
    surface.set_with_colormap(false);
    surface.draw_cylinder(&[0.0, 0.0, 0.0], &[5.0, 0.0, 0.0], 0.5, 1, 20)?;
    surface.draw_cylinder(&[0.0, 0.0, 0.0], &[0.0, 5.0, 0.0], 0.5, 1, 20)?;
    surface.draw_cylinder(&[0.0, 0.0, 0.0], &[0.0, 0.0, 5.0], 0.5, 1, 20)?;
    surface.draw_cylinder(&[0.0, 0.0, 0.0], &[5.0, 5.0, 5.0], 0.5, 1, 20)?;
    surface.draw_cylinder(&[5.0, 5.0, 0.0], &[5.0, 5.0, 5.0], 0.5, 1, 20)?;

    // add surface to plot
    let mut plot = Plot::new();
    plot.add(&surface);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_surface_cylinder.svg");
    plot.set_range_3d(-1.0, 6.0, -1.0, 6.0, -1.0, 6.0).set_equal_axes(true);
    plot.save(&path)?;
    // plot.save_and_show(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 1340);
    Ok(())
}

#[test]
fn test_surface_superquadric() -> Result<(), StrError> {
    // star
    let r = &[1.0, 1.0, 1.0];
    let c = &[-1.0, -1.0, -1.0];
    let k = &[0.5, 0.5, 0.5];
    let mut star = Surface::new();
    star.set_colormap_name("jet")
        .draw_superquadric(c, r, k, -180.0, 180.0, -90.0, 90.0, 40, 20)?;

    // pyramids
    let c = &[1.0, -1.0, -1.0];
    let k = &[1.0, 1.0, 1.0];
    let mut pyramids = Surface::new();
    pyramids
        .set_colormap_name("inferno")
        .draw_superquadric(c, r, k, -180.0, 180.0, -90.0, 90.0, 40, 20)?;

    // rounded cube
    let c = &[-1.0, 1.0, 1.0];
    let k = &[4.0, 4.0, 4.0];
    let mut cube = Surface::new();
    cube.set_solid_color("#ee29f2")
        .draw_superquadric(c, r, k, -180.0, 180.0, -90.0, 90.0, 40, 20)?;

    // sphere
    let c = &[0.0, 0.0, 0.0];
    let k = &[2.0, 2.0, 2.0];
    let mut sphere = Surface::new();
    sphere
        .set_colormap_name("rainbow")
        .draw_superquadric(c, r, k, -180.0, 180.0, -90.0, 90.0, 40, 20)?;

    // sphere (direct)
    let mut sphere_direct = Surface::new();
    sphere_direct.draw_sphere(&[1.0, 1.0, 1.0], 1.0, 40, 20)?;

    // add features to plot
    let mut plot = Plot::new();
    plot.add(&star)
        .add(&pyramids)
        .add(&cube)
        .add(&sphere)
        .add(&sphere_direct);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_surface_superquadric.svg");
    plot.set_equal_axes(true)
        .set_figure_size_points(600.0, 600.0)
        .save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 24780);
    Ok(())
}
