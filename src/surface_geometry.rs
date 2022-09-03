use crate::{StrError, Surface};
use russell_lab::{generate3d, suq_cos, suq_sin, Matrix};
use std::f64::consts::PI;

impl Surface {
    /// Draws a cylinder
    ///
    /// # Input
    ///
    /// * `a` -- first point on the cylinder (centered) axis
    /// * `b` -- second point on the cylinder (centered) axis
    /// * `radius` -- the cylinder's radius
    /// * `ndiv_axis` -- number of divisions along the axis (≥ 1)
    /// * `ndiv_perimeter` -- number of divisions along the cross-sectional circle perimeter (≥ 3)
    ///
    /// # Example
    ///
    /// ```
    /// use plotpy::{Plot, StrError, Surface};
    /// use std::path::Path;
    ///
    /// fn main() -> Result<(), StrError> {
    ///     // configure and draw surface
    ///     let mut surface = Surface::new();
    ///     let a = &[0.0, 0.0, 0.0];
    ///     let b = &[0.0, 0.0, 1.0];
    ///     surface.set_solid_color("#fcb827")
    ///            .draw_cylinder(a, b, 0.25, 1, 20)?;
    ///
    ///     // add surface to plot
    ///     let mut plot = Plot::new();
    ///     plot.add(&surface);
    ///
    ///     // save figure
    ///     plot.set_range_3d(-1.0, 1.0, -1.0, 1.0, 0.0, 1.0)
    ///         .set_equal_axes(true)
    ///         .save("/tmp/plotpy/doc_tests/doc_cylinder.svg")?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ![doc_cylinder.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/doc_cylinder.svg)
    ///
    /// See also integration tests in the [tests directory](https://github.com/cpmech/plotpy/tree/main/tests)
    pub fn draw_cylinder(
        &mut self,
        a: &[f64],
        b: &[f64],
        radius: f64,
        ndiv_axis: usize,
        ndiv_perimeter: usize,
    ) -> Result<(), StrError> {
        if a.len() != 3 {
            return Err("a.len() must equal to 3");
        }
        if b.len() != 3 {
            return Err("b.len() must equal to 3");
        }
        if ndiv_axis < 1 {
            return Err("ndiv_axis must be ≥ 1");
        }
        if ndiv_perimeter < 3 {
            return Err("ndiv_perimeter must be ≥ 3");
        }
        let (e0, e1, e2) = Surface::aligned_system(a, b)?;
        let cylinder_height =
            f64::sqrt((b[0] - a[0]) * (b[0] - a[0]) + (b[1] - a[1]) * (b[1] - a[1]) + (b[2] - a[2]) * (b[2] - a[2]));
        let (n_height, n_alpha) = (ndiv_axis + 1, ndiv_perimeter + 1);
        let mut x = Matrix::new(n_alpha, n_height);
        let mut y = Matrix::new(n_alpha, n_height);
        let mut z = Matrix::new(n_alpha, n_height);
        let delta_height = cylinder_height / ((n_height - 1) as f64);
        let delta_alpha = 2.0 * std::f64::consts::PI / ((n_alpha - 1) as f64);
        let mut p = vec![0.0; 3];
        for i in 0..n_alpha {
            let v = (i as f64) * delta_alpha;
            for j in 0..n_height {
                let u = (j as f64) * delta_height;
                for k in 0..3 {
                    p[k] = a[k] + u * e0[k] + radius * f64::sin(v) * e1[k] + radius * f64::cos(v) * e2[k];
                }
                x[i][j] = p[0];
                y[i][j] = p[1];
                z[i][j] = p[2];
            }
        }
        self.draw(&x, &y, &z);
        Ok(())
    }

