use std::fmt::Write;

// Converts a vector to a Python list of numbers
pub(crate) fn vec_to_py_list_num<T>(buf: &mut String, name: &str, values: &[T])
where
    T: std::fmt::Display,
{
    write!(buf, "{}=[", name).unwrap();
    for val in values.iter() {
        write!(buf, "{},", val).unwrap();
    }
    write!(buf, "]\n").unwrap();
}

// Converts a vector to a Python list of strings
pub(crate) fn vec_to_py_list_str<T>(buf: &mut String, name: &str, values: &[T])
where
    T: std::fmt::Display,
{
    write!(buf, "{}=[", name).unwrap();
    for val in values.iter() {
        write!(buf, "'{}',", val).unwrap();
    }
    write!(buf, "]\n").unwrap();
}

// Writes a vector of vector as Python nested list
pub(crate) fn vec_vec_to_py_list_num<T>(buf: &mut String, name: &str, data: &[&[T]])
where
    T: std::fmt::Display,
{
    write!(buf, "{}=[", name).unwrap();
    for row in data.iter() {
        write!(buf, "[").unwrap();
        for val in row.iter() {
            write!(buf, "{},", val).unwrap();
        }
        write!(buf, "],").unwrap();
    }
    write!(buf, "]\n").unwrap();
}

// Writes a vector as Numpy array to buffer
pub(crate) fn vec_to_numpy_array<T>(buf: &mut String, name: &str, vec: &[T])
where
    T: std::fmt::Display,
{
    write!(buf, "{}=np.array([", name).unwrap();
    for val in vec.iter() {
        write!(buf, "{},", val).unwrap();
    }
    write!(buf, "],dtype=float)\n").unwrap();
}

// Writes a vector of vector as Numpy 2D array to buffer
pub(crate) fn vec_vec_to_numpy_array_2d<T>(buf: &mut String, name: &str, vec_vec: &[&[T]]) -> Result<(), &'static str>
where
    T: std::fmt::Display,
{
    let mut ncol = 0_usize;
    let mut first = true;
    write!(buf, "{}=np.array([", name).unwrap();
    for row in vec_vec.iter() {
        if !first && row.len() != ncol {
            write!(buf, "],dtype=float)\n").unwrap();
            return Err("all rows must have the same number of columns");
        }
        if first {
            ncol = row.len();
            first = false;
            if ncol == 0 {
                write!(buf, "],dtype=float)\n").unwrap();
                return Err("the matrix must have one column at least");
            }
        }
        write!(buf, "[").unwrap();
        for val in row.iter() {
            write!(buf, "{},", val).unwrap();
        }
        write!(buf, "],").unwrap();
    }
    write!(buf, "],dtype=float)\n").unwrap();
    Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_to_py_list_num_works() {
        let mut buf = String::new();
        vec_to_py_list_num(&mut buf, "x", &[1.0, 2.0, 3.0]);
        assert_eq!(buf, "x=[1,2,3,]\n");
    }

    #[test]
    fn vec_to_py_list_str_works() {
        let mut buf = String::new();
        vec_to_py_list_str(&mut buf, "x", &[1.0, 2.0, 3.0]);
        assert_eq!(buf, "x=['1','2','3',]\n");
    }

    #[test]
    fn vec_vec_to_py_list_works() {
        let mut buf = String::new();
        vec_vec_to_py_list_num(&mut buf, "x", &[&[1.0, 2.0, 3.0], &[4.0], &[5.0, 6.0]]);
        assert_eq!(buf, "x=[[1,2,3,],[4,],[5,6,],]\n");
    }

    #[test]
    fn vec_to_numpy_array_works() {
        let x = &[1.0, 2.0, 3.0, 4.0, 5.0];
        let mut buf = String::new();
        vec_to_numpy_array(&mut buf, "x", x);
        assert_eq!(buf, "x=np.array([1,2,3,4,5,],dtype=float)\n");
    }

    #[test]
    fn vec_vec_to_numpy_array_2d_fails_on_wrong_ncol() -> Result<(), &'static str> {
        let a: &[&[f64]] = &[&[], &[2.0], &[3.0]];
        let mut buf = String::new();
        assert_eq!(
            vec_vec_to_numpy_array_2d(&mut buf, "a", a).err(),
            Some("the matrix must have one column at least")
        );
        assert_eq!(buf, "a=np.array([],dtype=float)\n");
        Ok(())
    }

    #[test]
    fn vec_vec_to_numpy_array_2d_fails_on_wrong_mat() -> Result<(), &'static str> {
        let a: &[&[f64]] = &[&[1.0, 2.0, 3.0], &[4.0, 6.0], &[7.0, 8.0, 9.0]];
        let mut buf = String::new();
        assert_eq!(
            vec_vec_to_numpy_array_2d(&mut buf, "a", a).err(),
            Some("all rows must have the same number of columns")
        );
        assert_eq!(buf, "a=np.array([[1,2,3,],],dtype=float)\n");
        Ok(())
    }

    #[test]
    fn vec_vec_to_numpy_array_2d_works() -> Result<(), &'static str> {
        let a: &[&[f64]] = &[&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0], &[7.0, 8.0, 9.0]];
        let mut buf = String::new();
        vec_vec_to_numpy_array_2d(&mut buf, "a", a)?;
        assert_eq!(buf, "a=np.array([[1,2,3,],[4,5,6,],[7,8,9,],],dtype=float)\n");
        Ok(())
    }
}
