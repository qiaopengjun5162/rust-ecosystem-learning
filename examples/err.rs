use anyhow::Context;
use std::fs;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),
    #[error("Serialize json error: {0}")]
    Serialize(#[from] serde_json::Error),
    // #[error("Error: {a}, {b:?}, {c:?}, {d:?}")]
    // BigError {
    //     a: String,
    //     b: Vec<String>,
    //     c: [u8; 64],
    //     d: u64,
    // },
    #[error("Error: {0:?}")]
    BigError(Box<BigError>),

    #[error("An error occurred: {0}")]
    Custom(String),
}

#[derive(Debug)]
pub struct BigError {
    pub a: String,
    pub b: Vec<String>,
    pub c: [u8; 64],
    pub d: u64,
}

/// Print the size of different types of errors and a string.
/// This function is intended to compare the size of different error types.
///
/// This function will fail with an error using `fail_with_error` after printing the sizes.
///
/// This function will intentionally try to open a non-existent file and handle the error
/// with `anyhow::Error`.
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
    let _fd = fs::File::open(filename).with_context(|| format!("Can not find file: {filename}"))?;

    fail_with_error()?;
    Ok(())
}

/// Simulates a failure by returning a custom error.
///
/// This function always returns an error of type `MyError::Custom` with a predefined error message.
/// It's primarily used for testing error handling and propagation in the application.
///
/// # Returns
///
/// Returns a `Result` where:
/// - `Err` contains a `MyError::Custom` with the message "This is a custom error".
/// - `Ok` is never returned as this function always fails.
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