    /// Draws a plane that has a normal vector with a non-zero z (nzz) component
    ///
    /// The plane may be perpendicular to z if n = (0,0,1)
    ///
    /// # Input
    ///
    /// * `p` -- (len=3) point on plane
    /// * `n` -- (len=3) normal vector
    /// * `xmin` and `xmax` -- limits along x
    /// * `ymin` and `ymax` -- limits along y
    /// * `nx` -- number of divisions along x (must be ≥ 2)
    /// * `ny` -- number of divisions along y (must be ≥ 2)
    ///
    /// # Output
    ///
    /// * `x`, `y`, `z` -- the coordinates of all points as in a meshgrid
    ///
    /// # Example
    ///
    /// ```
    /// use plotpy::{Plot, StrError, Surface};
    /// use std::path::Path;
    ///
    /// fn main() -> Result<(), StrError> {
    ///     // configure and draw surface
    ///     let mut surface = Surface::new();
    ///     let p = &[0.0, 0.0, 0.0];
    ///     let n = &[0.0, 0.0, 1.0];
    ///     surface.set_solid_color("#5359e9")
    ///            .draw_plane_nzz(p, n, -1.0, 1.0, -1.0, 1.0, 3, 3)?;
    ///
    ///     // add surface to plot
    ///     let mut plot = Plot::new();
    ///     plot.add(&surface);
    ///
    ///     // save figure
    ///     plot.set_range_3d(-1.0, 1.0, -1.0, 1.0, 0.0, 1.0)
    ///         .set_equal_axes(true)
    ///         .save("/tmp/plotpy/doc_tests/doc_plane_nzz.svg")?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ![doc_plane_nzz.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/doc_plane_nzz.svg)
    ///
    /// See also integration test in the **tests** directory.
    ///
    pub fn draw_plane_nzz(
        &mut self,
        p: &[f64],
        n: &[f64],
        xmin: f64,
        xmax: f64,
        ymin: f64,
        ymax: f64,
        nx: usize,
        ny: usize,
    ) -> Result<(Matrix, Matrix, Matrix), StrError> {
        if p.len() != 3 || n.len() != 3 {
            return Err("p.len() and n.len() must be equal to 3");
        }
        if f64::abs(n[2]) < 1e-10 {
            return Err("the z-component of the normal vector cannot be zero");
        }
        if nx < 2 || ny < 2 {
            return Err("nx and ny must be ≥ 2");
        }
        let d = -n[0] * p[0] - n[1] * p[1] - n[2] * p[2];
        let (x, y, z) = generate3d(xmin, xmax, ymin, ymax, nx + 1, ny + 1, |x, y| {
            (-d - n[0] * x - n[1] * y) / n[2]
        });
        self.draw(&x, &y, &z);
        Ok((x, y, z))
    }

