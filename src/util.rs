// Converts an array to a string representing a Python list
pub(crate) fn array2list<T: std::fmt::Display>(values: &[T]) -> String {
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

// Writes array to buffer an returns key = name + uid
pub(crate) fn write_array(buffer: &mut String, name: &str, array: &[f64]) {
    buffer.push_str(&format!("{}=np.array([", name));
    for val in array.iter() {
        let v = format!("{:.15},", val);
        buffer.push_str(&v);
    }
    buffer.push_str("],dtype=float)\n");
}

// Writes arrays to buffer and returns key = name + uid for each array
pub(crate) fn write_arrays(buffer: &mut String, name_x: &str, name_y: &str, array_x: &[f64], array_y: &[f64]) {
    write_array(buffer, name_x, array_x);
    write_array(buffer, name_y, array_y);
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array2list_works() {
        let results = array2list(&[1.0, 2.0, 3.0]);
        assert_eq!(results, "['1','2','3']");
    }

    #[test]
    fn write_array_works() {
        let x = &[1.0, 2.0, 3.0, 4.0, 5.0];
        let mut buffer = String::new();
        write_array(&mut buffer, "x", x);
        assert_eq!(buffer, "x=np.array([1.000000000000000,2.000000000000000,3.000000000000000,4.000000000000000,5.000000000000000,],dtype=float)\n");
    }

    #[test]
    fn write_arrays_works() {
        let x = &[1.0, 2.0, 3.0, 4.0, 5.0];
        let y = &[1.0, 4.0, 9.0, 16.0, 25.0];
        let mut buffer = String::new();
        write_arrays(&mut buffer, "x", "y", x, y);
        assert_eq!(buffer, "x=np.array([1.000000000000000,2.000000000000000,3.000000000000000,4.000000000000000,5.000000000000000,],dtype=float)\ny=np.array([1.000000000000000,4.000000000000000,9.000000000000000,16.000000000000000,25.000000000000000,],dtype=float)\n");
    }
}
