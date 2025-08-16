pub mod email;
pub mod f32_ext;
pub mod slice_ext;
pub mod string_ext;

#[cfg(test)]
mod tests {
    use crate::f32_ext::half::F32ExtHalf;
    use crate::f32_ext::round::F32ExtRound2;
    use crate::slice_ext::SliceExt;
    use crate::string_ext;

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

    fn change_string(s: &mut String) {
        s.push_str(" - modified");
    }

    #[test]
    fn test_str() {
        let mut s: String = "  Hello, world!".to_string();
        string_ext::trim_string(&mut s);
        change_string(&mut s);
        dbg!(s);
    }

    #[test]
    fn test_error() -> Result<(), Box<dyn std::error::Error>> {
        fn testerror() -> Result<usize, String> {
            Ok(123)
        }

        let v = vec![1, 2, 3];
        v.iter()
            .map(|x| {
                let t = testerror()?;
                dbg!(x + t);
                Ok(x + t)
            })
            .collect::<Result<Vec<usize>, String>>()?;

        Ok(())
    }
}
