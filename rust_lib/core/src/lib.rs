pub fn hello(name: &str) -> String {
    format!("Hello, {}! This is from Rust.", name)
}

#[cfg(test)]
mod tests {
    use crate::hello;

    #[test]
    fn test_hello() {
        hello("world");
    }
}
