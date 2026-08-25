use plotpy::{Plot, StrError, Surface};

fn main() -> Result<(), StrError> {
    let mut surf = Surface::new();
    surf.set_surf_color("#ff880090") // Orange with some transparency
        .set_with_wireframe(true)
        .set_wire_line_color("black")
        .draw_sphere(&[0.0, 0.0, 0.0], 1.0, 30, 30)?;

    let mut plot = Plot::new();
    plot.add(&surf)
        .set_equal_axes(true)  // Forces 1:1:1 aspect ratio so it's a perfect sphere
        .set_zoom_3d(1.5)      // Zooms in optically
        .set_figure_size_points(800.0, 800.0)
        .save("/tmp/plotpy/sphere_zoomed.png")?;

    println!("Saved /tmp/plotpy/sphere_zoomed.png");

    Ok(())
}
