/// Implements the sign function
///
/// ```text
///           │ -1   if x < 0
/// sign(x) = ┤  0   if x = 0
///           │  1   if x > 0
///
///           |x|    x
/// sign(x) = ——— = ———
///            x    |x|
///
/// sign(x) = 2 · heaviside(x) - 1
/// ```
///
/// Reference: <https://en.wikipedia.org/wiki/Sign_function>
pub fn sign(x: f64) -> f64 {
    if x < 0.0 {
        -1.0
    } else if x > 0.0 {
        1.0
    } else {
        0.0
    }
}

/// Implements the superquadric function involving sin(x)
///
/// ```text
/// suq_sin(x;k) = sign(sin(x)) · |sin(x)|ᵏ
/// ```
///
/// `suq_sin(x;k)` is the `f(ω;m)` function from <https://en.wikipedia.org/wiki/Superquadrics>
pub fn suq_sin(x: f64, k: f64) -> f64 {
    sign(f64::sin(x)) * f64::powf(f64::abs(f64::sin(x)), k)
}

/// Implements the superquadric auxiliary involving cos(x)
///
/// ```text
/// suq_cos(x;k) = sign(cos(x)) · |cos(x)|ᵏ
/// ```
///
/// `suq_cos(x;k)` is the `g(ω;m)` function from <https://en.wikipedia.org/wiki/Superquadrics>
pub fn suq_cos(x: f64, k: f64) -> f64 {
    sign(f64::cos(x)) * f64::powf(f64::abs(f64::cos(x)), k)
}

/// Returns evenly spaced numbers over a specified closed interval
pub fn linspace(start: f64, stop: f64, count: usize) -> Vec<f64> {
    if count == 0 {
        return Vec::new();
    }
    let mut res = vec![0.0; count];
    res[0] = start;
    if count == 1 {
        return res;
    }
    res[count - 1] = stop;
    if count == 2 {
        return res;
    }
    let den = (count - 1) as f64;
    let step = (stop - start) / den;
    for i in 1..count {
        let p = i as f64;
        res[i] = start + p * step;
    }
    res
}

/// Generates 2d points (meshgrid)
///
/// # Input
///
/// * `xmin`, `xmax` -- range along x
/// * `ymin`, `ymax` -- range along y
/// * `nx` -- is the number of points along x (must be `>= 2`)
/// * `ny` -- is the number of points along y (must be `>= 2`)
///
/// # Output
///
/// * `x`, `y` -- (`ny` by `nx`) 2D arrays
pub fn generate2d(xmin: f64, xmax: f64, ymin: f64, ymax: f64, nx: usize, ny: usize) -> (Vec<Vec<f64>>, Vec<Vec<f64>>) {
    let mut x = vec![vec![0.0; nx]; ny];
    let mut y = vec![vec![0.0; nx]; ny];
    if nx == 0 || ny == 0 {
        return (x, y);
    }
    let dx = if nx == 1 {
        xmin
    } else {
        (xmax - xmin) / ((nx - 1) as f64)
    };
    let dy = if ny == 1 {
        ymin
    } else {
        (ymax - ymin) / ((ny - 1) as f64)
    };
    for i in 0..ny {
        let v = ymin + (i as f64) * dy;
        for j in 0..nx {
            let u = xmin + (j as f64) * dx;
            x[i][j] = u;
            y[i][j] = v;
        }
    }
    (x, y)
}

