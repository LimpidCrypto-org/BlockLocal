use k3d::Error as K3dError;

pub type Result<T> = core::result::Result<T, K3dError>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("K3d error: {0}")]
    K3dError(#[from] K3dError),
}
