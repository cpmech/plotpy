use russell_lab::Matrix;

/// Defines a trait to handle Matrix-like data
///
/// # Example
///
/// ```
/// use plotpy::AsMatrix;
/// use russell_lab::Matrix;
///
/// fn sum<'a, T, U>(array: &'a T) -> f64
/// where
///     T: AsMatrix<'a, U>,
///     U: 'a + Into<f64>,
/// {
///     let mut res = 0.0;
///     let (m, n) = array.size();
///     for i in 0..m {
///         for j in 0..n {
///             res += array.at(i, j).into();
///         }
///     }
///     res
/// }
///
/// // heap-allocated 2D array (vector of vectors)
/// const IGNORED: f64 = 123.456;
/// let a = vec![
///     vec![1.0, 2.0],
///     vec![3.0, 4.0, IGNORED, IGNORED, IGNORED],
///     vec![5.0, 6.0],
/// ];
/// assert_eq!(sum(&a), 21.0);
///
/// // heap-allocated 2D array (aka slice of slices)
/// let b: &[&[f64]] = &[
///     &[10.0, 20.0],
///     &[30.0, 40.0, IGNORED],
///     &[50.0, 60.0, IGNORED, IGNORED],
/// ];
/// assert_eq!(sum(&b), 210.0);
///
/// // stack-allocated (fixed-size) 2D array
/// let c = [
///     [100.0, 200.0],
///     [300.0, 400.0],
///     [500.0, 600.0],
/// ];
/// assert_eq!(sum(&c), 2100.0);
///
/// // using Matrix
/// let d = Matrix::from(&[
///     [0.1, 0.2],
///     [0.3, 0.4],
///     [0.5, 0.6],
/// ]);
/// assert_eq!(sum(&d), 2.1);
/// ```
pub trait AsMatrix<'a, U: 'a> {
    /// Returns the size of the matrix
    fn size(&self) -> (usize, usize);

    /// Returns the value at index i
    fn at(&self, i: usize, j: usize) -> U;
}

/// Defines a heap-allocated 2D array (vector of vectors)
///
/// # Notes
///
/// * The number of columns is defined by the first row
/// * The next rows must have at least the same number of columns as the first row
impl<'a, U: 'a> AsMatrix<'a, U> for Vec<Vec<U>>
where
    U: 'a + Copy,
{
    fn size(&self) -> (usize, usize) {
        (self.len(), self[0].len())
    }
    fn at(&self, i: usize, j: usize) -> U {
        self[i][j]
    }
}

/// Defines a heap-allocated 2D array (slice of slices)
///
/// # Notes
///
/// * The number of columns is defined by the first row
/// * The next rows must have at least the same number of columns as the first row
impl<'a, U> AsMatrix<'a, U> for &'a [&'a [U]]
where
    U: 'a + Copy,
{
    fn size(&self) -> (usize, usize) {
        (self.len(), self[0].len())
    }
    fn at(&self, i: usize, j: usize) -> U {
        self[i][j]
    }
}

/// Defines a stack-allocated (fixed-size) 2D array
impl<'a, U, const M: usize, const N: usize> AsMatrix<'a, U> for [[U; N]; M]
where
    U: 'a + Copy,
{
    fn size(&self) -> (usize, usize) {
        (self.len(), self[0].len())
    }
    fn at(&self, i: usize, j: usize) -> U {
        self[i][j]
    }
}

/// Handles Matrix
impl<'a> AsMatrix<'a, f64> for Matrix {
    fn size(&self) -> (usize, usize) {
        self.dims()
    }
    fn at(&self, i: usize, j: usize) -> f64 {
        self.get(i, j)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::AsMatrix;
    use russell_lab::Matrix;
    use std::fmt::Write;

    fn matrix_str<'a, T, U>(array: &'a T) -> String
    where
        T: AsMatrix<'a, U>,
        U: 'a + std::fmt::Display,
    {
        let mut buf = String::new();
        let (m, n) = array.size();
        for i in 0..m {
            for j in 0..n {
                write!(&mut buf, "{},", array.at(i, j)).unwrap();
            }
            write!(&mut buf, "\n").unwrap();
        }
        buf
    }

    #[test]
    fn as_matrix_works() {
        // heap-allocated 2D array (vector of vectors)
        const IGNORED: f64 = 123.456;
        let a = vec![
            vec![1.0, 2.0],
            vec![3.0, 4.0, IGNORED, IGNORED, IGNORED],
            vec![5.0, 6.0],
        ];
        assert_eq!(
            matrix_str(&a),
            "1,2,\n\
             3,4,\n\
             5,6,\n"
        );

        // heap-allocated 2D array (aka slice of slices)
        let b: &[&[f64]] = &[&[10.0, 20.0], &[30.0, 40.0, IGNORED], &[50.0, 60.0, IGNORED, IGNORED]];
        assert_eq!(
            matrix_str(&b),
            "10,20,\n\
             30,40,\n\
             50,60,\n"
        );

        // stack-allocated (fixed-size) 2D array
        let c = [[100.0, 200.0], [300.0, 400.0], [500.0, 600.0]];
        assert_eq!(
            matrix_str(&c),
            "100,200,\n\
             300,400,\n\
             500,600,\n"
        );

        // matrix
        let d = Matrix::from(&[[0.1, 0.2], [0.3, 0.4], [0.5, 0.6]]);
        assert_eq!(
            matrix_str(&d),
            "0.1,0.2,\n\
             0.3,0.4,\n\
             0.5,0.6,\n"
        );
    }
}
