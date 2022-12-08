pub fn example_function() -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_function_test() {
        assert_eq!(example_function(), 0);
    }
}
