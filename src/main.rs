use anyhow::Context;
use rust_ecosystem_learning::MyError;
use std::fs;

fn main() -> Result<(), anyhow::Error> {
    println!("size of anyhow::Error: {}", size_of::<anyhow::Error>());
    println!("size of std::io::Error: {}", size_of::<std::io::Error>());
    println!(
        "size of std::num::ParseIntError: {}",
        size_of::<std::num::ParseIntError>()
    );
    println!(
        "size of serde_json::Error: {}",
        size_of::<serde_json::Error>()
    );
    println!("size of string: {}", size_of::<String>());
    println!("size of MyError: {}", size_of::<MyError>());

    let filename = "non_existent_file.txt";
    let _fd =
        fs::File::open(filename).with_context(|| format!("Can not find file: {}", filename))?;

    fail_with_error()?;
    Ok(())
}

fn fail_with_error() -> Result<(), MyError> {
    Err(MyError::Custom("This is a custom error".to_string()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
