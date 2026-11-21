#[derive(Debug,Error)]
pub enum FileError {
    #[error("[failed] setting paths")]
    NullOfPath,
    #[error("[failed] generating key")]
    NotVaildKey,
    #[error("[failed] lock file")]
    UsedFile,
    #[error("[failed] no hit ever")]
    EndCondition,
}