use std::{fs::{self, OpenOptions}, io::{Read, Seek, SeekFrom}, path::Path};
use lai_contracts::{DomainError, ErrorCode, Result};
use crate::{DirectoryListing, EntryKind, LocalFilesystem, NativeFilesystem, ReadChunk};

fn io_error(error: std::io::Error) -> DomainError {
    let code = match error.kind() {
        std::io::ErrorKind::NotFound => ErrorCode::NotFound,
        std::io::ErrorKind::PermissionDenied => ErrorCode::LocalAccessDenied,
        _ => ErrorCode::ProviderUnavailable,
    };
    DomainError::new(code, "Local filesystem operation could not be completed")
}
fn invalid(message: &str) -> DomainError { DomainError::new(ErrorCode::InvalidRequest, message) }
impl LocalFilesystem for NativeFilesystem {
    fn probe(&self, path: &Path) -> Result<EntryKind> {
        match fs::symlink_metadata(path) {
            Ok(m) => Ok(if m.file_type().is_symlink() { EntryKind::Symlink }
                else if m.is_file() { EntryKind::File }
                else if m.is_dir() { EntryKind::Directory } else { EntryKind::Other }),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(EntryKind::Missing),
            Err(e) => Err(io_error(e)),
        }
    }
    fn children(&self, path: &Path, max_entries: usize) -> Result<DirectoryListing> {
        if max_entries == 0 || max_entries > 4096 { return Err(invalid("Directory budget must be 1..4096")); }
        if self.probe(path)? != EntryKind::Directory { return Err(invalid("Expected a regular directory")); }
        let mut entries = Vec::new();
        let mut truncated = false;
        for entry in fs::read_dir(path).map_err(io_error)? {
            if entries.len() == max_entries { truncated = true; break; }
            entries.push(entry.map_err(io_error)?.path());
        }
        entries.sort();
        Ok(DirectoryListing { entries, truncated })
    }
    fn read_chunk(&self, path: &Path, offset: u64, max_bytes: usize) -> Result<ReadChunk> {
        if max_bytes == 0 || max_bytes > 16*1024*1024 { return Err(invalid("Read budget must be 1..16777216 bytes")); }
        if self.probe(path)? != EntryKind::File { return Err(invalid("Only regular files may be read")); }
        let mut options = OpenOptions::new();
        options.read(true);
        #[cfg(unix)] {
            use std::os::unix::fs::OpenOptionsExt;
            options.custom_flags(libc::O_NONBLOCK | libc::O_NOFOLLOW);
        }
        #[cfg(windows)] {
            use std::os::windows::fs::OpenOptionsExt;
            options.custom_flags(0x00200000); // FILE_FLAG_OPEN_REPARSE_POINT
        }
        let mut file = options.open(path).map_err(io_error)?;
        let metadata = file.metadata().map_err(io_error)?;
        if !metadata.is_file() || metadata.file_type().is_symlink() { return Err(invalid("Only regular files may be read")); }
        let total_size = metadata.len();
        if offset > total_size { return Err(invalid("Read offset exceeds current file size")); }
        file.seek(SeekFrom::Start(offset)).map_err(io_error)?;
        let mut bytes = Vec::with_capacity(max_bytes.min((total_size-offset).min(usize::MAX as u64) as usize));
        file.take(max_bytes as u64).read_to_end(&mut bytes).map_err(io_error)?;
        let next_offset = offset + bytes.len() as u64;
        Ok(ReadChunk { bytes, next_offset, total_size, truncated: next_offset < total_size })
    }
}
