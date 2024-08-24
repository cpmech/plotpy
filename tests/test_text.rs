use plotpy::{Plot, StrError, Text};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integ_tests";

#[test]
fn test_text() -> Result<(), StrError> {
    // text object and options
    let mut text = Text::new();
    text.set_color("blue")
        .set_align_horizontal("center")
        .set_align_vertical("center")
        .set_fontsize(40.0)
        .set_rotation(45.0)
        .set_bbox(true)
        .set_bbox_facecolor("pink")
        .set_bbox_edgecolor("black")
        .set_bbox_alpha(0.3)
        // .set_bbox_style("square,pad=0.3");
        // .set_bbox_style("circle,pad=0.3");
        // .set_bbox_style("larrow,pad=0.3");
        // .set_bbox_style("rarrow,pad=0.3");
        // .set_bbox_style("darrow,pad=0.3");
        // .set_bbox_style("round,pad=0.3,rounding_size=0.15");
        // .set_bbox_style("round4,pad=0.3,rounding_size=0.2");
        .set_bbox_style("sawtooth,pad=0.3,tooth_size=0.1")
        .set_extra("fontweight=10");
    // .set_bbox_style("roundtooth,pad=0.3,tooth_size=0.2");

    // draw text
    text.draw(0.5, 0.5, "message: $\\frac{\\alpha}{\\beta} = \\gamma$");

    // add text to plot
    let mut plot = Plot::new();
    plot.add(&text);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_text.svg");
    plot.set_show_errors(true).save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 560);
    Ok(())
}

#[test]
fn test_text_3d() -> Result<(), StrError> {
    // text object and options
    let mut text = Text::new();
    text.set_color("blue")
        .set_align_horizontal("center")
        .set_align_vertical("center")
        .set_fontsize(32.0)
        .set_rotation(45.0)
        .set_bbox(true)
        .set_bbox_facecolor("pink")
        .set_bbox_edgecolor("black")
        .set_bbox_alpha(0.3);

    // draw text
    text.draw_3d(0.5, 0.5, 0.5, "message: $\\frac{\\alpha}{\\beta} = \\gamma$");

    // add text to plot
    let mut plot = Plot::new();
    plot.add(&text);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_text_3d.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    assert!(lines_iter.count() > 800);
    Ok(())
}
