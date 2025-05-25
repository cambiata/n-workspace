pub mod slice_ext;

#[cfg(test)]
mod tests {
    use crate::slice_ext::SliceExt;

    #[test]
    fn test_slice_ext() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(v.fourth(), Some(&4));
    }
}
