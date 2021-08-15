use super::Plot;

impl Plot {
    /// Generates unique ID = key + "_" + buffer.len()
    pub(crate) fn generate_uid(&self, key: &str) -> String {
        format!("{}_{}", key, self.buffer.len())
    }

    // Writes array to buffer an returns key = name + uid
    pub(crate) fn write_array(&mut self, name: &str, array: &[f64]) -> String {
        let uid = self.generate_uid(name);
        self.buffer.push_str(&uid);
        self.buffer.push_str("=np.array([");
        for val in array.iter() {
            let v = format!("{:.15},", val);
            self.buffer.push_str(&v);
        }
        self.buffer.push_str("],dtype=float)\n");
        uid
    }

    // Writes arrays to buffer and returns key = name + uid for each array
    pub(crate) fn write_arrays(
        &mut self,
        name_x: &str,
        name_y: &str,
        array_x: &[f64],
        array_y: &[f64],
    ) -> (String, String) {
        let uid_x = self.write_array(name_x, array_x);
        let uid_y = self.write_array(name_y, array_y);
        (uid_x, uid_y)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_uid_works() {
        let plt = Plot::new();
        assert_eq!(plt.generate_uid("x"), "x_0");
    }

    #[test]
    fn write_array_works() {
        let x = &[1.0, 2.0, 3.0, 4.0, 5.0];
        let mut plt = Plot::new();
        let uid = plt.write_array("x", x);
        assert_eq!(uid, "x_0");
        assert_eq!(plt.buffer, "x_0=np.array([1.000000000000000,2.000000000000000,3.000000000000000,4.000000000000000,5.000000000000000,],dtype=float)\n");
    }

    #[test]
    fn write_arrays_works() {
        let x = &[1.0, 2.0, 3.0, 4.0, 5.0];
        let y = &[1.0, 4.0, 9.0, 16.0, 25.0];
        let mut plt = Plot::new();
        let (uid_x, uid_y) = plt.write_arrays("x", "y", x, y);
        assert_eq!(uid_x, "x_0");
        assert_eq!(uid_y, "y_119");
        assert_eq!(plt.buffer, "x_0=np.array([1.000000000000000,2.000000000000000,3.000000000000000,4.000000000000000,5.000000000000000,],dtype=float)\ny_119=np.array([1.000000000000000,4.000000000000000,9.000000000000000,16.000000000000000,25.000000000000000,],dtype=float)\n");
    }
}
