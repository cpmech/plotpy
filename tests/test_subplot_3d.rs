use plotpy::{generate3d, Plot, StrError, Surface};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integ_tests";

#[test]
fn test_subplot_3d() -> Result<(), StrError> {
    // data
    let n = 9;
    let (x, y, z) = generate3d(-2.0, 2.0, -2.0, 2.0, n, n, |x, y| x * x + y * y);

    // plot
    let mut plot = Plot::new();

    // surfaces
    let mut surf1 = Surface::new();
    let mut surf2 = Surface::new();
    let mut surf3 = Surface::new();
    let mut surf4 = Surface::new();
    surf1.set_colormap_name("terrain").draw(&x, &y, &z);
    surf2.set_with_surface(false).set_with_wireframe(true).draw(&x, &y, &z);
    surf3.set_surf_color("gold").draw(&x, &y, &z);
    surf4
        .set_with_surface(false)
        .set_with_wireframe(true)
        .set_with_points(true)
        .set_wire_line_color("lime")
        .set_wire_line_width(1.5)
        .set_point_color("black")
        .set_point_void(true)
        .set_point_size(30.0)
        .draw(&x, &y, &z);

    // add surfaces to plot
    plot.set_subplot_3d(2, 2, 1)
        .set_label_x("X AXIS IS BEAUTIFUL")
        .set_label_y("Y AXIS IS BEAUTIFUL")
        .set_label_z("Z AXIS IS BEAUTIFUL")
        .add(&surf1);
    plot.set_subplot_3d(2, 2, 2)
        .set_labels_3d("X AXIS IS BEAUTIFUL", "Y AXIS IS BEAUTIFUL", "Z AXIS IS BEAUTIFUL")
        .add(&surf2);
    plot.set_subplot_3d(2, 2, 3)
        .set_labels_3d("X AXIS IS BEAUTIFUL", "Y AXIS IS BEAUTIFUL", "Z AXIS IS BEAUTIFUL")
        .add(&surf3);
    plot.set_subplot_3d(2, 2, 4)
        .set_labels_3d("X AXIS IS BEAUTIFUL", "Y AXIS IS BEAUTIFUL", "Z AXIS IS BEAUTIFUL")
        .add(&surf4);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_subplot_3d.svg");
    plot.set_figure_size_points(600.0, 600.0)
        .set_save_pad_inches(0.4)
        .save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let n_lines = lines_iter.count();
    assert!(n_lines > 3300 && n < 3400);
    Ok(())
}
