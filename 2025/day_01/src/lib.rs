pub type MyResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub enum Turn {
    Left(i32),  // take away Lefts
    Right(i32), // add Rights
}
