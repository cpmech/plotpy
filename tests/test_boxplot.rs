use plotpy::{Boxplot, Plot, StrError};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

const OUT_DIR: &str = "/tmp/plotpy/integ_tests";

#[test]
fn test_boxplot_1() -> Result<(), StrError> {
    let data = vec![
        //   A  B  C  D  E
        vec![1, 2, 3, 4, 5],
        vec![2, 3, 4, 5, 6],
        vec![3, 4, 5, 6, 7],
        vec![4, 5, 6, 7, 8],
        vec![5, 6, 7, 8, 9],
        vec![6, 7, 8, 9, 10],
        vec![15, 15, 15, 15, 15], // fliers
    ];

    let positions = [2.0, 2.5, 3.0, 3.5, 4.0];
    let labels = ["A", "B", "C", "D", "E"];

    // boxplot object and options
    let mut boxes = Boxplot::new();
    boxes
        .set_symbol("b.")
        .set_horizontal(true)
        .set_positions(&positions)
        .set_width(0.45)
        .set_whisker(2.0)
        .set_no_fliers(false)
        .set_extra("notch=True")
        .draw_mat(&data);

    let mut plot = Plot::new();
    plot.add(&boxes)
        .set_inv_y()
        .set_title("boxplot integration test")
        .set_ticks_y_labels(&positions, &labels);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_boxplot_1.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let c = lines_iter.count();
    assert!(c > 1000 && c < 1100);
    Ok(())
}

#[test]
fn test_boxplot_2() -> Result<(), StrError> {
    let data = vec![
        vec![1, 2, 3, 4, 5],              // A
        vec![2, 3, 4, 5, 6, 7, 8, 9, 10], // B
        vec![3, 4, 5, 6],                 // C
        vec![4, 5, 6, 7, 8, 9, 10],       // D
        vec![5, 6, 7],                    // E
    ];

    let positions = [2.0, 2.5, 3.0, 3.5, 4.0];
    let labels = ["A", "B", "C", "D", "E"];

    // boxplot object and options
    let mut boxes = Boxplot::new();
    boxes
        .set_symbol("b.")
        .set_horizontal(true)
        .set_positions(&positions)
        .set_width(0.45)
        .draw(&data);

    let mut plot = Plot::new();
    plot.add(&boxes)
        .set_inv_y()
        .set_title("boxplot integration test")
        .set_ticks_y_labels(&positions, &labels);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_boxplot_2.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let c = lines_iter.count();
    assert!(c > 950 && c < 1000);
    Ok(())
}

#[test]
fn test_boxplot_3() -> Result<(), StrError> {
    let data = vec![
        //    A    B    C    D    E ← matrix: columns are series
        //                            nested: rows are series
        //                                ↓
        vec![1.0, 2.0, 3.0, 4.0, 5.0], // A
        vec![1.1, 2.1, 3.1, 4.1, 5.1], // B
        vec![1.2, 2.2, 3.2, 4.2, 5.2], // C
        vec![1.3, 2.3, 3.3, 4.3, 5.3], // D
        vec![1.4, 2.4, 3.4, 4.4, 5.4], // E
    ];

    let ticks = [1, 2, 3, 4, 5];
    let labels = ["A", "B", "C", "D", "E"];

    let mut boxes_nes = Boxplot::new();
    boxes_nes.draw(&data);

    let mut boxes_mat = Boxplot::new();
    boxes_mat.draw_mat(&data);

    let mut plot = Plot::new();
    plot.set_subplot(1, 2, 1)
        .set_title("nested")
        .add(&boxes_nes)
        .set_ticks_x_labels(&ticks, &labels)
        .set_subplot(1, 2, 2)
        .set_title("matrix")
        .add(&boxes_mat)
        .set_ticks_x_labels(&ticks, &labels);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_boxplot_3.svg");
    plot.set_figure_size_points(650.0, 300.0).save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let c = lines_iter.count();
    assert!(c > 1180 && c < 1260);
    Ok(())
}

#[test]
fn test_boxplot_4() -> Result<(), StrError> {
    let data = vec![
        vec![1, 2, 3, 4, 5, 10],          // A
        vec![2, 3, 4, 5, 6, 7, 8, 9, 10], // B
        vec![3, 4, 5, 6, 10],             // C
    ];

    let ticks = [1, 2, 3];
    let labels = ["A", "B", "C"];

    let mut b1 = Boxplot::new();
    b1.draw(&data);

    let mut b2 = Boxplot::new();
    b2.set_symbol("b+").draw(&data);

    let mut b3 = Boxplot::new();
    b3.set_symbol("b+").set_no_fliers(true).draw(&data);

    let mut plot = Plot::new();
    plot.set_vertical_gap(0.0)
        .set_subplot(3, 1, 1)
        .add(&b1)
        .set_subplot(3, 1, 2)
        .add(&b2)
        .set_subplot(3, 1, 3)
        .add(&b3)
        .set_ticks_x_labels(&ticks, &labels)
        .set_figure_size_points(300.0, 450.0);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_boxplot_4.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let c = lines_iter.count();
    assert!(c > 1020 && c < 1100);
    Ok(())
}

#[test]
fn test_boxplot_5() -> Result<(), StrError> {
    let x = vec![
        vec![1, 2, 3],       // A
        vec![2, 3, 4, 5, 6], // B
        vec![6, 7],          // C
    ];
    let mut boxes = Boxplot::new();
    boxes
        .set_symbol("b+")
        .set_no_fliers(true)
        .set_horizontal(true)
        .set_whisker(1.5)
        .set_positions(&[1.0, 2.0, 3.0])
        .set_width(0.5)
        .set_patch_artist(true)
        .set_boxprops("{'facecolor': 'C0', 'edgecolor': 'white','linewidth': 0.5}")
        .draw(&x);
    let mut plot = Plot::new();
    plot.add(&boxes);

    // save figure
    let path = Path::new(OUT_DIR).join("integ_boxplot_5.svg");
    plot.save(&path)?;

    // check number of lines
    let file = File::open(path).map_err(|_| "cannot open file")?;
    let buffered = BufReader::new(file);
    let lines_iter = buffered.lines();
    let c = lines_iter.count();
    assert!(c > 1020 && c < 1100);
    Ok(())
}
