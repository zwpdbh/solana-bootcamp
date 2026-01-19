use solana_sdk::program_error::ProgramError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    SolanaClientError(String),
    SolanaProgramError(String),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<solana_client::client_error::ClientError> for Error {
    fn from(value: solana_client::client_error::ClientError) -> Self {
        Self::SolanaClientError(value.to_string())
    }
}
impl From<ProgramError> for Error {
    fn from(value: ProgramError) -> Self {
        Self::SolanaProgramError(value.to_string())
    }
}
