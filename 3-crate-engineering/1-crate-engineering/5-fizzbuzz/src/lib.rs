use std::ops::Add;

/// Very naive implementation of FizzBuzz
pub fn fizz_buzz(i: u32) -> String {
    if i % 3 == 0 {
        if i % 5 == 0 {
            "FizzBuzz".to_owned()
        } else {
            "Fizz".to_owned()
        }
    } else if i % 5 == 0 {
        "Buzz".to_owned()
    } else {
        format!("{i}")
    }
}

pub fn fast_fizz_buzz(i: u32) -> String {
    if i % 3 == 0 {
        "Fizz".to_owned()
    } else if i % 5 == 0 {
        "Buzz".to_owned()
    } else if i % 15 == 0 {
        "FizzBuzz".to_owned()
    } else {
        i.to_string()
    }
}

// TODO Write a unit test, using the contents of `fizzbuzz.out` file
// to compare.
// You can use the `include_str!()` macro to include file
// contents as `&str` in your artifact.
#[cfg(test)]
mod tests {
    use crate::fizz_buzz;

    #[test]
    fn test_fizz_buzz() {
        let expected_output = include_str!("../fizzbuzz.out").lines();

        for (index, line) in expected_output.enumerate() {
            assert_eq!(line, fizz_buzz(index as u32 + 1))
        }
    }
}
