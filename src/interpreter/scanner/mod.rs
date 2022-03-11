pub type Tokens = Vec<String>;

pub fn scan(file: &String) -> Tokens {
    println!("Scanning: {}", file);

    let mut tokens = Tokens::new();

    for _ in 0..10 {
        tokens.push(String::from("foo"));
    }

    tokens
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_scan() {
        let file_name = String::from("foo.sc");
        for token in &super::scan(&file_name) {
            assert_eq!("foo", token);
        }
    }
}
