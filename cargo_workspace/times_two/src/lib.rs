pub fn times_two(num: usize) -> usize {
    num * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = times_two(9);
        assert_eq!(result, 18);
    }
}
