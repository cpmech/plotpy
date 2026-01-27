use super::GraphMaker;

/// Implements a dark mode enabler for plots
///
/// **Warning;** This instance must be the **first** to be added to the `Plot` object,
pub struct DarkMode {
    buffer: String,
}

impl DarkMode {
    /// Allocates a new instance
    ///
    /// **Warning;** This instance must be the **first** to be added to the `Plot` object,
    pub fn new() -> Self {
        let mut dm = DarkMode { buffer: String::new() };
        dm.set_dark_background();
        dm
    }

    /// Sets the Matplotlib native dark mode (dark_background)
    pub fn set_dark_background(&mut self) {
        self.buffer.clear();
        self.buffer.push_str("plt.style.use('dark_background')\n");
    }

    /// Sets the Mathematica-like dark mode
    ///
    /// **Important:** This mode requires `cycler` package in Python environment.
    pub fn set_mathematica(&mut self) {
        self.buffer.clear();
        self.buffer.push_str(
            r#"
########### Setting dark mode: begin ###########

from cycler import cycler

# 1. Background and Text Colors
plt.rcParams.update({
    'figure.facecolor': '#000000',   # Pure black background
    'axes.facecolor': '#000000',     # Pure black plotting area
    'text.color': '#FFFFFF',         # White text
    'axes.labelcolor': '#FFFFFF',    # White axis labels
    'xtick.color': '#FFFFFF',        # White x-axis ticks
    'ytick.color': '#FFFFFF',        # White y-axis ticks
    'axes.edgecolor': '#555555',     # Muted gray spines (Mathematica style)
})

# 2. Mathematica 'Vibrant' Color Cycle
# These hex codes approximate the default Mathematica 10+ plot palette
mathematica_colors = [
    '#5E81B5', # Blue
    '#E19C24', # Orange
    '#8FB032', # Green
    '#EB6238', # Red
    '#9467BD', # Purple
    '#8C564B', # Brown
    '#E377C2'  # Pink
]
plt.rcParams['axes.prop_cycle'] = cycler('color', mathematica_colors)

# 3. Refined Details
plt.rcParams.update({
    'grid.color': '#313244',         # Surface 0 (Subtle grid)
    'legend.facecolor': '#181825',   # Mantle
    'legend.edgecolor': '#313244',
    'legend.labelcolor': '#cdd6f4'
})

########### Setting dark mode: end ###########

"#,
        );
    }

    /// **Important:** This mode requires `cycler` package in Python environment.
    pub fn set_mocha(&mut self) {
        self.buffer.clear();
        self.buffer.push_str(
            r#"
########### Setting dark mode: begin ###########

from cycler import cycler

# 1. Background and Base Colors (Catppuccin Mocha)
plt.rcParams.update({
    'figure.facecolor': '#11111b',   # Crust (Deepest dark)
    'axes.facecolor': '#1e1e2e',     # Base (Slightly lighter for contrast)
    'savefig.facecolor': '#11111b',
    'text.color': '#cdd6f4',         # Text
    'axes.labelcolor': '#cdd6f4',    # Text
    'xtick.color': '#7f849c',        # Overlay 1 (Muted gray)
    'ytick.color': '#7f849c',
    'axes.edgecolor': '#45475a',     # Surface 1
})

# 2. Catppuccin Mocha Palette Color Cycle
# Selecting the most vibrant "flavor" accents
mocha_colors = [
    '#89b4fa', # Blue
    '#fab387', # Peach
    '#a6e3a1', # Green
    '#f38ba8', # Red
    '#cba6f7', # Mauve
    '#94e2d5', # Teal
    '#f9e2af'  # Yellow
]
plt.rcParams['axes.prop_cycle'] = cycler('color', mocha_colors)

# 3. Refined Details
plt.rcParams.update({
    'grid.color': '#313244',         # Surface 0 (Subtle grid)
    'legend.facecolor': '#181825',   # Mantle
    'legend.edgecolor': '#313244',
    'legend.labelcolor': '#cdd6f4'
})

########### Setting dark mode: end ###########

"#,
        );
    }

    /// Sets an alternative dark mode ("Nordic Night" or "Material Dark")
    ///
    /// **Important:** This mode requires `cycler` package in Python environment.
    pub fn set_nordic(&mut self) {
        self.buffer.clear();
        self.buffer.push_str(
            r#"
########### Setting dark mode: begin ###########

from cycler import cycler

# 1. Background and Base Colors
plt.rcParams.update({
    'figure.facecolor': '#2E3440',   # Soft charcoal
    'axes.facecolor': '#2E3440',     # Match axes to figure
    'savefig.facecolor': '#2E3440',  # Ensure saved images are dark
    'text.color': '#D8DEE9',         # Off-white/Silver text
    'axes.labelcolor': '#D8DEE9',
    'xtick.color': '#4C566A',        # Muted gray ticks
    'ytick.color': '#4C566A',
    'axes.edgecolor': '#4C566A',     # Muted borders
})

# 2. Nord Palette Color Cycle (Modern Pastels)
nord_colors = [
    '#88C0D0', # Frost Blue
    '#81A1C1', # Glacial Blue
    '#BF616A', # Soft Red
    '#D08770', # Orange
    '#EBCB8B', # Yellow
    '#A3BE8C', # Sage Green
    '#B48EAD'  # Muted Purple
]
plt.rcParams['axes.prop_cycle'] = cycler('color', nord_colors)

# 3. Refined Details
plt.rcParams.update({
    'grid.color': '#3B4252',       # Darker gray grid lines
    'legend.facecolor': '#181825',   # Mantle
    'legend.edgecolor': '#313244',
    'legend.labelcolor': '#D8DEE9'
})

########### Setting dark mode: end ###########

"#,
        );
    }
}

impl GraphMaker for DarkMode {
    fn get_buffer<'a>(&'a self) -> &'a String {
        &self.buffer
    }
    fn clear_buffer(&mut self) {
        self.buffer.clear();
    }
}
