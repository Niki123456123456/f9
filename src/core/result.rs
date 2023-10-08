pub enum Error {
    NotFound,
    WrongType
}

pub type Result<T> = std::result::Result<T, Error>;

