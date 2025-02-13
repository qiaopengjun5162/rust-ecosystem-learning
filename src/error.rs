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
