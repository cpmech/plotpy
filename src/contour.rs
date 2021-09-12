use super::*;
use std::fmt::Write;

/// Generates a contour plot
pub struct Contour {
    /// colors
    pub colors: Vec<String>,

    /// levels (may be nil)
    pub levels: Vec<f64>,

    /// colormap index
    pub colormap_index: i32,

    /// number format
    pub number_format: String,

    /// no lines on top of filled contour
    pub no_lines: bool,

    /// no labels
    pub no_labels: bool,

    /// no labels 'inline'
    pub no_inline: bool,

    /// no colorbar
    pub no_colorbar: bool,

    /// colorbar label
    pub colorbar_label: String,

    /// selected value
    pub selected_value: f64,

    /// color to mark selected level
    pub selected_color: String,

    /// zero level linewidth
    pub selected_linewidth: f64,

    // buffer
    pub(crate) buffer: String,
}

impl Contour {
    pub fn new() -> Self {
        Contour {
            colors: Vec::new(),
            levels: Vec::new(),
            colormap_index: 0,
            number_format: String::new(),
            no_lines: false,
            no_labels: false,
            no_inline: false,
            no_colorbar: false,
            colorbar_label: String::new(),
            selected_value: 0.0,
            selected_color: String::new(),
            selected_linewidth: 0.0,
            buffer: String::new(),
        }
    }

    pub fn draw_filled(&mut self, x: &[&[f64]], y: &[&[f64]], z: &[&[f64]]) -> Result<(), &'static str> {
        vec_vec_to_numpy_array_2d(&mut self.buffer, "x", x)?;
        vec_vec_to_numpy_array_2d(&mut self.buffer, "y", y)?;
        vec_vec_to_numpy_array_2d(&mut self.buffer, "z", z)?;
        let opt = self.options();
        write!(&mut self.buffer, "plt.contourf(x,y,z{})\n", &opt).unwrap();
        Ok(())
    }

    pub(crate) fn options(&self) -> String {
        let mut opt = String::new();
        if self.colors.len() > 0 {
            write!(&mut opt, ",colors={}", vec_to_py_list_str(&self.colors)).unwrap();
        } else {
            write!(&mut opt, ",cmap=getColormap({})", self.colormap_index).unwrap();
        }
        if self.levels.len() > 0 {
            write!(&mut opt, ",levels={}", vec_to_py_list_num(&self.levels)).unwrap();
        }
        opt
    }
}

impl GraphMaker for Contour {
    fn get_buffer<'a>(&'a self) -> &'a String {
        &self.buffer
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_xyz() -> (
        &'static [&'static [f64]],
        &'static [&'static [f64]],
        &'static [&'static [f64]],
    ) {
        let x: &[&[f64]] = &[
            &[-1.0, -0.5, 0.0, 0.5],
            &[-1.0, -0.5, 0.0, 0.5],
            &[-1.0, -0.5, 0.0, 0.5],
            &[-1.0, -0.5, 0.0, 0.5],
        ];
        let y: &[&[f64]] = &[
            &[-1.0, -1.0, -1.0, -1.0],
            &[-0.5, -0.5, -0.5, -0.5],
            &[0.0, 0.0, 0.0, 0.0],
            &[0.5, 0.5, 0.5, 0.5],
        ];
        let z: &[&[f64]] = &[
            &[2.00, 1.25, 1.00, 1.25],
            &[1.25, 0.50, 0.25, 0.50],
            &[1.00, 0.25, 0.00, 0.25],
            &[1.25, 0.50, 0.25, 0.50],
        ];
        (x, y, z)
    }

    #[test]
    fn options_works() -> Result<(), &'static str> {
        let mut contour = Contour::new();
        contour.colors = vec!["#f00".to_string(), "#0f0".to_string(), "#00f".to_string()];
        contour.levels = vec![0.25, 0.5, 1.0];
        contour.colormap_index = 4;
        contour.number_format = "%.4f".to_string();
        contour.no_lines = true;
        contour.no_labels = true;
        contour.no_inline = true;
        contour.no_colorbar = true;
        contour.colorbar_label = "temperature".to_string();
        contour.selected_value = 0.75;
        contour.selected_linewidth = 2.0;
        let opt = contour.options();
        assert_eq!(
            opt,
            ",colors=['#f00','#0f0','#00f']\
             ,levels=[0.25,0.5,1]"
        );
        Ok(())
    }

    #[test]
    fn draw_filled_works() -> Result<(), &'static str> {
        let mut contour = Contour::new();
        let (x, y, z) = gen_xyz();
        contour.draw_filled(x, y, z)?;
        let correct: &str = "x=np.array([[-1,-0.5,0,0.5,],[-1,-0.5,0,0.5,],[-1,-0.5,0,0.5,],[-1,-0.5,0,0.5,],],dtype=float)\n\
                             y=np.array([[-1,-1,-1,-1,],[-0.5,-0.5,-0.5,-0.5,],[0,0,0,0,],[0.5,0.5,0.5,0.5,],],dtype=float)\n\
                             z=np.array([[2,1.25,1,1.25,],[1.25,0.5,0.25,0.5,],[1,0.25,0,0.25,],[1.25,0.5,0.25,0.5,],],dtype=float)\n\
                             plt.contourf(x,y,z,cmap=getColormap(0))\n";
        assert_eq!(contour.buffer, correct);
        Ok(())
    }
}
