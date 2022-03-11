mod scanner;

pub type FilePath = String;

pub fn run(file: &FilePath) -> Result<(), &'static str> {
    if !is_valid_script(file) {
        return Err("unknown file type");
    }

    print!("Running: {}", file);

    for token in scanner::scan(file) {
        println!("{}", token);
    }

    Ok(())
}

fn is_valid_script(file: &FilePath) -> bool {
    file.contains(".sc")
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_is_valid_script() {
        let file_name = String::from("foo.sc");

        assert_eq!(true, super::is_valid_script(&file_name));
    }

    #[test]
    #[should_panic(expected = "does not end in '.sc'")]
    fn test_is_invalid_script() {
        let file = String::from("foo.rs");
        if !super::is_valid_script(&file) {
            panic!("File: {} does not end in '.sc'", file);
        }
    }
}
