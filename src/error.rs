#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("CSV error")]
    Csv(#[from] csv::Error),
    #[error("Invalid flagged record")]
    InvalidFlaggedRecord(csv::StringRecord),
    #[error("Invalid screen names record")]
    InvalidScreenNamesRecord(csv::StringRecord),
    #[error("Invalid suspensions record")]
    InvalidSuspensionsRecord(csv::StringRecord),
    #[error("memory.lol error")]
    MemoryLol(#[from] memory_lol::db::Error),
    #[error("Template error")]
    Tera(#[from] tera::Error),
    #[error("Log initialization error")]
    LogInitialization(#[from] log::SetLoggerError),
}
