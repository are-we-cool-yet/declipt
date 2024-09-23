use std::sync::mpsc::RecvError;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error loading library: {0}")]
    LibLoading(#[from] libloading::Error),
    #[error("error hooking function: {0:?}")]
    MhStatus(minhook::MH_STATUS),
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Error reading PDB: {0}")]
    PdbError(#[from] pdb::Error),
    #[error("Error receiving message from thread: {0}")]
    RecvError(#[from] RecvError),
}

impl From<minhook::MH_STATUS> for Error {
    fn from(value: minhook::MH_STATUS) -> Self {
        Self::MhStatus(value)
    }
}
