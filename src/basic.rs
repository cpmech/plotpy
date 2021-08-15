use super::Plot;

impl Plot {
    pub fn scatter(&mut self, x: &[f64], y: &[f64]) {
        let (sx, sy) = self.write_arrays("x", "y", x, y);
        let command = format!("plt.scatter({},{})\n", sx, sy);
        self.buffer.push_str(&command);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plotxy_works() {
        let x = &[1.0, 2.0, 3.0, 4.0, 5.0];
        let y = &[1.0, 4.0, 9.0, 16.0, 25.0];
        let mut plt = Plot::new();
        plt.scatter(x, y);
        let correct ="x_0=np.array([1.000000000000000,2.000000000000000,3.000000000000000,4.000000000000000,5.000000000000000,],dtype=float)
y_119=np.array([1.000000000000000,4.000000000000000,9.000000000000000,16.000000000000000,25.000000000000000,],dtype=float)
plt.scatter(x_0,y_119)
";
        assert_eq!(plt.buffer, correct);
    }
}
