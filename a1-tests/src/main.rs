fn main() {
    let hello = "hello";
    println!("{} world", hello);
}

fn truthy() -> bool {
    return true;
}

#[cfg(test)]
mod test {
    use super::truthy;

    #[test]
    fn test_something() {
        assert_eq!(truthy(), true);
    }
}
