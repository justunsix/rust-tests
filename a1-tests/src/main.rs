fn main() {
    let hello = "hello";
    println!("{} world", hello);
    truthy();
}

fn truthy() -> bool {
    true
}

#[cfg(test)]
mod test {
    use super::truthy;

    #[test]
    fn test_something() {
        assert!(truthy());
    }
}
