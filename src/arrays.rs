use super::Plot;

impl Plot {
    pub(crate) fn write_array(&mut self, name: &str, array: &[f64]) {
        self.buffer.push_str(name);
        self.buffer.push_str(&self.uid());
        self.buffer.push_str("=np.array([");
        for val in array.iter() {
            let v = format!("{:.15},", val);
            self.buffer.push_str(&v);
        }
        self.buffer.push_str("],dtype=float)\n");
    }

    pub(crate) fn write_arrays(
        &mut self,
        name_x: &str,
        name_y: &str,
        array_x: &[f64],
        array_y: &[f64],
    ) {
        self.write_array(name_x, array_x);
        self.write_array(name_y, array_y);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_array_works() {
        let x = &[1.0, 2.0, 3.0, 4.0, 5.0];
        let mut plt = Plot::new();
        plt.write_array("x", x);
        assert_eq!(plt.buffer, "x1=np.array([1.000000000000000,2.000000000000000,3.000000000000000,4.000000000000000,5.000000000000000,],dtype=float)\n");
    }

    #[test]
    fn write_arrays_works() {
        let x = &[1.0, 2.0, 3.0, 4.0, 5.0];
        let y = &[1.0, 4.0, 9.0, 16.0, 25.0];
        let mut plt = Plot::new();
        plt.write_arrays("x", "y", x, y);
        assert_eq!(plt.buffer, "x1=np.array([1.000000000000000,2.000000000000000,3.000000000000000,4.000000000000000,5.000000000000000,],dtype=float)\ny119=np.array([1.000000000000000,4.000000000000000,9.000000000000000,16.000000000000000,25.000000000000000,],dtype=float)\n");
    }
}
