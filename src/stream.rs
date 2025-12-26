use super::GraphMaker;
use crate::conversions::matrix_to_array;
use crate::AsMatrix;
use num_traits::Num;
use std::fmt::Write;

/// Implements functions to illustrate vector fields using streamlines and quiver plots
pub struct Stream {
    // common options
    color: String,

    // streamplot options
    streamplot_linewidth: f64,
    streamplot_arrow_style: String,
    streamplot_density: f64,
    streamplot_extra: String,

    // quiver options
    quiver_scale: f64,
    quiver_pivot: String,
    quiver_extra: String,

    // buffer
    buffer: String,
}

impl Stream {
    /// Creates a new Stream object
    pub fn new() -> Self {
        Stream {
            // common options
            color: String::new(),
            // streamplot options
            streamplot_linewidth: 0.0,
            streamplot_arrow_style: String::new(),
            streamplot_density: 0.0,
            streamplot_extra: String::new(),
            // quiver options
            quiver_scale: 0.0,
            quiver_pivot: String::new(),
            quiver_extra: String::new(),
            // extra options
            // buffer
            buffer: String::new(),
        }
    }

    /// Draws streamlines (stream plot)
    pub fn draw<'a, T, U>(&mut self, xx: &'a T, yy: &'a T, dx: &'a T, dy: &'a T)
    where
        T: AsMatrix<'a, U>,
        U: 'a + std::fmt::Display + Num,
    {
        matrix_to_array(&mut self.buffer, "xx", xx);
        matrix_to_array(&mut self.buffer, "yy", yy);
        matrix_to_array(&mut self.buffer, "dx", dx);
        matrix_to_array(&mut self.buffer, "dy", dy);
        let opt = self.options_streamplot();
        write!(&mut self.buffer, "plt.streamplot(xx,yy,dx,dy{})\n", &opt).unwrap();
    }

    /// Draws arrows (quiver plot)
    pub fn draw_arrows<'a, T, U>(&mut self, xx: &'a T, yy: &'a T, dx: &'a T, dy: &'a T)
    where
        T: AsMatrix<'a, U>,
        U: 'a + std::fmt::Display + Num,
    {
        matrix_to_array(&mut self.buffer, "xx", xx);
        matrix_to_array(&mut self.buffer, "yy", yy);
        matrix_to_array(&mut self.buffer, "dx", dx);
        matrix_to_array(&mut self.buffer, "dy", dy);
        let opt = self.options_quiver();
        write!(&mut self.buffer, "plt.quiver(xx,yy,dx,dy{})\n", &opt).unwrap();
    }

    /// Sets the line color (quiver or streamlines)
    pub fn set_color(&mut self, color: &str) -> &mut Self {
        self.color = String::from(color);
        self
    }

    /// Sets the line width of streamlines
    pub fn set_streamline_linewidth(&mut self, width: f64) -> &mut Self {
        self.streamplot_linewidth = width;
        self
    }

    /// Sets the arrow style
    ///
    /// Options:
    ///
    /// * "`-`"      -- Curve         : None
    /// * "`->`"     -- CurveB        : head_length=0.4,head_width=0.2
    /// * "`-[`"     -- BracketB      : widthB=1.0,lengthB=0.2,angleB=None
    /// * "`-|>`"    -- CurveFilledB  : head_length=0.4,head_width=0.2
    /// * "`<-`"     -- CurveA        : head_length=0.4,head_width=0.2
    /// * "`<->`"    -- CurveAB       : head_length=0.4,head_width=0.2
    /// * "`<|-`"    -- CurveFilledA  : head_length=0.4,head_width=0.2
    /// * "`<|-|>`"  -- CurveFilledAB : head_length=0.4,head_width=0.2
    /// * "`]-`"     -- BracketA      : widthA=1.0,lengthA=0.2,angleA=None
    /// * "`]-[`"    -- BracketAB     : widthA=1.0,lengthA=0.2,angleA=None,widthB=1.0,lengthB=0.2,angleB=None
    /// * "`fancy`"  -- Fancy         : head_length=0.4,head_width=0.4,tail_width=0.4
    /// * "`simple`" -- Simple        : head_length=0.5,head_width=0.5,tail_width=0.2
    /// * "`wedge`"  -- Wedge         : tail_width=0.3,shrink_factor=0.5
    /// * "`|-|`"    -- BarAB         : widthA=1.0,angleA=None,widthB=1.0,angleB=None
    /// * As defined in <https://matplotlib.org/stable/api/_as_gen/matplotlib.patches.FancyArrowPatch.html>
    pub fn set_streamplot_arrow_style(&mut self, style: &str) -> &mut Self {
        self.streamplot_arrow_style = String::from(style);
        self
    }

    /// Sets the density of streamlines
    pub fn set_streamplot_density(&mut self, density: f64) -> &mut Self {
        self.streamplot_density = density;
        self
    }

    /// Sets extra options for streamlines
    ///
    /// See <https://matplotlib.org/stable/api/_as_gen/matplotlib.pyplot.streamplot.html>
    pub fn set_streamplot_extra(&mut self, extra: &str) -> &mut Self {
        self.streamplot_extra = extra.to_string();
        self
    }

    /// Sets the quiver inverse scale
    pub fn set_quiver_inv_scale(&mut self, scale: f64) -> &mut Self {
        self.quiver_scale = scale;
        self
    }

    /// Sets the quiver pivot
    ///
    /// Options: 'tail', 'mid', 'middle', 'tip'
    ///
    /// Default = 'tail'
    pub fn set_quiver_pivot(&mut self, pivot: &str) -> &mut Self {
        self.quiver_pivot = String::from(pivot);
        self
    }

    /// Sets extra options for quiver
    ///
    /// See <https://matplotlib.org/stable/api/_as_gen/matplotlib.pyplot.quiver.html>
    pub fn set_quiver_extra(&mut self, extra: &str) -> &mut Self {
        self.quiver_extra = extra.to_string();
        self
    }

    /// Returns options for streamplot
    fn options_streamplot(&self) -> String {
        let mut opt = String::new();
        if self.color != "" {
            write!(&mut opt, ",color='{}'", self.color).unwrap();
        }
        if self.streamplot_linewidth > 0.0 {
            write!(&mut opt, ",linewidth={}", self.streamplot_linewidth).unwrap();
        }
        if self.streamplot_arrow_style != "" {
            write!(&mut opt, ",arrowstyle='{}'", self.streamplot_arrow_style).unwrap();
        }
        if self.streamplot_density > 0.0 {
            write!(&mut opt, ",density={}", self.streamplot_density).unwrap();
        }
        if self.streamplot_extra != "" {
            write!(&mut opt, ",{}", self.streamplot_extra).unwrap();
        }
        opt
    }

    /// Returns options for quiver
    fn options_quiver(&self) -> String {
        let mut opt = String::new();
        if self.color != "" {
            write!(&mut opt, ",color='{}'", self.color).unwrap();
        }
        if self.quiver_scale > 0.0 {
            write!(&mut opt, ",scale={}", self.quiver_scale).unwrap();
        }
        if self.quiver_pivot != "" {
            write!(&mut opt, ",pivot='{}'", self.quiver_pivot).unwrap();
        }
        if self.quiver_extra != "" {
            write!(&mut opt, ",{}", self.quiver_extra).unwrap();
        }
        opt
    }
}

impl GraphMaker for Stream {
    fn get_buffer<'a>(&'a self) -> &'a String {
        &self.buffer
    }
    fn clear_buffer(&mut self) {
        self.buffer.clear();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {}
