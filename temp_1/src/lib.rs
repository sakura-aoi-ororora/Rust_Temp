pub fn add_one(left: usize, right: usize) -> usize {
    println!("test");
    left + right
}

#[cfg(test)]
mod tests {
    use crate::add_one;

    #[test]
    fn test(){
        assert_eq!(2,add_one(1, 1))
    }
}
