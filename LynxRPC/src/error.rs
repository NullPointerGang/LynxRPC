#[derive(Debug)]
pub enum Error {
    Serialization(String),
    Deserialization(String),
    AuthError,
    MethodNotFound,
    HandlerError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Serialization(msg) => write!(f, "Ошибка сериализации: {}", msg),
            Error::Deserialization(msg) => write!(f, "Ошибка десериализации: {}", msg),
            Error::AuthError => write!(f, "Ошибка аутентификации"),
            Error::MethodNotFound => write!(f, "Метод не найден"),
            Error::HandlerError(msg) => write!(f, "Ошибка обработчика: {}", msg),
        }
    }
}

impl std::error::Error for Error {} 