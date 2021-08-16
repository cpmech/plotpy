/// Many options that can be passed to Matplotlib
pub struct Arguments {
    // lines
    pub line_alpha: f64,    // alpha (0, 1]. A<1e-14 => A=1.0
    pub line_color: String, // color
    pub line_style: String, // style
    pub line_width: f64,    // width

    // markers
    pub marker_alpha: f64,         // alpha (0, 1]
    pub marker_color: String,      // color
    pub marker_every: i32,         // mark-every
    pub marker_is_void: bool,      // void marker (draw edge only)
    pub marker_line_color: String, // edge color
    pub marker_line_style: String, // edge style
    pub marker_line_width: f64,    // edge width
    pub marker_size: f64,          // size
    pub marker_type: String,       // type, e.g., "o", "+"

    // shapes
    pub shape_edge_color: String, // edge color
    pub shape_face_color: String, // face color
    pub shape_is_closed: bool,    // closed shape
    pub shape_scale: f64,         // scale
    pub shape_style: String,      // style

    // text
    pub text_alignment_horizontal: String, // e.g., 'center'
    pub text_alignment_vertical: String,   // e.g., 'center'
    pub text_rotation: f64,                // text rotation
    pub text_font_size: f64,               // font size

    // legend
    pub legend_show_frame: bool,      // show frame around legend
    pub legend_length_indicator: f64, // length of legend's indicator line
    pub legend_location: String,      // e.g., "right", "center left"
    pub legend_number_columns: i32,   // number of columns
    pub legend_coordinates: Vec<f64>, // normalized coordinates to put legend outsize
    pub legend_is_outside: bool,      // put legend outside

    // contour
    pub contour_colors: Vec<String>,     // colors
    pub contour_levels: Vec<f64>,        // levels (may be nil)
    pub contour_colormap_index: i32,     // colormap index
    pub contour_number_format: String,   // number format
    pub contour_no_lines: bool,          // no lines on top of filled contour
    pub contour_no_labels: bool,         // no labels
    pub contour_no_inline: bool,         // no labels 'inline'
    pub contour_no_colorbar: bool,       // no colorbar
    pub contour_colorbar_label: String,  // colorbar label
    pub contour_selected_value: f64,     // selected value
    pub contour_selected_color: String,  // color to mark selected level
    pub contour_selected_linewidth: f64, // zero level linewidth

    // histograms
    pub histogram_colors: Vec<String>, // colors
    pub histogram_type: String,        // type; e.g. "bar"
    pub histogram_stacked: bool,       // stacked
    pub histogram_no_fill: bool,       // do not fill bars
    pub histogram_number_bins: i32,    // number of bins
    pub histogram_normalized: bool,    // normed

    // 3d graphs
    pub d3_row_stride: i32, // row stride
    pub d3_col_stride: i32, // column stride
    pub d3_surface: bool,   // generate surface
    pub d3_wireframe: bool, // generate wireframe
}

impl Arguments {
    pub fn new() -> Self {
        Arguments {
            // lines
            line_alpha: 0.0,
            line_color: String::new(),
            line_style: String::new(),
            line_width: 0.0,

            // markers
            marker_alpha: 0.0,
            marker_color: String::new(),
            marker_every: 0,
            marker_is_void: false,
            marker_line_color: String::new(),
            marker_line_style: String::new(),
            marker_line_width: 0.0,
            marker_size: 0.0,
            marker_type: String::new(),

            // shapes
            shape_edge_color: String::new(),
            shape_face_color: String::new(),
            shape_is_closed: false,
            shape_scale: 0.0,
            shape_style: String::new(),

            // text
            text_alignment_horizontal: String::new(),
            text_alignment_vertical: String::new(),
            text_rotation: 0.0,
            text_font_size: 0.0,

            // legend
            legend_show_frame: true,
            legend_length_indicator: 3.0,
            legend_location: "best".to_string(),
            legend_number_columns: 1,
            legend_coordinates: vec![0.0, 1.02, 1.0, 0.102],
            legend_is_outside: false,

            // contour
            contour_colors: Vec::new(),
            contour_levels: Vec::new(),
            contour_colormap_index: 0,
            contour_number_format: String::new(),
            contour_no_lines: false,
            contour_no_labels: false,
            contour_no_inline: false,
            contour_no_colorbar: false,
            contour_colorbar_label: String::new(),
            contour_selected_value: 0.0,
            contour_selected_color: String::new(),
            contour_selected_linewidth: 0.0,

            // histograms
            histogram_colors: Vec::new(),
            histogram_type: String::new(),
            histogram_stacked: false,
            histogram_no_fill: false,
            histogram_number_bins: 0,
            histogram_normalized: false,

            // 3d graphs
            d3_row_stride: 0,
            d3_col_stride: 0,
            d3_surface: false,
            d3_wireframe: false,
        }
    }

