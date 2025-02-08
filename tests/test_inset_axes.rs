use plotpy::{Image, InsetAxes, Plot, StrError};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integ_tests";

#[test]
fn test_inset_axes() -> Result<(), StrError> {
    // object and options
    let mut inset = InsetAxes::new("zoom");

    // draw image
    let mut plot = Plot::new();
    draw_image(&mut plot);

    // add inset axes
    inset.set_xlim(0.0, 1.0).set_ylim(6.0, 5.0).draw(3.0, 4.0, 2.0, 2.0);
    plot.add(&inset);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_inset_axes.svg");
    plot.set_show_errors(true).save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 0);
    Ok(())
}

fn draw_image(plot: &mut Plot) {
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
    img.set_colormap_name("terrain").set_extra("alpha=0.8").draw(&data);
    plot.add(&img);
}
