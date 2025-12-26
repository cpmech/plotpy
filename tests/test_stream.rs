use plotpy::{generate2d, Plot, StrError, Stream};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integ_tests";

#[test]
fn test_stream_arrows_1() -> Result<(), StrError> {
    // object and options
    let mut stream = Stream::new();
    let mut quiver = Stream::new();

    // data
    let (nx, ny) = (10, 10);
    let (xx, yy) = generate2d(-2.0, 2.0, -2.0, 2.0, nx, ny);
    let (mut dx, mut dy) = generate2d(0.0, 1.0, 0.0, 1.0, nx, ny);
    for j in 0..ny {
        for i in 0..nx {
            let x = xx[j][i];
            let y = yy[j][i];
            dx[j][i] = -y;
            dy[j][i] = x;
        }
    }

    // draw arrows
    stream
        .set_color("#dfa629ff")
        .set_streamline_linewidth(0.75)
        .set_streamplot_arrow_style("fancy")
        .set_streamplot_density(0.8)
        .set_streamplot_extra("broken_streamlines=False")
        .draw(&xx, &yy, &dx, &dy);
    quiver
        .set_color("#4752c7ff")
        .set_quiver_inv_scale(15.0)
        .set_quiver_pivot("mid")
        .draw_arrows(&xx, &yy, &dx, &dy);

    // add contour to plot
    let mut plot = Plot::new();
    plot.add(&stream).add(&quiver);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_stream_arrows_1.svg");
    plot.set_equal_axes(true).save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let n = lines_iter.count();
    assert!(n > 8650 && n < 8730);
    Ok(())
}
