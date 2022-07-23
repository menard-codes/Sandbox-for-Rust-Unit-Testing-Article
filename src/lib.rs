pub fn adder(x: i32, y: i32) -> i32 {
    x + y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds() {
        assert_eq!(adder(4, 5), 9);
    }
}
