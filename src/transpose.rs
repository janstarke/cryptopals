pub trait Transpose<T> {
    fn transpose(self) -> Self;
}

impl<T> Transpose<T> for Vec<Vec<T>> {
    /// transpose a matrix
    /// <https://stackoverflow.com/questions/64498617/how-to-transpose-a-vector-of-vectors-in-rust>
    ///
    /// ```rust
    /// use cryptopals::Transpose;
    /// let v = vec![vec![1,2,3,4], vec![5,6,7,8]];
    /// let v2 = v.clone().transpose().transpose();
    /// assert_eq!(v, v2);
    /// assert_eq!( vec![vec![1,2,3,4], vec![5,6,7,8]].transpose(),
    ///             vec![vec![1,5],vec![2,6],vec![3,7],vec![4,8]]);
    /// ```
    fn transpose(self) -> Self {
        assert!(!self.is_empty());
        let len = self[0].len();
        let mut iters: Vec<_> = self.into_iter().map(|n| n.into_iter()).collect();
        (0..len)
            .map(|_| {
                iters
                    .iter_mut()
                    .map(|n| n.next().unwrap())
                    .collect::<Vec<T>>()
            })
            .collect()
    }
}
