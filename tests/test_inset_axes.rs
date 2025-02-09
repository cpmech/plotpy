use plotpy::{Image, InsetAxes, Plot, StrError};
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
        .set_indicator_line_color("red")
        .set_indicator_line_style("--")
        .set_extra_for_axes("xticklabels=[],yticklabels=[]")
        .set_extra_for_indicator("label='INDICATOR',linewidth=2")
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
    assert!(n > 510 && n < 600);
    Ok(())
}
