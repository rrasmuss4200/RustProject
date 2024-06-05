pub fn add_one(num: usize) -> usize {
    num + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(1);
        assert_eq!(result, 2);
    }
}
