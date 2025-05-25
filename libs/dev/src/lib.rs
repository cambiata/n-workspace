#[derive(Debug)]
pub enum Test {
    TestA(usize),
    TestB(usize),
    TestC(usize),
}

#[cfg(test)]
mod tests2 {
    use std::error::Error;

    use crate::Test;

    #[test]
    fn example() {
        let mut t = Test::TestA(1);

        match t {
            Test::TestA(x) => t = Test::TestA(222),
            _ => {}
        };

        dbg!(t);
    }

    fn return_error() -> Result<(), Box<dyn std::error::Error>> {
        Err("Error".into())
    }

    fn get_number(v: usize) -> Result<usize, Box<dyn std::error::Error>> {
        Ok(v * 2)
    }

    fn ret_err() -> Result<usize, Box<dyn std::error::Error>> {
        Err("Error".into())
    }

    #[test]
    fn ex2() -> Result<(), Box<dyn std::error::Error>> {
        let mut t = Test::TestA(1);

        let arr: Vec<usize> = vec![1, 2, 3];

        let sum = arr
            .iter()
            .map(|x| {
                // return_error()?;
                let n = get_number(*x)?;
                Ok(n)
            })
            .collect::<Result<Vec<usize>, Box<dyn std::error::Error>>>()?;

        println!("Sum: {:?}", sum);

        match t {
            Test::TestA(x) => t = Test::TestA(222),
            _ => {}
        };

        dbg!(t);

        let double = arr.iter().map(|x| Ok(x * 2)).collect::<Result<Vec<usize>, Box<dyn Error>>>()?;

        Ok(())
    }
}
