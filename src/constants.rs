// The code in `set_equal_axes` is based on:
// https://stackoverflow.com/questions/13685386/matplotlib-equal-unit-length-with-equal-aspect-ratio-z-axis-is-not-equal-to
//
// It needs Matplotlib version at least 3.3.0 (Jul 16, 2020)
// https://github.com/matplotlib/matplotlib/blob/f6e0ee49c598f59c6e6cf4eefe473e4dc634a58a/doc/users/prev_whats_new/whats_new_3.3.0.rst

/// Commands to be added at the beginning of the Python script
///
/// The python definitions are:
///
/// * `NaN` -- Variable to handle NaN values coming from Rust
/// * `EXTRA_ARTISTS` -- List of additional objects that must not be ignored when saving the figure
/// * `add_to_ea` -- Adds an entity to the EXTRA_ARTISTS list to prevent them being ignored
///    when Matplotlib decides to calculate the bounding boxes. The Legend is an example of entity that could
///    be ignored by the savefig command (this is issue is prevented here).
/// * `THREE_D` -- Is a dictionary of mplot3d objects (one for each subplot3d)
/// * `THREE_D_ACTIVE` -- Is a tuple holding the key to the current THREE_D object (defines the subplot3d)
/// * `ax3d` -- Creates or returns the mplot3d object with the current subplot3d definition specified by THREE_D_ACTIVE
/// * `subplot3d` -- Specifies the THREE_D_ACTIVE parameters to define a subplot3d
/// * `data_to_axis` -- Transforms data limits to axis limits
/// * `axis_to_data` -- Transforms axis limits to data limits
/// * `set_equal_axes` -- Configures the aspect of axes with a same scaling from data to plot units for x, y and z.
///   For example a circle will show as a circle in the screen and not an ellipse. This function also handles
///   the 3D case which is a little tricky with Matplotlib. In this case (3D), the version of Matplotlib
///   must be greater than 3.3.0.
/// * `set_axis_label` -- Sets the label of the axis along the dimension 'dim'
pub const PYTHON_HEADER: &str = "### file generated by the 'plotpy' Rust crate

import numpy as np
import matplotlib.pyplot as plt
import matplotlib.ticker as tck
import matplotlib.patches as pat
import matplotlib.path as pth
import matplotlib.patheffects as pff
import matplotlib.lines as lns
import matplotlib.transforms as tra
import mpl_toolkits.mplot3d

# Variable to handle NaN values coming from Rust
NaN = np.NaN

# List of additional objects to calculate bounding boxes
EXTRA_ARTISTS = []

# Adds an entity to the EXTRA_ARTISTS list to calculate bounding boxes
def add_to_ea(obj):
    if obj!=None: EXTRA_ARTISTS.append(obj)

# Is a dictionary of mplot3d objects (one for each subplot3d)
THREE_D = dict()

# Is a tuple holding the key to the current THREE_D object (defines the subplot3d)
THREE_D_ACTIVE = (1,1,1)

# Creates or returns the mplot3d object with the current subplot3d definition specified by THREE_D_ACTIVE
def ax3d():
    global THREE_D
    global THREE_D_ACTIVE
    if not THREE_D_ACTIVE in THREE_D:
        a, b, c = THREE_D_ACTIVE
        THREE_D[THREE_D_ACTIVE] = plt.gcf().add_subplot(a,b,c,projection='3d')
        THREE_D[THREE_D_ACTIVE].set_xlabel('x')
        THREE_D[THREE_D_ACTIVE].set_ylabel('y')
        THREE_D[THREE_D_ACTIVE].set_zlabel('z')
        add_to_ea(THREE_D[THREE_D_ACTIVE])
    return THREE_D[THREE_D_ACTIVE]

# Specifies the THREE_D_ACTIVE parameters to define a subplot3d
def subplot3d(a,b,c):
    global THREE_D_ACTIVE
    THREE_D_ACTIVE = (a,b,c)
    ax3d()

# Transforms data limits to axis limits
def data_to_axis(coords):
    plt.axis() # must call this first
    return plt.gca().transLimits.transform(coords)

# Transforms axis limits to data limits
def axis_to_data(coords):
    plt.axis() # must call this first
    return plt.gca().transLimits.inverted().transform(coords)

# Configures the aspect of axes with a same scaling from data to plot units for x, y and z.
def set_equal_axes():
    global THREE_D
    if len(THREE_D) == 0:
        ax = plt.gca()
        ax.axes.set_aspect('equal')
        return
    try:
        ax = ax3d()
        ax.set_box_aspect([1,1,1])
        limits = np.array([ax.get_xlim3d(), ax.get_ylim3d(), ax.get_zlim3d()])
        origin = np.mean(limits, axis=1)
        radius = 0.5 * np.max(np.abs(limits[:, 1] - limits[:, 0]))
        x, y, z = origin
        ax.set_xlim3d([x - radius, x + radius])
        ax.set_ylim3d([y - radius, y + radius])
        ax.set_zlim3d([z - radius, z + radius])
    except:
        import matplotlib
        print('VERSION of MATPLOTLIB = {}'.format(matplotlib.__version__))
        print('ERROR: set_box_aspect is missing in this version of Matplotlib')

# Sets the label of the axis along the dimension 'dim'
def set_axis_label(dim, label):
    global THREE_D
    if len(THREE_D) == 0:
        if dim == 1: plt.gca().set_xlabel(label)
        if dim == 2: plt.gca().set_ylabel(label)
    else:
        if dim == 1: ax3d().set_xlabel(label)
        if dim == 2: ax3d().set_ylabel(label)
        if dim == 3: ax3d().set_zlabel(label)

################## plotting commands follow after this line ############################

";

const PY_NUM_MARKERS: [&str; 12] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11"];

/// Quotes or not the marker style
///
/// This is needed because the following markers are python numbers,
/// and not strings (so, they must not be quoted):
///
/// ```text
/// 0 (TICKLEFT)
/// 1 (TICKRIGHT)
/// 2 (TICKUP)
/// 3 (TICKDOWN)
/// 4 (CARETLEFT)
/// 5 (CARETRIGHT)
/// 6 (CARETUP)
/// 7 (CARETDOWN)
/// 8 (CARETLEFTBASE)
/// 9 (CARETRIGHTBASE)
/// 10 (CARETUPBASE)
/// 11 (CARETDOWNBASE)
/// ```
///
/// See: <https://matplotlib.org/stable/api/markers_api.html>
pub(crate) fn quote_marker(maker_style: &str) -> String {
    if PY_NUM_MARKERS.contains(&maker_style) {
        String::from(maker_style)
    } else {
        format!("'{}'", maker_style)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::PYTHON_HEADER;

    #[test]
    fn constants_are_correct() {
        assert_eq!(PYTHON_HEADER.len(), 3119);
    }
}