    pub(crate) fn to_string(&self, for_3d_points: bool) -> String {
        // fix color if marker is void
        let line_color = if self.marker_is_void && self.line_color == "" {
            "red"
        } else if self.marker_is_void && for_3d_points {
            "none"
        } else {
            &self.line_color
        };

        // output
        let mut args = String::new();

        // lines
        if self.line_alpha > 0.0 {
            args.push_str(&format!(",alpha={}", self.line_alpha));
        }
        if line_color != "" {
            args.push_str(&format!(",color='{}'", line_color));
        }
        if self.line_style != "" {
            args.push_str(&format!(",linestyle='{}'", self.line_style));
        }
        if self.line_width > 0.0 {
            args.push_str(&format!(",linewidth={}", self.line_width));
        }

        // markers
        if self.marker_alpha > 0.0 {
            args.push_str(&format!(",markeralpha={}", self.marker_alpha));
        }
        if self.marker_color != "" {
            args.push_str(&format!(",markerfacecolor='{}'", self.marker_color));
        }
        if self.marker_every > 0 {
            args.push_str(&format!(",markevery={}", self.marker_every));
        }
        if self.marker_is_void {
            args.push_str(",markerfacecolor='none'");
        }
        if self.marker_line_color != "" {
            args.push_str(&format!(",markeredgecolor='{}'", self.marker_line_color));
        }
        if self.marker_line_style != "" {
            args.push_str(&format!(",markerlinestyle='{}'", self.marker_line_style));
        }
        if self.marker_line_width > 0.0 {
            args.push_str(&format!(",markeredgewidth={}", self.marker_line_width));
        }
        if self.marker_size > 0.0 {
            args.push_str(&format!(",markersize={}", self.marker_size));
        }
        if self.marker_type != "" {
            args.push_str(&format!(",marker='{}'", self.marker_type));
        }

        // shapes
        if self.shape_edge_color != "" {
            args.push_str(&format!(",edgecolor='{}'", self.shape_edge_color));
        }
        if self.shape_face_color != "" {
            args.push_str(&format!(",facecolor='{}'", self.shape_face_color));
        }

        // text
        if self.text_alignment_horizontal != "" {
            args.push_str(&format!(",ha='{}'", self.text_alignment_horizontal));
        }
        if self.text_alignment_vertical != "" {
            args.push_str(&format!(",va='{}'", self.text_alignment_vertical));
        }
        if self.text_rotation > 0.0 {
            args.push_str(&format!(",rotation={}", self.text_rotation));
        }
        if self.text_font_size > 0.0 {
            args.push_str(&format!(",fontsize={}", self.text_font_size));
        }

        // contour
        if self.contour_colors.len() > 0 {
            args.push_str(&format!(",colors={}", array2list(&self.contour_colors)));
        }
        if self.contour_levels.len() > 0 {
            args.push_str(&format!(",levels={}", array2list(&self.contour_levels)));
        }

        // histograms
        if self.histogram_colors.len() > 0 {
            args.push_str(&format!(",color={}", array2list(&self.histogram_colors)));
        }
        if self.histogram_type != "" {
            args.push_str(&format!(",histtype='{}'", self.histogram_type));
        }
        if self.histogram_stacked {
            args.push_str(",stacked=True");
        }
        if self.histogram_no_fill {
            args.push_str(",fill=False");
        }
        if self.histogram_number_bins > 0 {
            args.push_str(&format!(",bins={}", self.histogram_number_bins));
        }
        if self.histogram_normalized {
            args.push_str(",normed=True");
        }

        // 3d graphs
        if self.d3_row_stride > 0 {
            args.push_str(&format!(",rstride={}", self.d3_row_stride));
        }
        if self.d3_col_stride > 0 {
            args.push_str(&format!(",cstride={}", self.d3_col_stride));
        }

        // done
        args
    }
}

// Converts an array to a string representing a Python list
fn array2list<T: std::fmt::Display>(values: &[T]) -> String {
    let mut result = "[".to_string();
    let mut first = true;
    for val in values.iter() {
        if !first {
            result.push_str(",");
        }
        result.push_str(&format!("'{}'", val));
        first = false;
    }
    result.push_str("]");
    result
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_args_works() {
        let style = Arguments::new();
        assert_eq!(style.line_color, "");
    }

    #[test]
    fn to_string_works() {
        let mut style = Arguments::new();
        style.line_alpha = 0.7;
        style.line_color = "#b33434".to_string();
        style.line_style = "-".to_string();
        style.line_width = 3.0;
        style.marker_alpha = 0.5;
        style.marker_color = "#4c4deb".to_string();
        style.marker_every = 2;
        style.marker_is_void = false;
        style.marker_line_color = "blue".to_string();
        style.marker_line_style = "--".to_string();
        style.marker_line_width = 1.5;
        style.marker_size = 8.0;
        style.marker_type = "o".to_string();
        let args = style.to_string(false);
        assert_eq!(
            args,
            "\
            ,alpha=0.7\
            ,color='#b33434'\
            ,linestyle='-'\
            ,linewidth=3\
            ,markeralpha=0.5\
            ,markerfacecolor='#4c4deb'\
            ,markevery=2\
            ,markeredgecolor='blue'\
            ,markerlinestyle='--'\
            ,markeredgewidth=1.5\
            ,markersize=8\
            ,marker='o'\
            "
        );
    }
}
