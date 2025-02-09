use plotpy::{Barplot, Canvas, Image, InsetAxes, Plot, StrError};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integ_tests";

#[test]
fn test_inset_axes_1() -> Result<(), StrError> {
    // draw image
    let data = [
        [0.8, 2.4, 2.5, 3.9, 0.0, 4.0, 0.0],
        [2.4, 0.0, 4.0, 1.0, 2.7, 0.0, 0.0],
        [1.1, 2.4, 0.8, 4.3, 1.9, 4.4, 0.0],
        [0.6, 0.0, 0.3, 0.0, 3.1, 0.0, 0.0],
        [0.7, 1.7, 0.6, 2.6, 2.2, 6.2, 0.0],
        [1.3, 1.2, 0.0, 0.0, 0.0, 3.2, 5.1],
        [0.1, 2.0, 0.0, 1.4, 0.0, 1.9, 6.3],
    ];
    let mut img = Image::new();
    let mut plot = Plot::new();
    img.set_colormap_name("terrain").set_extra("alpha=0.8").draw(&data);
    plot.add(&img);

    // inset axes
    let mut inset = InsetAxes::new();
    inset
        .set_title("ZOOM")
        .set_visibility(true)
        .set_indicator_line_color("red")
        .set_indicator_line_style("--")
        .set_indicator_line_width(2.0)
        .set_indicator_alpha(1.0)
        .set_indicator_hatch("x")
        .set_extra_for_axes("xlabel='X',ylabel='Y'")
        .set_extra_for_indicator("label='INDICATOR',visible=True")
        .set_range(0.0, 1.0, 5.0, 6.0);
    inset.add(&img).draw(0.5, 0.5, 0.4, 0.3);

    // add entities to plot
    plot.add(&img).add(&inset);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_inset_axes_1.svg");
    plot.set_show_errors(true).save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let n = lines_iter.count().clone();
    assert!(n > 680 && n < 800);
    Ok(())
}

#[test]
fn test_inset_axes_2() -> Result<(), StrError> {
    // data
    let x = [0, 1, 2, 3, 4];
    let y = [5, 4, 3, 2, 1];

    // define a function to draw with vertical and horizontal bars
    let draw = |plot: &mut Plot, horizontal: bool| {
        // allocate the Barplot and InsetAxes instances
        let mut bar = Barplot::new();
        let mut inset = InsetAxes::new();

        // configure the barplot
        bar.set_horizontal(horizontal).draw(&x, &y);

        // configure the inset axes
        inset.set_range(0.5, 2.5, 2.0, 4.5);

        // add barplot to inset
        inset.add(&bar).draw(0.65, 0.65, 0.335, 0.33);

        // add barplot and inset to plot
        plot.add(&bar).add(&inset);
    };

    // allocate plot and add each type of figure to a subplot
    let mut plot = Plot::new();

    // vertical bars
    plot.set_subplot(1, 2, 1);
    draw(&mut plot, false);

    // horizontal bars
    plot.set_subplot(1, 2, 2);
    draw(&mut plot, true);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_inset_axes_2.svg");
    plot.set_figure_size_points(650.0, 250.0)
        .set_show_errors(true)
        .save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let n = lines_iter.count().clone();
    assert!(n > 790 && n < 850);
    Ok(())
}

#[test]
fn test_inset_axes_3() -> Result<(), StrError> {
    // canvas
    let mut canvas = Canvas::new();
    canvas
        .set_face_color("None")
        .set_edge_color("red")
        .draw_circle(0.5, 0.5, 0.45);

    // inset axes
    let mut inset = InsetAxes::new();
    inset
        .set_indicator_alpha(1.0)
        .set_indicator_line_color("blue")
        .set_range(0.5, 1.0, 0.5, 1.0)
        .add(&canvas)
        .draw(0.65, 0.65, 0.335, 0.33);

    // add to plot
    let mut plot = Plot::new();
    plot.add(&canvas).add(&inset);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_inset_axes_3.svg");
    plot.set_range(0.0, 2.0, 0.0, 2.0)
        .set_equal_axes(true)
        .set_show_errors(true)
        .save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let n = lines_iter.count().clone();
    assert!(n > 680 && n < 800);
    Ok(())
}
