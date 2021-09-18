use super::*;
use std::fmt::Write;

/// Converts vector to a Python list of numbers
pub(crate) fn vector_to_numbers<T>(buf: &mut String, name: &str, vector: &[T])
where
    T: std::fmt::Display,
{
    write!(buf, "{}=[", name).unwrap();
    for val in vector.into_iter() {
        write!(buf, "{},", val).unwrap();
    }
    write!(buf, "]\n").unwrap();
}

/// Converts vector to a Python list of strings
pub(crate) fn vector_to_strings<T>(buf: &mut String, name: &str, vector: &[T])
where
    T: std::fmt::Display,
{
    write!(buf, "{}=[", name).unwrap();
    for val in vector.into_iter() {
        write!(buf, "'{}',", val).unwrap();
    }
    write!(buf, "]\n").unwrap();
}

/// Converts vector to a 1D NumPy array
pub(crate) fn vector_to_array<'a, T, U>(buf: &mut String, name: &str, vector: &'a T)
where
    T: AsVector<'a, U>,
    U: 'a + std::fmt::Display,
{
    write!(buf, "{}=np.array([", name).unwrap();
    let m = vector.vec_size();
    for i in 0..m {
        write!(buf, "{},", vector.vec_at(i)).unwrap();
    }
    write!(buf, "],dtype=float)\n").unwrap();
}

/// Converts a matrix to a nested Python list
pub(crate) fn matrix_to_list<T>(buf: &mut String, name: &str, matrix: &Vec<Vec<T>>)
where
    T: std::fmt::Display,
{
    write!(buf, "{}=[", name).unwrap();
    for row in matrix.into_iter() {
        write!(buf, "[").unwrap();
        for val in row.into_iter() {
            write!(buf, "{},", val).unwrap();
        }
        write!(buf, "],").unwrap();
    }
    write!(buf, "]\n").unwrap();
}

/// Converts a matrix to a 2D NumPy array
pub(crate) fn matrix_to_array<'a, T, U>(buf: &mut String, name: &str, matrix: &'a T)
where
    T: AsMatrix<'a, U>,
    U: 'a + std::fmt::Display,
{
    write!(buf, "{}=np.array([", name).unwrap();
    let (m, n) = matrix.mat_size();
    for i in 0..m {
        write!(buf, "[").unwrap();
        for j in 0..n {
            write!(buf, "{},", matrix.mat_at(i, j)).unwrap();
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
    fn vector_to_numbers_works() {
        let mut buf = String::new();
        let x: Vec<f64> = vec![0.1, 0.2, 0.3];
        let y: [f64; 3] = [1.0, 2.0, 3.0];
        let z: &[f64] = &[10.0, 20.0, 30.0];
        vector_to_numbers(&mut buf, "x", &x);
        vector_to_numbers(&mut buf, "y", &y);
        vector_to_numbers(&mut buf, "z", z);
        assert_eq!(
            buf,
            "x=[0.1,0.2,0.3,]\n\
             y=[1,2,3,]\n\
             z=[10,20,30,]\n"
        );
    }

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
        vector_to_array(&mut buf, "z", &z);
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
        let a = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0], vec![6.0, 7.0, 8.0, 9.0]];
        matrix_to_list(&mut buf, "a", &a);
        assert_eq!(buf, "a=[[1,2,3,],[4,5,],[6,7,8,9,],]\n");
    }

    #[test]
    fn matrix_to_array_works() {
        let mut buf = String::new();
        let a: Vec<Vec<f64>> = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0], vec![7.0, 8.0, 9.0]];
        let b: [[f64; 3]; 3] = [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]];
        let c: &[&[f64]] = &[&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0], &[7.0, 8.0, 9.0]];
        matrix_to_array(&mut buf, "a", &a);
        matrix_to_array(&mut buf, "b", &b);
        matrix_to_array(&mut buf, "c", &c);
        assert_eq!(
            buf,
            "a=np.array([[1,2,3,],[4,5,6,],[7,8,9,],],dtype=float)\n\
             b=np.array([[1,2,3,],[4,5,6,],[7,8,9,],],dtype=float)\n\
             c=np.array([[1,2,3,],[4,5,6,],[7,8,9,],],dtype=float)\n"
        );
    }
}
