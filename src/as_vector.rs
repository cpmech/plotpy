use russell_lab::Vector;

/// Defines a trait to handle Vector-like data
///
/// # Example
///
/// ```
/// use plotpy::AsVector;
/// use russell_lab::Vector;
///
/// fn sum<'a, T, U>(array: &'a T) -> f64
/// where
///     T: AsVector<'a, U>,
///     U: 'a + Into<f64>,
/// {
///     let mut res = 0.0;
///     let m = array.vec_size();
///     for i in 0..m {
///         res += array.vec_at(i).into();
///     }
///     res
/// }
///
/// // heap-allocated 1D array (vector)
/// let x = vec![1.0, 2.0, 3.0];
/// assert_eq!(sum(&x), 6.0);
///
/// // heap-allocated 1D array (slice)
/// let y: &[f64] = &[10.0, 20.0, 30.0];
/// assert_eq!(sum(&y), 60.0);
///
/// // stack-allocated (fixed-size) 2D array
/// let z = [100.0, 200.0, 300.0];
/// assert_eq!(sum(&z), 600.0);
///
/// // using Vector
/// let w = Vector::from(&[5.0, 5.0, 5.0]);
/// assert_eq!(sum(&w), 15.0);
/// ```
pub trait AsVector<'a, U: 'a> {
    /// Returns the size of the vector
    fn vec_size(&self) -> usize;

    /// Returns the value at index i
    fn vec_at(&self, i: usize) -> U;
}

/// Defines a heap-allocated 1D array (vector)
impl<'a, U: 'a> AsVector<'a, U> for Vec<U>
where
    U: 'a + Copy,
{
    fn vec_size(&self) -> usize {
        self.len()
    }
    fn vec_at(&self, i: usize) -> U {
        self[i]
    }
}

/// Defines a heap-allocated 1D array (slice)
impl<'a, U> AsVector<'a, U> for &'a [U]
where
    U: 'a + Copy,
{
    fn vec_size(&self) -> usize {
        self.len()
    }
    fn vec_at(&self, i: usize) -> U {
        self[i]
    }
}

/// Defines a stack-allocated (fixed-size) 1D array
impl<'a, U, const M: usize> AsVector<'a, U> for [U; M]
where
    U: 'a + Copy,
{
    fn vec_size(&self) -> usize {
        self.len()
    }
    fn vec_at(&self, i: usize) -> U {
        self[i]
    }
}

/// Handles Vector
impl<'a> AsVector<'a, f64> for Vector {
    fn vec_size(&self) -> usize {
        self.dim()
    }
    fn vec_at(&self, i: usize) -> f64 {
        self.get(i)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::AsVector;
    use russell_lab::Vector;
    use std::fmt::Write;

    fn vector_str<'a, T, U>(array: &'a T) -> String
    where
        T: AsVector<'a, U>,
        U: 'a + std::fmt::Display,
    {
        let mut buf = String::new();
        let m = array.vec_size();
        for i in 0..m {
            write!(&mut buf, "{},", array.vec_at(i)).unwrap();
        }
        write!(&mut buf, "\n").unwrap();
        buf
    }

    #[test]
    fn as_vector_works() {
        // heap-allocated 1D array (vector)
        let x = vec![1.0, 2.0, 3.0];
        assert_eq!(vector_str(&x), "1,2,3,\n");

        // heap-allocated 1D array (slice)
        let y: &[f64] = &[10.0, 20.0, 30.0];
        assert_eq!(vector_str(&y), "10,20,30,\n");

        // stack-allocated (fixed-size) 2D array
        let z = [100.0, 200.0, 300.0];
        assert_eq!(vector_str(&z), "100,200,300,\n");

        // vector
        let w = Vector::from(&[10.0, 10.0, 10.0]);
        assert_eq!(vector_str(&w), "10,10,10,\n");
    }
}