    /// Draws a hemisphere
    ///
    /// # Input
    ///
    /// * `c` -- (len=3) center coordinates
    /// * `r` -- radius
    /// * `alpha_min` -- min α angle in [-180, 180) degrees
    /// * `alpha_max` -- max α angle in (-180, 180] degrees
    /// * `n_alpha` -- number of divisions along α (must be ≥ 2)
    /// * `n_theta` -- number of divisions along θ (must be ≥ 2)
    /// * `cup` -- upside-down; like a cup
    ///
    /// # Output
    ///
    /// * `x`, `y`, `z` -- the coordinates of all points as in a meshgrid
    ///
    /// # Example
    ///
    /// ```
    /// use plotpy::{Plot, StrError, Surface};
    /// use std::path::Path;
    ///
    /// fn main() -> Result<(), StrError> {
    ///     // draw hat
    ///     let mut hat = Surface::new();
    ///     let c = &[-0.5, 0.0, 0.0];
    ///     hat.set_solid_color("#17af14")
    ///        .draw_hemisphere(c, 0.5, -180.0, 180.0, 20, 20, false)?;
    ///
    ///     // draw cup
    ///     let mut cup = Surface::new();
    ///     let c = &[0.5, 0.0, 0.0];
    ///     cup.set_solid_color("#ff8787")
    ///        .draw_hemisphere(c, 0.5, -180.0, 180.0, 20, 20, true)?;
    ///
    ///     // add surfaces to plot
    ///     let mut plot = Plot::new();
    ///     plot.add(&hat).add(&cup);
    ///
    ///     // save figure
    ///     plot.set_range_3d(-1.0, 1.0, -1.0, 1.0, 0.0, 1.0)
    ///         .set_equal_axes(true)
    ///         .save("/tmp/plotpy/doc_tests/doc_hemisphere.svg")?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ![doc_hemisphere.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/doc_hemisphere.svg)
    ///
    /// See also integration test in the **tests** directory.
    ///
    pub fn draw_hemisphere(
        &mut self,
        c: &[f64],
        r: f64,
        alpha_min: f64,
        alpha_max: f64,
        n_alpha: usize,
        n_theta: usize,
        cup: bool,
    ) -> Result<(Matrix, Matrix, Matrix), StrError> {
        if c.len() != 3 {
            return Err("c.len() must be equal to 3");
        }
        if n_alpha < 2 || n_theta < 2 {
            return Err("n_alpha and n_theta must be ≥ 2");
        }
        let a_min = alpha_min * PI / 180.0;
        let a_max = alpha_max * PI / 180.0;
        let d_alpha = (a_max - a_min) / (n_alpha as f64);
        let d_theta = (PI / 2.0) / (n_theta as f64);
        let mut x = Matrix::new(n_alpha + 1, n_theta + 1);
        let mut y = Matrix::new(n_alpha + 1, n_theta + 1);
        let mut z = Matrix::new(n_alpha + 1, n_theta + 1);
        for i in 0..n_alpha + 1 {
            let alpha = a_min + (i as f64) * d_alpha;
            for j in 0..n_theta + 1 {
                let theta = (j as f64) * d_theta;
                if cup {
                    x[i][j] = c[0] + r * f64::cos(alpha) * f64::sin(theta);
                    y[i][j] = c[1] + r * f64::sin(alpha) * f64::sin(theta);
                    z[i][j] = c[2] - r * f64::cos(theta);
                } else {
                    x[i][j] = c[0] + r * f64::cos(alpha) * f64::sin(theta);
                    y[i][j] = c[1] + r * f64::sin(alpha) * f64::sin(theta);
                    z[i][j] = c[2] + r * f64::cos(theta);
                }
            }
        }
        self.draw(&x, &y, &z);
        Ok((x, y, z))
    }

