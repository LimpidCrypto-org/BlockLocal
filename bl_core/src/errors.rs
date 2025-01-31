use k3d::Error as K3dError;

pub type Result<T> = core::result::Result<T, K3dError>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    K3dError(#[from] K3dError),
}
