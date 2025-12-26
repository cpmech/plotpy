use plotpy::{generate2d, linspace, Plot, StrError, Stream};
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
    let (mut uu, mut vv) = generate2d(0.0, 1.0, 0.0, 1.0, nx, ny);
    for j in 0..ny {
        for i in 0..nx {
            let x = xx[j][i];
            let y = yy[j][i];
            uu[j][i] = -y;
            vv[j][i] = x;
        }
    }

    // draw arrows
    stream
        .set_color("#dfa629ff")
        .set_streamline_linewidth(0.75)
        .set_streamplot_arrow_style("fancy")
        .set_streamplot_density(0.8)
        .set_streamplot_extra("broken_streamlines=False")
        .draw(&xx, &yy, &uu, &vv);
    quiver
        .set_color("#4752c7ff")
        .set_quiver_inv_scale(15.0)
        .set_quiver_pivot("mid")
        .draw_arrows(&xx, &yy, &uu, &vv);

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

#[test]
fn test_stream_arrows_2() -> Result<(), StrError> {
    // object and options
    let mut stream = Stream::new();
    let mut quiver = Stream::new();

    // data
    let n = 10;
    let x = linspace(1.0, 2.0, n);
    let y = x.clone();
    let u = linspace(1.0, 2.0, n);
    let v = u.clone();
    let (uu, vv) = generate2d(1.0, 2.0, 1.0, 2.0, n, n);

    // draw arrows
    stream.set_streamline_zorder(1).draw_alt(&x, &y, &uu, &vv);
    quiver
        .set_quiver_zorder(2)
        .set_color("red")
        .set_quiver_inv_scale(20.0)
        .draw_arrows_alt(&x, &y, &u, &v);

    // add contour to plot
    let mut plot = Plot::new();
    plot.add(&stream).add(&quiver);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_stream_arrows_2.svg");
    plot.set_equal_axes(true).save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let n = lines_iter.count();
    assert!(n > 3600 && n < 3680);
    Ok(())
}
