use plotpy::{linspace, Canvas, Curve, Plot, StrError, Surface};
use std::f64::consts::PI;

fn cross(a: &[f64; 3], b: &[f64; 3]) -> [f64; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn dot(a: &[f64; 3], b: &[f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

fn norm(a: &[f64; 3]) -> f64 {
    dot(a, a).sqrt()
}

fn rodrigues_rotation(axis: &[f64; 3], angle: f64) -> [[f64; 3]; 3] {
    let [kx, ky, kz] = *axis;
    let k = [[0.0, -kz, ky], [kz, 0.0, -kx], [-ky, kx, 0.0]];
    let mut k2 = [[0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            for m in 0..3 {
                k2[i][j] += k[i][m] * k[m][j];
            }
        }
    }

    let cos_a = angle.cos();
    let sin_a = angle.sin();

    let mut r = [[0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            r[i][j] = (if i == j { 1.0 } else { 0.0 }) + sin_a * k[i][j] + (1.0 - cos_a) * k2[i][j];
        }
    }
    r
}

fn plot_von_mises(elev: f64, azim: f64) -> Result<(), StrError> {
    let radius = 3.0;
    let height = 15.0;

    let n_z = 50;
    let n_theta = 50;

    let z_cyl = linspace(0.0, height, n_z);
    let theta = linspace(0.0, 2.0 * PI, n_theta);

    let v = [
        1.0 / 3.0f64.sqrt(),
        1.0 / 3.0f64.sqrt(),
        1.0 / 3.0f64.sqrt(),
    ];
    let z_axis = [0.0, 0.0, 1.0];

    let mut axis = cross(&z_axis, &v);
    let n_axis = norm(&axis);
    for i in 0..3 {
        axis[i] /= n_axis;
    }
    let angle = dot(&z_axis, &v).acos();

    let r = rodrigues_rotation(&axis, angle);

    let mut x_mat = vec![vec![0.0; n_theta]; n_z];
    let mut y_mat = vec![vec![0.0; n_theta]; n_z];
    let mut z_mat = vec![vec![0.0; n_theta]; n_z];

    for i in 0..n_z {
        for j in 0..n_theta {
            let cx = radius * theta[j].cos();
            let cy = radius * theta[j].sin();
            let cz = z_cyl[i];

            x_mat[i][j] = r[0][0] * cx + r[0][1] * cy + r[0][2] * cz;
            y_mat[i][j] = r[1][0] * cx + r[1][1] * cy + r[1][2] * cz;
            z_mat[i][j] = r[2][0] * cx + r[2][1] * cy + r[2][2] * cz;
        }
    }

    let mut surf = Surface::new();
    surf.set_surf_color("blue").draw(&x_mat, &y_mat, &z_mat);

    let mut axis_line = Curve::new();
    axis_line
        .set_line_color("red")
        .set_line_style("--")
        .set_line_width(2.0)
        .set_label("Hydrostatic Axis");
    axis_line.draw_3d(&[0.0, 15.0], &[0.0, 15.0], &[0.0, 15.0]);

    let mut plot = Plot::new();
    plot.set_figure_size_points(800.0, 800.0)
        .add(&surf)
        .add(&axis_line)
        .set_labels_3d(r"$\sigma_1$", r"$\sigma_2$", r"$\sigma_3$")
        .set_title("von Mises Yield Surface (Cylinder)")
        .set_range_3d(0.0, 15.0, 0.0, 15.0, 0.0, 15.0)
        .set_camera(elev, azim)
        .legend();

    plot.save("/tmp/plotpy/von_mises_surface.png")?;
    println!("Saved /tmp/plotpy/von_mises_surface.png");
    Ok(())
}

fn plot_mohr_coulomb(elev: f64, azim: f64) -> Result<(), StrError> {
    let apex = [-2.0, -2.0, -2.0];

    let r_c = 6.0;
    let r_e = 3.5;

    let angles = [0.0, 60.0, 120.0, 180.0, 240.0, 300.0];
    let radii = [r_c, r_e, r_c, r_e, r_c, r_e];

    let base_center = [12.0, 12.0, 12.0];

    let n = [
        1.0 / 3.0f64.sqrt(),
        1.0 / 3.0f64.sqrt(),
        1.0 / 3.0f64.sqrt(),
    ];
    let u = [
        1.0 / 2.0f64.sqrt(),
        -1.0 / 2.0f64.sqrt(),
        0.0,
    ];
    let v_vec = cross(&n, &u);

    // Create vertices array
    let mut xx = vec![apex[0]];
    let mut yy = vec![apex[1]];
    let mut zz = vec![apex[2]];

    for i in 0..6 {
        let ang = angles[i] * PI / 180.0;
        let r = radii[i];
        let px = base_center[0] + r * (ang.cos() * u[0] + ang.sin() * v_vec[0]);
        let py = base_center[1] + r * (ang.cos() * u[1] + ang.sin() * v_vec[1]);
        let pz = base_center[2] + r * (ang.cos() * u[2] + ang.sin() * v_vec[2]);
        xx.push(px);
        yy.push(py);
        zz.push(pz);
    }

    // connectivity: 6 triangles
    let mut connectivity = vec![vec![0; 3]; 6];
    for i in 0..6 {
        connectivity[i][0] = 0;
        connectivity[i][1] = i + 1;
        connectivity[i][2] = if i == 5 { 1 } else { i + 2 };
    }

    let mut canvas = Canvas::new();
    canvas
        .set_face_color("green")
        .set_edge_color("black")
        .set_line_width(1.0)
        .draw_triangles_3d(&xx, &yy, &zz, &connectivity);

    let mut axis_line = Curve::new();
    axis_line
        .set_line_color("red")
        .set_line_style("--")
        .set_line_width(2.0)
        .set_label("Hydrostatic Axis");
    axis_line.draw_3d(&[-3.0, 15.0], &[-3.0, 15.0], &[-3.0, 15.0]);

    let mut plot = Plot::new();
    plot.set_figure_size_points(800.0, 800.0)
        .add(&canvas)
        .add(&axis_line)
        .set_labels_3d(r"$\sigma_1$", r"$\sigma_2$", r"$\sigma_3$")
        .set_title("Mohr-Coulomb Yield Surface (Hexagonal Pyramid)")
        .set_range_3d(-3.0, 15.0, -3.0, 15.0, -3.0, 15.0)
        .set_camera(elev, azim)
        .legend();

    plot.save("/tmp/plotpy/mohr_coulomb_surface.png")?;
    println!("Saved /tmp/plotpy/mohr_coulomb_surface.png");
    Ok(())
}

fn main() -> Result<(), StrError> {
    let elev = 20.0;
    let azim = 20.0;
    plot_von_mises(elev, azim)?;
    plot_mohr_coulomb(elev, azim)?;
    Ok(())
}
