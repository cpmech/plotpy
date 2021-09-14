use core::borrow::Borrow;
use std::fmt::Write;

/// Converts vector to a Python list of strings
///
/// The vector may be one of the following
///
/// * `Vec<U>` -- a contiguous growable array type
/// * `[U; n]` -- a fixed-size array
/// * `&[U]` -- a slice with size only known at runtime
///
pub(crate) fn vector_to_strings<'a, T, U>(buf: &mut String, name: &str, vector: &'a T)
where
    T: ?Sized,
    &'a T: IntoIterator<Item = U>,
    U: std::fmt::Display,
{
    write!(buf, "{}=[", name).unwrap();
    for val in vector.into_iter() {
        write!(buf, "'{}',", val).unwrap();
    }
    write!(buf, "]\n").unwrap();
}

/// Converts vector to a 1D NumPy array
///
/// The vector may be one of the following
///
/// * `Vec<U>` -- a contiguous growable array type
/// * `[U; n]` -- a fixed-size array
/// * `&[U]` -- a slice with size only known at runtime
///
pub(crate) fn vector_to_array<'a, T, U>(buf: &mut String, name: &str, vector: &'a T)
where
    T: ?Sized,
    &'a T: IntoIterator<Item = U>,
    U: std::fmt::Display,
{
    write!(buf, "{}=np.array([", name).unwrap();
    for val in vector.into_iter() {
        write!(buf, "{},", val).unwrap();
    }
    write!(buf, "],dtype=float)\n").unwrap();
}

/// Converts a matrix to a 2D NumPy array
///
/// The matrix may be one of the following
///
/// * `Vec<Vec<U>>` -- a contiguous growable array of array
/// * `[[U; m]; n]` -- a nested fixed-size array of array
/// * `&[&[U]]` -- a nested slice of references with size only known at runtime
///
pub(crate) fn matrix_to_array<'a, T, U, V>(buf: &mut String, name: &str, matrix: &'a T)
where
    T: ?Sized,
    &'a T: std::iter::IntoIterator<Item = &'a U>,
    U: 'a + Borrow<[V]>,
    V: std::fmt::Display,
{
    write!(buf, "{}=np.array([", name).unwrap();
    for row in matrix.into_iter() {
        write!(buf, "[").unwrap();
        for val in row.borrow().into_iter() {
            write!(buf, "{},", val).unwrap();
        }
        write!(buf, "],").unwrap();
    }
    write!(buf, "],dtype=float)\n").unwrap();
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_to_strings_works() {
        let mut buf = String::new();
        let x: Vec<&str> = vec!["red", "green", "blue"];
        let y: [String; 3] = ["cyan".to_string(), "magenta".to_string(), "white".to_string()];
        let z: &[&str] = &["#f00", "#0f0", "#00f"];
        vector_to_strings(&mut buf, "x", &x);
        vector_to_strings(&mut buf, "y", &y);
        vector_to_strings(&mut buf, "z", z);
        assert_eq!(
            buf,
            "x=['red','green','blue',]\n\
             y=['cyan','magenta','white',]\n\
             z=['#f00','#0f0','#00f',]\n"
        );
    }

    #[test]
    fn vector_to_array_works() {
        let mut buf = String::new();
        let x: Vec<f64> = vec![0.1, 0.2, 0.3];
        let y: [f64; 3] = [1.0, 2.0, 3.0];
        let z: &[f64] = &[10.0, 20.0, 30.0];
        vector_to_array(&mut buf, "x", &x);
        vector_to_array(&mut buf, "y", &y);
        vector_to_array(&mut buf, "z", z);
        assert_eq!(
            buf,
            "x=np.array([0.1,0.2,0.3,],dtype=float)\n\
             y=np.array([1,2,3,],dtype=float)\n\
             z=np.array([10,20,30,],dtype=float)\n"
        );
    }

    #[test]
    fn matrix_to_list_works() {
        let mut buf = String::new();
        let a: Vec<Vec<f64>> = vec![vec![0.1, 0.2, 0.3], vec![0.4, 0.5, 0.6], vec![0.7, 0.8, 0.9]];
        let b: [[f64; 3]; 3] = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
        let c: &[&[f64]] = &[&[10.0, 20.0, 30.0], &[40.0, 50.0, 60.0], &[70.0, 80.0, 90.0]];
        matrix_to_array(&mut buf, "a", &a);
        matrix_to_array(&mut buf, "b", &b);
        matrix_to_array(&mut buf, "c", c);
        assert_eq!(
            buf,
            "a=np.array([[0.1,0.2,0.3,],[0.4,0.5,0.6,],[0.7,0.8,0.9,],],dtype=float)\n\
             b=np.array([[1,2,3,],[4,5,6,],[7,8,9,],],dtype=float)\n\
             c=np.array([[10,20,30,],[40,50,60,],[70,80,90,],],dtype=float)\n"
        );
    }
}
