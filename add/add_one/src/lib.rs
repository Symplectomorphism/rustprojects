use rand;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn my_it_works() {
        assert_eq!(3, add_one(2));
    }
}

pub fn add_one(x: i32) -> i32 {
    x + 1
}