    /// Draws a superquadric (includes sphere, super-ellipsoid, and super-hyperboloid)
    ///
    /// # Input
    ///
    /// * `c` -- (len=3) center coordinates
    /// * `r` -- (len=3) radii
    /// * `k` -- (len=3) exponents (must all be ≥ 0)
    /// * `alpha_min` -- min α angle in [-180, 180) degrees
    /// * `alpha_max` -- max α angle in (-180, 180] degrees
    /// * `theta_min` -- min θ angle in [-90, 90) degrees
    /// * `theta_max` -- max θ angle in (-90, 90] degrees
    /// * `n_alpha` -- number of divisions along α (must be ≥ 2)
    /// * `n_theta` -- number of divisions along θ (must be ≥ 2)
    ///
    /// # Output
    ///
    /// * `x`, `y`, `z` -- the coordinates of all points as in a meshgrid
    ///
    /// Reference: <https://en.wikipedia.org/wiki/Superquadrics>
    ///
    /// # Example
    ///
    /// ```
    /// use plotpy::{Plot, StrError, Surface};
    /// use std::path::Path;
    ///
    /// fn main() -> Result<(), StrError> {
    ///     // configure and draw surface
    ///     let c = &[0.0, 0.0, 0.0];
    ///     let r = &[1.0, 1.0, 1.0];
    ///     let k = &[1.0, 2.0, 0.5];
    ///     let mut surface = Surface::new();
    ///     surface.set_solid_color("#cd0000")
    ///         .draw_superquadric(c, r, k, -180.0, 180.0, -90.0, 90.0, 40, 20)?;
    ///
    ///     // add surface to plot
    ///     let mut plot = Plot::new();
    ///     plot.add(&surface);
    ///
    ///     // save figure
    ///     plot.set_equal_axes(true)
    ///         .save("/tmp/plotpy/doc_tests/doc_superquadric.svg")?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ![doc_superquadric.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/doc_superquadric.svg)
    ///
    /// See also integration test in the **tests** directory.
    ///
    pub fn draw_superquadric(
        &mut self,
        c: &[f64],
        r: &[f64],
        k: &[f64],
        alpha_min: f64,
        alpha_max: f64,
        theta_min: f64,
        theta_max: f64,
        n_alpha: usize,
        n_theta: usize,
    ) -> Result<(Matrix, Matrix, Matrix), StrError> {
        if c.len() != 3 || r.len() != 3 || k.len() != 3 {
            return Err("c.len(), r.len(), and k.len() must be equal to 3");
        }
        if n_alpha < 2 || n_theta < 2 {
            return Err("n_alpha and n_theta must be ≥ 2");
        }
        if k[0] < 0.0 || k[1] < 0.0 || k[2] < 0.0 {
            return Err("exponents k must be greater than zero");
        }
        let (aa, bb, cc) = (2.0 / k[0], 2.0 / k[1], 2.0 / k[2]);
        let a_min = alpha_min * PI / 180.0;
        let a_max = alpha_max * PI / 180.0;
        let t_min = theta_min * PI / 180.0;
        let t_max = theta_max * PI / 180.0;
        let d_alpha = (a_max - a_min) / (n_alpha as f64);
        let d_theta = (t_max - t_min) / (n_theta as f64);
        let mut x = Matrix::new(n_alpha + 1, n_theta + 1);
        let mut y = Matrix::new(n_alpha + 1, n_theta + 1);
        let mut z = Matrix::new(n_alpha + 1, n_theta + 1);
        for i in 0..n_alpha + 1 {
            let alpha = a_min + (i as f64) * d_alpha;
            for j in 0..n_theta + 1 {
                let theta = t_min + (j as f64) * d_theta;
                x[i][j] = c[0] + r[0] * suq_cos(theta, aa) * suq_cos(alpha, aa);
                y[i][j] = c[1] + r[1] * suq_cos(theta, bb) * suq_sin(alpha, bb);
                z[i][j] = c[2] + r[2] * suq_sin(theta, cc);
            }
        }
        self.draw(&x, &y, &z);
        Ok((x, y, z))
    }

