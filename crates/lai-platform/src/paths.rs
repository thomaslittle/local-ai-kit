use lai_contracts::{DomainError, ErrorCode, Result};
use std::path::{Component, Path, PathBuf};

pub fn child_path(root: &Path, name: &str) -> Result<PathBuf> {
    if name.is_empty()
        || name.len() > 255
        || name.contains(['/', char::from(92), ':', char::from(0)])
        || !matches!(
            Path::new(name).components().collect::<Vec<_>>().as_slice(),
            [Component::Normal(_)]
        )
    {
        return Err(DomainError::new(
            ErrorCode::InvalidRequest,
            "Expected one safe path component",
        ));
    }
    Ok(root.join(name))
}
