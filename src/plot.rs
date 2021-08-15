pub struct Plot {
    buffer: String,
}

impl Plot {
    pub fn new() -> Self {
        Plot {
            buffer: String::new(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_plot_works() {
        let plt = Plot::new();
        assert_eq!(plt.buffer.len(), 0);
    }
}