/// Generates 3d points (function over meshgrid)
///
/// # Input
///
/// * `xmin`, `xmax` -- range along x
/// * `ymin`, `ymax` -- range along y
/// * `nx` -- is the number of points along x (must be `>= 2`)
/// * `ny` -- is the number of points along y (must be `>= 2`)
/// * `calc_z` -- is a function of (xij, yij) that calculates zij
///
/// # Output
///
/// * `x`, `y`, `z` -- (`ny` by `nx`) 2D arrays
pub fn generate3d<F>(
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
    nx: usize,
    ny: usize,
    calc_z: F,
) -> (Vec<Vec<f64>>, Vec<Vec<f64>>, Vec<Vec<f64>>)
where
    F: Fn(f64, f64) -> f64,
{
    let mut x = vec![vec![0.0; nx]; ny];
    let mut y = vec![vec![0.0; nx]; ny];
    let mut z = vec![vec![0.0; nx]; ny];
    if nx == 0 || ny == 0 {
        return (x, y, z);
    }
    let dx = if nx == 1 {
        xmin
    } else {
        (xmax - xmin) / ((nx - 1) as f64)
    };
    let dy = if ny == 1 {
        ymin
    } else {
        (ymax - ymin) / ((ny - 1) as f64)
    };
    for i in 0..ny {
        let v = ymin + (i as f64) * dy;
        for j in 0..nx {
            let u = xmin + (j as f64) * dx;
            x[i][j] = u;
            y[i][j] = v;
            z[i][j] = calc_z(u, v);
        }
    }
    (x, y, z)
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::{generate2d, generate3d, linspace, sign, suq_cos, suq_sin};

    fn approx_eq(a: f64, b: f64, tol: f64) {
        let diff = f64::abs(a - b);
        if diff > tol {
            panic!("numbers are not approximately equal. diff = {:?}", diff);
        }
    }

    #[test]
    #[should_panic(expected = "numbers are not approximately equal. diff = 1.0")]
    fn approx_eq_captures_errors() {
        approx_eq(1.0, 2.0, 1e-15);
    }

    #[test]
    fn sign_works() {
        let xx = [-2.0, -1.6, -1.2, -0.8, -0.4, 0.0, 0.4, 0.8, 1.2, 1.6, 2.0];
        for x in xx {
            let s = sign(x);
            if x == 0.0 {
                assert_eq!(s, 0.0);
            } else {
                assert_eq!(s, f64::abs(x) / x);
            }
        }
    }

    #[test]
    fn suq_sin_and_cos_work() {
        const PI: f64 = std::f64::consts::PI;
        approx_eq(suq_sin(0.0, 1.0), 0.0, 1e-14);
        approx_eq(suq_sin(PI, 1.0), 0.0, 1e-14);
        approx_eq(suq_sin(PI / 2.0, 0.0), 1.0, 1e-14);
        approx_eq(suq_sin(PI / 2.0, 1.0), 1.0, 1e-14);
        approx_eq(suq_sin(PI / 2.0, 2.0), 1.0, 1e-14);
        approx_eq(suq_sin(PI / 4.0, 2.0), 0.5, 1e-14);
        approx_eq(suq_sin(-PI / 4.0, 2.0), -0.5, 1e-14);

        approx_eq(suq_cos(0.0, 1.0), 1.0, 1e-14);
        approx_eq(suq_cos(PI, 1.0), -1.0, 1e-14);
        approx_eq(suq_cos(PI / 2.0, 0.0), 1.0, 1e-14); // because sign(cos(pi/2))=1
        approx_eq(suq_cos(PI / 2.0, 1.0), 0.0, 1e-14);
        approx_eq(suq_cos(PI / 2.0, 2.0), 0.0, 1e-14);
        approx_eq(suq_cos(PI / 4.0, 2.0), 0.5, 1e-14);
        approx_eq(suq_cos(-PI / 4.0, 2.0), 0.5, 1e-14);
    }

    #[test]
    fn linspace_works() {
        let x = linspace(0.0, 1.0, 11);
        let correct = &[0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
        let mut k = 0;
        for v in &x {
            approx_eq(*v, correct[k], 1e-15);
            k += 1;
        }

        let x = linspace(2.0, 3.0, 0);
        assert_eq!(x.len(), 0);

        let x = linspace(2.0, 3.0, 1);
        assert_eq!(x.len(), 1);
        assert_eq!(x[0], 2.0);

        let x = linspace(2.0, 3.0, 2);
        assert_eq!(x.len(), 2);
        assert_eq!(x[0], 2.0);
        assert_eq!(x[1], 3.0);

        let x = linspace(0.0, 10.0, 0);
        assert_eq!(x.len(), 0);

        let x = linspace(0.0, 10.0, 1);
        assert_eq!(x, &[0.0]);

        let x = linspace(0.0, 10.0, 2);
        assert_eq!(x, [0.0, 10.0]);

        let x = linspace(0.0, 10.0, 3);
        assert_eq!(x, [0.0, 5.0, 10.0]);
    }

    #[test]
    fn generate2d_edge_cases_work() {
        let (x, y) = generate2d(-1.0, 1.0, -3.0, 3.0, 0, 0);
        assert_eq!(x.len(), 0);
        assert_eq!(y.len(), 0);

        let (x, y) = generate2d(-1.0, 1.0, -3.0, 3.0, 1, 1);
        assert_eq!(x.len(), 1);
        assert_eq!(y.len(), 1);
        assert_eq!(x[0], &[-1.0]);
        assert_eq!(y[0], &[-3.0]);
    }

    #[test]
    fn generate2d_works() {
        let (x, y) = generate2d(-1.0, 1.0, -3.0, 3.0, 0, 2);
        assert_eq!(x.len(), 2);
        assert_eq!(y.len(), 2);
        assert_eq!(x[0].len(), 0);
        assert_eq!(x[1].len(), 0);
        assert_eq!(y[0].len(), 0);
        assert_eq!(y[1].len(), 0);

        let (x, y) = generate2d(-1.0, 1.0, -3.0, 3.0, 2, 0);
        assert_eq!(x.len(), 0);
        assert_eq!(y.len(), 0);

        let (x, y) = generate2d(-1.0, 1.0, -3.0, 3.0, 1, 2);
        assert_eq!(x, &[[-1.0], [-1.0]]);
        assert_eq!(y, &[[-3.0], [3.0]]);

        let (x, y) = generate2d(-1.0, 1.0, -3.0, 3.0, 2, 1);
        assert_eq!(x, &[[-1.0, 1.0]]);
        assert_eq!(y, &[[-3.0, -3.0]]);

        let (x, y) = generate2d(-1.0, 1.0, -3.0, 3.0, 2, 3);
        // -1.0, 1.0,
        // -1.0, 1.0,
        // -1.0, 1.0,
        assert_eq!(x, &[[-1.0, 1.0], [-1.0, 1.0], [-1.0, 1.0]]);
        // -3.0, -3.0,
        //  0.0,  0.0,
        //  3.0,  3.0,
        assert_eq!(y, &[[-3.0, -3.0], [0.0, 0.0], [3.0, 3.0]]);
    }

    fn calc_z(x: f64, y: f64) -> f64 {
        x + y
    }

    #[test]
    fn generate3d_edge_cases_work() {
        let (x, y, z) = generate3d(-1.0, 1.0, -3.0, 3.0, 0, 0, calc_z);
        assert_eq!(x.len(), 0);
        assert_eq!(y.len(), 0);
        assert_eq!(z.len(), 0);

        let (x, y, z) = generate3d(-1.0, 1.0, -3.0, 3.0, 1, 1, calc_z);
        assert_eq!(x.len(), 1);
        assert_eq!(y.len(), 1);
        assert_eq!(z.len(), 1);
        assert_eq!(x[0], &[-1.0]);
        assert_eq!(y[0], &[-3.0]);
        assert_eq!(z[0], &[-4.0]);
    }

    #[test]
    fn generate3d_works() {
        let (x, y, z) = generate3d(-1.0, 1.0, -3.0, 3.0, 0, 2, calc_z);
        assert_eq!(x.len(), 2);
        assert_eq!(y.len(), 2);
        assert_eq!(z.len(), 2);
        assert_eq!(x[0].len(), 0);
        assert_eq!(x[1].len(), 0);
        assert_eq!(y[0].len(), 0);
        assert_eq!(y[1].len(), 0);
        assert_eq!(z[0].len(), 0);
        assert_eq!(z[1].len(), 0);

        let (x, y, z) = generate3d(-1.0, 1.0, -3.0, 3.0, 2, 0, calc_z);
        assert_eq!(x.len(), 0);
        assert_eq!(y.len(), 0);
        assert_eq!(z.len(), 0);

        let (x, y, z) = generate3d(-1.0, 1.0, -3.0, 3.0, 1, 2, calc_z);
        assert_eq!(x.len(), 2);
        assert_eq!(y.len(), 2);
        assert_eq!(z.len(), 2);
        assert_eq!(x, &[[-1.0], [-1.0]]);
        assert_eq!(y, &[[-3.0], [3.0]]);
        assert_eq!(z, &[[-4.0], [2.0]]);

        let (x, y, z) = generate3d(-1.0, 1.0, -3.0, 3.0, 2, 1, calc_z);
        assert_eq!(x.len(), 1);
        assert_eq!(y.len(), 1);
        assert_eq!(z.len(), 1);
        assert_eq!(x, &[[-1.0, 1.0]]);
        assert_eq!(y, &[[-3.0, -3.0]]);
        assert_eq!(z, &[[-4.0, -2.0]]);

        let (x, y, z) = generate3d(-1.0, 1.0, -3.0, 3.0, 2, 3, calc_z);
        // -1.0, 1.0,
        // -1.0, 1.0,
        // -1.0, 1.0,
        assert_eq!(x, &[[-1.0, 1.0], [-1.0, 1.0], [-1.0, 1.0]]);
        // -3.0, -3.0,
        //  0.0,  0.0,
        //  3.0,  3.0,
        assert_eq!(y, &[[-3.0, -3.0], [0.0, 0.0], [3.0, 3.0]]);
        // -4.0, -2.0,
        // -1.0,  1.0,
        //  2.0,  4.0,
        assert_eq!(z, &[[-4.0, -2.0], [-1.0, 1.0], [2.0, 4.0]]);
    }
}