    /// Draws a sphere
    ///
    /// # Input
    ///
    /// * `c` -- (len=3) center coordinates
    /// * `r` -- radius
    /// * `n_alpha` -- number of divisions along α (must be ≥ 2)
    /// * `n_theta` -- number of divisions along θ (must be ≥ 2)
    ///
    /// # Output:
    ///
    /// * `x`, `y`, `z` -- the coordinates of all points as in a meshgrid
    ///
    /// # Example
    ///
    /// ```
    /// use plotpy::{Plot, StrError, Surface};
    /// use std::path::Path;
    ///
    /// fn main() -> Result<(), StrError> {
    ///     // configure and draw surface
    ///     let mut surface = Surface::new();
    ///     let c = &[0.0, 0.0, 0.0];
    ///     surface.set_solid_color("#7812c3")
    ///            .draw_sphere(c, 1.0, 20, 20)?;
    ///
    ///     // add surface to plot
    ///     let mut plot = Plot::new();
    ///     plot.add(&surface);
    ///
    ///     // save figure
    ///     plot.set_equal_axes(true)
    ///         .save("/tmp/plotpy/doc_tests/doc_sphere.svg")?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// ![doc_sphere.svg](https://raw.githubusercontent.com/cpmech/plotpy/main/figures/doc_sphere.svg)
    ///
    /// See also integration test in the **tests** directory.
    ///
    pub fn draw_sphere(
        &mut self,
        c: &[f64],
        r: f64,
        n_alpha: usize,
        n_theta: usize,
    ) -> Result<(Matrix, Matrix, Matrix), StrError> {
        if c.len() != 3 {
            return Err("c.len() must be equal to 3");
        }
        if n_alpha < 2 || n_theta < 2 {
            return Err("n_alpha and n_theta must be ≥ 2");
        }
        let (alpha_min, alpha_max) = (-180.0, 180.0);
        let (theta_min, theta_max) = (-90.0, 90.0);
        self.draw_superquadric(
            c,
            &[r, r, r],
            &[2.0, 2.0, 2.0],
            alpha_min,
            alpha_max,
            theta_min,
            theta_max,
            n_alpha,
            n_theta,
        )
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::Surface;
    use crate::GraphMaker;

    #[test]
    fn draw_cylinder_fails_on_wrong_input() {
        let mut surf = Surface::new();
        let res = surf.draw_cylinder(&[0.0, 0.0], &[1.0, 1.0, 1.0], 1.0, 1, 3);
        assert_eq!(res.err(), Some("a.len() must equal to 3"));

        let res = surf.draw_cylinder(&[0.0, 0.0, 0.0], &[1.0, 1.0], 1.0, 1, 3);
        assert_eq!(res.err(), Some("b.len() must equal to 3"));

        let res = surf.draw_cylinder(&[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0], 1.0, 0, 3);
        assert_eq!(res.err(), Some("ndiv_axis must be ≥ 1"));

        let res = surf.draw_cylinder(&[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0], 1.0, 1, 2);
        assert_eq!(res.err(), Some("ndiv_perimeter must be ≥ 3"));

        let res = surf.draw_cylinder(&[0.0, 0.0, 0.0], &[0.0, 0.0, 0.0], 1.0, 1, 3);
        assert_eq!(res.err(), Some("a-to-b segment is too short"));
    }

    #[test]
    fn draw_cylinder_works() {
        let mut surf = Surface::new();
        surf.draw_cylinder(&[0.0, 0.0, 0.0], &[1.0, 0.0, 0.0], 1.0, 2, 3)
            .unwrap();
        assert!(surf.get_buffer().len() > 0);
    }

    #[test]
    fn draw_plane_nzz_fails_on_wrong_input() {
        let mut surf = Surface::new();
        let res = surf.draw_plane_nzz(&[0.0, 0.0], &[1.0, 1.0], 0.0, 1.0, 0.0, 1.0, 2, 2);
        assert_eq!(res.err(), Some("p.len() and n.len() must be equal to 3"));
        let res = surf.draw_plane_nzz(&[0.0, 0.0, 0.0], &[1.0, 1.0], 0.0, 1.0, 0.0, 1.0, 2, 2);
        assert_eq!(res.err(), Some("p.len() and n.len() must be equal to 3"));

        let res = surf.draw_plane_nzz(&[0.0, 0.0, 0.0], &[1.0, 1.0, 0.0], 0.0, 1.0, 0.0, 1.0, 2, 2);
        assert_eq!(res.err(), Some("the z-component of the normal vector cannot be zero"));

        let res = surf.draw_plane_nzz(&[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0], 0.0, 1.0, 0.0, 1.0, 1, 2);
        assert_eq!(res.err(), Some("nx and ny must be ≥ 2"));
        let res = surf.draw_plane_nzz(&[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0], 0.0, 1.0, 0.0, 1.0, 2, 1);
        assert_eq!(res.err(), Some("nx and ny must be ≥ 2"));
    }

    #[test]
    fn draw_plane_nzz_works() {
        let mut surf = Surface::new();
        surf.draw_plane_nzz(&[0.0, 0.0, 0.0], &[1.0, 1.0, 1.0], 0.0, 1.0, 0.0, 1.0, 2, 2)
            .unwrap();
        assert!(surf.get_buffer().len() > 0);
    }

    #[test]
    fn draw_hemisphere_fails_on_wrong_input() {
        let mut surf = Surface::new();
        let res = surf.draw_hemisphere(&[0.0, 0.0], 1.0, 0.0, 180.0, 2, 2, false);
        assert_eq!(res.err(), Some("c.len() must be equal to 3"));

        let res = surf.draw_hemisphere(&[0.0, 0.0, 0.0], 1.0, 0.0, 180.0, 1, 2, false);
        assert_eq!(res.err(), Some("n_alpha and n_theta must be ≥ 2"));
        let res = surf.draw_hemisphere(&[0.0, 0.0, 0.0], 1.0, 0.0, 180.0, 2, 1, false);
        assert_eq!(res.err(), Some("n_alpha and n_theta must be ≥ 2"));
    }

    #[test]
    fn draw_hemisphere_works() {
        let mut surf = Surface::new();
        surf.draw_hemisphere(&[0.0, 0.0, 0.0], 1.0, 0.0, 180.0, 2, 2, true)
            .unwrap();
        assert!(surf.get_buffer().len() > 0);
        surf.draw_hemisphere(&[0.0, 0.0, 0.0], 1.0, 0.0, 180.0, 2, 2, false)
            .unwrap();
        assert!(surf.get_buffer().len() > 0);
    }

    #[test]
    fn draw_superquadric_fails_on_wrong_input() {
        let d2 = &[0.0, 0.0];
        let d3 = &[0.0, 0.0, 0.0];

        let mut surf = Surface::new();
        let res = surf.draw_superquadric(d2, d3, d3, 0.0, 180.0, 0.0, 180.0, 2, 2);
        assert_eq!(res.err(), Some("c.len(), r.len(), and k.len() must be equal to 3"));
        let res = surf.draw_superquadric(d3, d2, d3, 0.0, 180.0, 0.0, 180.0, 2, 2);
        assert_eq!(res.err(), Some("c.len(), r.len(), and k.len() must be equal to 3"));
        let res = surf.draw_superquadric(d3, d3, d2, 0.0, 180.0, 0.0, 180.0, 2, 2);
        assert_eq!(res.err(), Some("c.len(), r.len(), and k.len() must be equal to 3"));

        let res = surf.draw_superquadric(d3, d3, d3, 0.0, 180.0, 0.0, 180.0, 1, 2);
        assert_eq!(res.err(), Some("n_alpha and n_theta must be ≥ 2"));

        let ka = &[-1.0, 0.0, 0.0];
        let kb = &[0.0, -1.0, 0.0];
        let kc = &[0.0, 0.0, -1.0];
        let res = surf.draw_superquadric(d3, d3, ka, 0.0, 180.0, 0.0, 180.0, 2, 2);
        assert_eq!(res.err(), Some("exponents k must be greater than zero"));
        let res = surf.draw_superquadric(d3, d3, kb, 0.0, 180.0, 0.0, 180.0, 2, 2);
        assert_eq!(res.err(), Some("exponents k must be greater than zero"));
        let res = surf.draw_superquadric(d3, d3, kc, 0.0, 180.0, 0.0, 180.0, 2, 2);
        assert_eq!(res.err(), Some("exponents k must be greater than zero"));
    }

    #[test]
    fn draw_superquadric_works() {
        let mut surf = Surface::new();
        surf.draw_superquadric(
            &[0.0, 0.0, 0.0],
            &[1.0, 1.0, 1.0],
            &[2.0, 2.0, 2.0],
            0.0,
            180.0,
            0.0,
            180.0,
            2,
            2,
        )
        .unwrap();
        assert!(surf.get_buffer().len() > 0);
    }

    #[test]
    fn draw_sphere_fails_on_wrong_input() {
        let mut surf = Surface::new();
        let res = surf.draw_sphere(&[0.0, 0.0], 1.0, 2, 2);
        assert_eq!(res.err(), Some("c.len() must be equal to 3"));

        let res = surf.draw_sphere(&[0.0, 0.0, 0.0], 1.0, 1, 2);
        assert_eq!(res.err(), Some("n_alpha and n_theta must be ≥ 2"));
        let res = surf.draw_sphere(&[0.0, 0.0, 0.0], 1.0, 2, 1);
        assert_eq!(res.err(), Some("n_alpha and n_theta must be ≥ 2"));
    }

    #[test]
    fn draw_sphere_works() {
        let mut surf = Surface::new();
        surf.draw_sphere(&[0.0, 0.0, 0.0], 1.0, 2, 2).unwrap();
        assert!(surf.get_buffer().len() > 0);
    }
}
