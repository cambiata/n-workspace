pub mod f32_ext;
pub mod slice_ext;

#[cfg(test)]
mod tests {
    use crate::f32_ext::half::F32ExtHalf;
    use crate::f32_ext::round::F32ExtRound2;
    use crate::slice_ext::SliceExt;

    #[test]
    fn test_slice_ext() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(v.fourth(), Some(&4));
    }

    #[test]
    fn test_half() {
        let value: f32 = 3.14159;
        dbg!(value);
        dbg!(value.r2());
        dbg!(value.half()); // Should print 5.0
    }
}
